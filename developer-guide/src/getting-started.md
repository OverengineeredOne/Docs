# Getting Started

Let's get started! We'll start by installing all the prerequisites then dive into the fundamental 
pieces of Overengineered.

To ensure rapid development we want to minimize the setup times for your development environment 
and tools to follow along with the book as much as possible.

We accomplish this by utilizing docker and in the future will provide a web based environment. 
We use Ubuntu as the base image and comes with all the tools required to be installed to follow 
along with the book.

## Debian / Ubuntu


### Set up Docker

You can find install instructions for ubuntu at 
[docs.docker.com](https://docs.docker.com/engine/install/ubuntu/). To make tihs process even 
easier We've created a convenience script to get you set up with docker quickly. Go ahead and copy 
paste this into your terminal.

```rust,ignore
wget -O - https://raw.githubusercontent.com/OverengineeredOne/easy-docker-setup/main/debian.sh | bash
```

If you find any issues with the script, please, fill out an issue on 
[github](https://github.com/OverengineeredOne/easy-docker-setup)

### Set up Overengineered

Everything's shiny captain, a development environment doesn't exist yet. If you read a head you 
can build the current state from the source.

### Building from source

Since the entire development environment is a container image, we can download the source from 
github and build it our selves.

For this we'll need to make sure we have git installed.

```rust,ignore
apt install git-all
```

Next clone the repository and navigate into the new directory.

```rust,ignore
git clone https://github.com/OverengineeredOne/oedev.git && cd oedev
```

Finally just build the image!

```rust,ignore
docker build -t oedev .
```


## Mac OS X

TODO: Create instructions for mac

### Set up Docker

### Set up Overengineered

## Raspbian (Raspberry Pi)

TODO: Create instructions for Raspberry Pi, similar to Ubuntu but a single install script.

### Set up Docker

### Set up Overengineered

## Windows

TODO: Create instructions for Windows, likely will set up a Ubuntu VM in hypervisor. Contribution 
ideas welcome.

### Set up Docker

### Set up Overengineered
