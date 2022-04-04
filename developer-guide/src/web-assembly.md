# WebAssembly

At the time of writing WebAssembly is sill new and is already gaining steam as the next big player 
in web development. It has a massive amount of potential to evolve and change the way applications 
are bulilt and shipped.

## What is WebAssembly?

I'll never do as good as a job as 
[Lin Clark's A Cartoon Intro to WebAssembly](https://www.youtube.com/watch?v=HktWin_LPf4), but
I'll try.

WebAssembly solves the problem to allow applications to be built for the browser, written in
languages other than Javascript. Javascript is much slower and there are a lot of problems that 
can not be solved with it today. WebAssembly is a low level assembly like lanague that is 
converted into a binary format this allows applications to run, without installation and provide
native performance. It also works on Windows, Mac, Linux, iOS, Android, and many others.

## The future of WebAssembly

WebAssembly is still new, for example today there is a lot of JS interopt. As data is passed
around it's serialzed / deserialized and causes some major performance issues that in some cases
doesn't make it practical to use. In the future it will have more funcationality
such as direct DOM and Websocket access.

## WASI

WASI or the WebAssembly System Interface is exciting since it brings WebAssembly to the backend. 
This is exciting for a few reasons, every WASI module runs in it's secure environment, interfaces 
allow for easy interopt between languages, and finally cold start times are nearly instantanous.
Drastically quicker than Docker.
