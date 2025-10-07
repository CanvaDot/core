# Contributing to Canvadot

Thanks for considering contributing to Canvadot.

## Setting up the project

This project uses docker, one of the advantages docker has is that it barely requires
any environment modifications, neither requires you to install tools or other rust targets.

You may want to have docker (https://docs.docker.com/engine/install/) along docker-compose
installed (https://docs.docker.com/compose/install).

To run the docker container and test the project tho, you need `just` which is a recipe runner,
you may download it with `cargo-binstall`, refer to their official guide @ https://just.systems/man/en/ .

The small bit of setup this project has can be done by running `just setup` after `just`
is installed.

## Running the dev server

To run the dev server you simply run `just dev` and it will start the front-end and back-end
together.

When everything is started you will see the logs attached to your terminal and prefixed
by whether it's front-end or back-end.

The front-end should be accessible on `http://127.0.0.1:8080` and the back-end
on `http://127.0.0.1:8080/api`. This pattern is also used in production, so there is
no need to separate domains or routes trough environment variables for now.

The dev server reloads automatically after there has been some change.

## Adding stuff

Before you add any code to the project, be sure this code is solving an issue,
if you want to add an idea of your own, create an issue and wait for it to be
accepted, otherwise any pull request you open will be automatically closed
as not planned.

Before making an issue, make sure another issue isn't already open with the same
idea. You can contribute to that issue too depending on it's state.

If you have everything running correctly you can add more code to the project,
for this it's recommended to read `TESTING.md` as there is information on
what each folder does and what is excluded or included from tests.

## Making a pull request

Your pull requests for now can be directly merged to main, but this is subject to
change, when the amount of pull requests requires it a roll-out strategy will
be used instead and a possible development branch.

The pull requests are as straight forward as is, they run CI automatically you can read
`TESTING.md` for more information, and code will be reviewed by a trusted reviewer.

After all of this is done, your pull request may be merged.
