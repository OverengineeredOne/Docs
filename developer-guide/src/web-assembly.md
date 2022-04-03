# WebAssembly

At the time of writing WebAssembly is sill new and is already gaining steam as the next big player 
in web development. It has a massive amount of potential to evolve and change the way applications 
are bulilt and shipped.

## What is WebAssembly?

I'll never do as good as a job as 
[Lin Clark's A Cartoon Intro to WebAssembly](https://www.youtube.com/watch?v=HktWin_LPf4), but
I'll try.

WebAssembly solves the problem to allow applications built to be used in the browser, written
in languages other than Javascript. This is a problem because Javascript is much slower than 
compiled languages and allows us to do some low level programming to solve problems that were not
previously possible. Since the code is compiled to the WebAssembly target and converted to a 
binary format this allows applications to run, without installation and provide native performance.

## Background

Javascript was created back in 1995 and was the only way to interact with the DOM and manipulate 
HTML. Overall JS performance is fairly poor, until the introduction of just in time compilers, 
which drastically increased performance and interactivity into what it is today.

## The future of WebAssembly

Like I mentioned WebAssembly is still new, for example today there is a lot of JS interopt. As 
data is passed around it's serialzed / deserialized and causes some major performance issues that 
in some cases doesn't make it practical to use. In the future it will have more funcationality 
such as direct DOM and Websocket access.

## WASI and beyond

WASI or the WebAssembly System Interface is exciting since it brings WebAssembly to the backend. 
This is exciting for a few reasons, every WASI module runs in it's secure environment, interfaces 
allow for easy interopt between languages, and finally cold start times are nearly instantanous.
Drastically quicker than Docker.
