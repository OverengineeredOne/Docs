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

First update your existing packages by running this in your terminal session.

    sudo apt update

Next install the prerequisites

    sudo apt install apt-transport-https ca-certificates curl software-properties-common

Add the GPG key for the Docker repository

    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -

Add the docker repositories 

    sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu focal stable"

Install docker

    sudo apt install docker-ce

NOTE: These steps likely can wrapped in a bash script and completed in a single step. Host on
https://github.com/overengineered/

### Set up Overengineered

## Mac OS X

TODO: Create instructions for mac

## Raspbian (Raspberry Pi)

TODO: Create instructions for Raspberry Pi, similar to Ubuntu but a single install script.

## Windows

TODO: Create instructions for Windows, likely will set up a Ubuntu VM in hypervisor. Contribution 
ideas welcome.
