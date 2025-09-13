#![allow(unused)] // Yew components don't account for usage.

use std::slice::Iter;

use palette::{FromColor, Hsv, Srgb};
use thiserror::Error;
use gloo::storage::{LocalStorage, Storage};
use gloo::storage::errors::StorageError;


#[derive(Error, Debug)]
pub enum ColorMemoryError {
    #[error("The browser '{0:#}' global object is missing.")]
    MissingObject(String),

    #[error("Couldn't read or write to local storage.")]
    LocalStorage(#[from] StorageError)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorMemory {
    ls_key: String,

    memory: Vec<Srgb<u8>>,
    max_size: usize,
}

impl ColorMemory {
    fn fill(memory: &mut Vec<Srgb<u8>>, max_size: usize) {
        const HSV_SATURATION: f32 = 0.9;
        const HSV_VALUE: f32 = 0.9;

        if max_size <= memory.len() {
            return;
        }

        let hue_step = 360.0 / max_size as f32;

        for slot in memory.len()..max_size {
            let hue = slot as f32 * hue_step;
            let hsv = Hsv::new(hue, HSV_SATURATION, HSV_VALUE);
            let rgb = Srgb::<f32>::from_color(hsv);
            memory.push(rgb.into_format())
        }
    }

    fn write(&self) -> Result<(), ColorMemoryError> {
        LocalStorage::set(&self.ls_key, &self.memory)?;

        Ok(())
    }

    pub fn from_ls(ls_key: String, max_size: usize) -> Result<Self, ColorMemoryError> {
        let mut memory = LocalStorage::get::<Vec<Srgb<u8>>>(&ls_key)
            .or_else(|error| match error {
                StorageError::KeyNotFound(_) => Ok(Vec::with_capacity(max_size)),
                error => Err(error)
            })?;

        memory.truncate(max_size);
        Self::fill(&mut memory, max_size);

        Ok(Self {
            ls_key,

            memory,
            max_size
        })
    }

    #[inline]
    pub fn new(ls_key: String, max_size: usize) -> Self {
        let mut memory = Vec::with_capacity(max_size);
        Self::fill(&mut memory, max_size);

        Self {
            ls_key,

            memory,
            max_size
        }
    }

    pub fn push(&mut self, color: Srgb<u8>) -> Result<(), ColorMemoryError> {
        if !self.memory.contains(&color) {
            self.memory.insert(0, color);
        }

        if self.memory.len() > self.max_size {
            self.memory.pop();
        }

        self.write()
    }

    pub fn get(&self, index: usize) -> Option<&Srgb<u8>> {
        self.memory.get(index)
    }

    pub fn iter(&'_ self) -> Iter<'_, Srgb<u8>> {
        self.memory.iter()
    }
}
