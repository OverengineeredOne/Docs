# WebAssembly

At the time of writing WebAssembly is sill new and is already gaining steam as the next big player 
in web development.

## What is WebAssembly?

I'll never do as good as a job as 
[Lin Clark's A Cartoon Intro to WebAssembly](https://www.youtube.com/watch?v=HktWin_LPf4), but
I'll take a crack at it.

WebAssembly solves the problem to allow applications built to be used in the browser to be written
in languages other than Javascript. In our case, we will be using Rust and compiling to the low 
level assembly. This allows applications to run, without installation and provide native 
performance.

## Background

Javascript was created back in 1995 and was the only way to interact with the Dom and manipulate 
HTML. Overall JS performance is fairly poor, until the introduction of just in time compilers, 
which drastically increased performance into what it is today.

## The future of WebAssembly

Like I mentioned WebAssembly is still new, for example today there is a lot of JS interopt. As 
data is passed around it's serialzed / deserialized. Today this is a big performance bottleneck. 
In the future as proposals get written into the WebAssembly standard, such as direct DOM access.

## WASI and beyond

WASI is exciting since it brings WebAssembly to the backend. This is exciting for a few reasons,
every WASI module runs in it's secure environment, interfaces allow for easy interopt between
languages that compile to the same target, and finally cold start times are nearly instantanous.
Drastically quicker than Docker, which is already quick.
