# Chapter 2.3 Todo App

This example contains a documented simple Todo App which follows the Chapter
2.3 Todo App.

## Running

This assumes you are using the Overengineered Development Environment, if you
are not see below.

### Run developer setup

From within the docker container, run the following command which will re-build
with every change to the code:

    trunk serve --address 0.0.0.0

From your machine head to the browser to: http://172.17.0.2

Note: If you have multiple docker containers running on the host machine, you
may not be able to access with that local address. See the chapter on
PatternFly for more details.

## Run instructions for not using The development Environment

### Pre-requisites

- Rust

      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

- Trunk

      cargo install trunk

- NodeJS `npm`

### Initialize

Fetch the PatternFly dependencies:

    npm install

### Run local developer setup

Start a local development server, which re-builds every time you make changes
to the code:

    trunk serve

Direct your web browser to: http://localhost:8080

### Perform a release build

To build the Rust components and package up the page, run:

    trunk build

The release is in the `dist/` folder.
