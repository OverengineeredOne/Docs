# Getting Started

Let's get started! We'll start by installing all the prerequisites then dive into the fundamental 
pieces of Overengineered.

To ensure rapid development we want to minimize the setup times for your development environment 
and tools. We accomplish this by utilizing docker and in the future will provide a web based 
environment. We will go into more detail in the following chapters, but for now know, Our image 
has Ubuntu as the base image. Also comes with all the tools preinstalled to follow along with the
book.


## Installing Docker

### Ubuntu and Debian based OS including Raspberry Pi

You can find install instructions for ubuntu at 
[docs.docker.com](https://docs.docker.com/engine/install/ubuntu/). To make tihs process even 
easier We've created a convenience script to get you set up with docker quickly. Go ahead and copy 
paste this into your terminal.

```rust,ignore
wget -O - https://raw.githubusercontent.com/OverengineeredOne/easy-docker-setup/main/debian.sh | bash
```

If you find any issues with the script, please, fill out an issue on 
[github](https://github.com/OverengineeredOne/easy-docker-setup)

#### Set up Overengineered

Everything's shiny captain, a development environment doesn't exist yet. If you read a head you 
can build the current state from the source.

### Mac Os X

### Windows


## Building from source

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

Finally just build the image! This part may take some time.

```rust,ignore
docker build -t oedev .
```

