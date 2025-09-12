use serde_json::{to_string as to_json_string, Error as JsonError, from_str as json_from_str};
use thiserror::Error;
use web_sys::{window, Storage};
use rgb::Rgb;

pub type Color = Rgb<u8>;

#[derive(Error, Debug)]
pub enum ColorMemoryError {
    #[error("The browser '{0:#}' global object is missing.")]
    MissingObject(String),

    #[error("A JSON error occurred: {0:#}")]
    JsonError(#[from] JsonError),

    #[error("Couldn't read or write to local storage.")]
    LocalStorage
}

pub struct ColorMemory {
    ls_key: String,

    memory: Vec<Color>,
    max_size: usize,
}

impl ColorMemory {
    fn local_storage() -> Result<Storage, ColorMemoryError> {
        window()
            .and_then(|window| window.local_storage().ok().flatten())
            .ok_or_else(|| ColorMemoryError::MissingObject("window.localStorage".into()))
    }

    fn write(&self) -> Result<(), ColorMemoryError> {
        Self::local_storage()?
            .set_item(&self.ls_key, &*to_json_string(&self.memory)?)
            .or(Err(ColorMemoryError::LocalStorage))?;

        Ok(())
    }

    pub fn from_ls(ls_key: String, max_size: usize) -> Result<Self, ColorMemoryError> {
        let raw_memory = Self::local_storage()?
            .get_item(&ls_key)
            .or(Err(ColorMemoryError::LocalStorage))?;

        let memory = raw_memory
            .map(|array| json_from_str(&array))
            .transpose()?
            .unwrap_or_else(|| Vec::<Color>::with_capacity(max_size));

        Ok(Self {
            ls_key,

            memory,
            max_size
        })
    }

    #[inline]
    pub fn new(ls_key: String, max_size: usize) -> Self {
        Self {
            ls_key,

            memory: Vec::with_capacity(max_size),
            max_size
        }
    }

    pub fn push(&mut self, color: Color) -> Result<(), ColorMemoryError> {
        self.memory.push(color);

        if self.memory.len() > self.max_size {
            self.memory.pop();
        }

        self.write()
    }

    pub fn get(&self, index: usize) -> Option<&Color> {
        self.memory.get(index)
    }
}
