# Development Environment

Overengineered is a very opinionated full stack environment. To make the installation of all the 
dependencies and tools need to work within the environment, we created a Docker image that has 
all of these prepackaged for convenience.

If you are not familiar with Docker, it is a quick and secure way to build applications 
consistently and just work. Docker is similar to a virtual operating system were it only shares 
the underlying kernel. It provides a declarative interface to compose images and can be published 
onto repositories.

## Launching the environment

Once Docker is installed and the image is built. You can start the image and jump into a bash 
session with the container with this command.

```rust,ignore
docker run -it oedev /bin/bash
```

In this case the i flag is indicating that we want to open an interactive SSH session to the 
container. The t flag allows to run commands interactively.

After the command is run the Docker container should boot up quickly. As soon as it is booted a 
ssh session should kick in and connect you to the container.

## Setting up

Once your successfully SSH'd in, we'll need to set up a few things before we get too deep in. 
Overengineered uses Github for our code repository and we will be setting up several repositories
throughout this book.

The Github CLI is already shipped with the Docker image. Let's start by logging in. This process 
is simple, we just need to run one command. But first let's grab a personal access token. 
Instructions can be found at the 
[github docs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token).

At the time of writing, You can find it under the developer settings.

* Upper-right corner of any page, click profile photo and settings.
* In the left bar click Developer settings.
* In the left sidebar click, Personal access tokens.
* Click Generate new token.
* Fill out the details and click generate.
* Make sure you copy the token.

Now from the SSH session we established with the Docker container we setup earlier run the command
to login replacing <your_token> with the token we copied in the previous step.

```rust,ignore
gh auth login --with-token <your_token>
```

## Stopping the container

There is no persistent data in this development environment. As soon as the container is no longer
running all modified files will be gone forever. There are a couple ways to deal with this, you 
can use source control or a remote home directory. (*Currently not supported*)
