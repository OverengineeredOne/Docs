# Introduction

Overengineered is here to help you develop fast, efficient, and safe decentralized, apps. We 
accomplish this by providing a easy to install *cloud at home*, where decentralized services can 
be installed as easy as it is to install an app on your phone. 

A GraphQL *data abstraction service* and decentralized *registries* are provided to help
distribute data schemas. Progressive Web Apps are able to request the data service to download and 
install the required schema.

This should enable developers to move quickly to implement new products and features. We also
provide all the glue needed to develop within this environment in an easy and cohesive manor.

## Rust

In the spirit of trying to create a safe and secure platform that treats data ownership as a first
class feature; Rust feels like a natural fit. 

The current state of many of the dependencies are not considered stable, our platform will not be 
considered stable or backwards compatible. These guarantees will likely evolve over time, but for 
now expect rapid breaking changes.

## Who is this book for?

This book is for any programmer or engineer that wants to learn about modern web programming
and how this landscape is changing with the next generation of the internet. 

## How to use this book.

When writing this book we have no expectations on the audience. In the earlier chapiters We try to
structure the text such that you can easily skim sections based on your knowledge and expertise.
Some sections may require spending time following up on the additional resources. 

Later in the book we'll be taking a deep dive into some of the services and libraries we've 
built to make development easier.

## Source Code

The source files from this book can be found on [GitHub](https://github.com/overengineered/) and 
will be found prefixed with `[Developer Book][Chapter][Section] - Title`

## Stack

Briefly let's cover the stack that we will be working with. Later we'll have a gentle introduction 
to many of them and others may not require direct interaction with, it may be important to know.

You can follow up on these at anytime to learn more.

### Frontend

* **Yew** - Yew is a rust crate similar to [react](https://reactjs.org/). Working with Yew feels 
much like working with react.
* **GraphQL** - [GraphQL](https://graphql.org/) was chosen as the primary server / client 
interaction to minimize network traffic. With the ever growing number of devices we want to keep 
payloads as small as possible.  GraphQL provides a very nice abstraction for developers and has 
existing client libraries for many languages.

### Backend

#### Webserver

* **Rocket** - [Rocket](https://rocket.rs/) is the primary recommended way to create a web server. 
Rocket is a rust crate that focuses on a great developer experience with an easy to use API. We 
think this fits perfectly with what we are trying to accomplish.
* **GraphQL** - Like previously mentioned for various reasons we use GraphQL. GraphQL is very easy 
to integrate with Rocket.

### DevOps

* **Docker** - We primarily use [Docker](https://docker.com/) for the build tools, but primarily 
use [Containard](https://containerd.io/) for the container service.
* **Kubernetes** - [Kubernetes](https://kubernetes.io/) is a container orchestration service to 
manage containers across all devices within your network. We specifically use 
[k3s](https://k3s.io/) since it supports the arm architecture and can easily be ran on Raspberry 
Pi to enable low cost hardware and low power consumption.
* **Template Cluster K3s** - An [opinionated](https://github.com/k8s-at-home/template-cluster-k3s)
template for deploying a single k3s cluster.
	* **Ansible** - [Ansible](https://ansible.com/) is an automation platform for managing 
	automation across devices.
	* **Terraform** - [Terraform](https://terraform.io/) provides infrastructure as code. We 
	use it for managing the Cloudflare domain.
	* **Flux** - [Flux](https://fluxcd.io/docs/) is a tool for keeping the Kubernetes cluster 
	in sync with the code repository.
	* **SOPS** - [SOPS](https://github.com/mozilla/sops) is a tool for managing private data 
	such as passwords and private information within the configuration files that will be 
	checked int source control.
	* **Cloudflare** - [Cloudflare](https://cloudflare.com/) manages domain and updates the 
	DNS.
