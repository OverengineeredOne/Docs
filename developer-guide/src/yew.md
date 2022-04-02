# Yew
Yew is a modern Rust framework for creating multi-threaded front-end web applications with 
WebAssembly.

Like React Yew is a component based framework which makes it easy to create interactive UIs. 
Performance is acheived by minimizing DOM calls and offloading proccing into background threads.

## Intoduction 

Let's take a look at how to build an example Yew app. If you are using the Overengineered 
development environment we are good to go and can skip the next step. If not there are a couple of
prerequisites you'll need to set up.

### Prerequisites.

You'll need to make sure you have the following tools up to date.

* Rust
* trunk
* Compile target wasm32-unknown-unknown

You can check out earlier chapters for instructions on installing rust.

After Rust is installed, you can use Cargo to install `trunk` by running:

```rust,ignore
cargo install trunk
```

### Setting up the Yew Project

For this project, we will need to expose the port 8080 so we can load the page in the browser 
later. Make sure you add the extra argument when launching the development environment.

```rust,ignore
docker run -it -p 8080:8080 oedev /bin/bash
```

Once connected let's go ahead and create our project.

```rust,ignore
cargo new example-yew-project
cd example-yew-project
```

And to veryify everything works perform an intital build with Cargo and you should see the 
expected Hello, World! example.

## Building a web page

The first thing we want to make sure we do is add the WASM build target by running: 

```rust,ignore
rustup target add wasm32-unknown-unknown
```
Now we can making some updates to our Cargo.toml file to add the Yew dependency.


```rust,ignore
[package]
name = "example-yew-project"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = "0.19"

```

Now in your main.rs let's create a simple component that renders the message `Hello, World!`

```rust,ignore
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
	<h1>{ "Hello, World!" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
```

The `function_component()` macro creates a functional component with the name being the identifier 
name passed in as an argument. Function components are a simplivied version of a normal component.
They render what every they return. In our case we return the JSX like markup within the html 
macro.

A couple things to note about the html macro.
* Expressions must be wrapped in curly braces `{ }`
* There must only be one root node. You can wrap them in a fragment `<></>`

Before we can see what this looks like in the browser let's make sure we add `index.html` to 
the root of the project.

```rust,ignore
<!DOCTYPE html>
<html lang="en">
    <head></head>
    <body>

    </body>
</html>
```

If you notice we don't add anything to the children of the body element. This is because Yew will 
insert this data for us.

### Viewing the page

During development we need to be able to view the changes as we develop. This is where trunk comes 
in to help us. Run the following command to build and serve the application.

```rust,ignore
trunk serve 
```

From here you can navigate to your browser and try reaching the webpage with the default IP 
Address. `http://172.17.0.1:8080`

Congradulations, you've built your first Yew app! 

### Hooks

Hooks are Yew functions that allow you to tie into the lifecycle or state of a component. We will 
take a look at [use_state](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).

`use_state` allows us to create and set a default value to be used within the component. The 
`UseStateHandle` it returns has a method that allows us to change the state value. This will in 
turn update the component.

Let's update our app method to test it out.

```rust,ignore
use yet::{Callback, function_component, html, use_state};

#[function_component(UseState)]
fn state() -> Html {
    let counter = use_state(|| 0);
    let onClick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    html! {
	<div>
	    <button {onclick}>{ "Add one" }</button>
	    <p>{ "Total: " }{ *counter }</p>
	</div>
    }
}

```

Go ahead and `trunk serve`. You can see as you click the button, it adds one to the total.

### Further Reading

I'll highly reccomend following up with the Yew documentation.

* [Components](https://yew.rs/docs/concepts/components/introduction)
* [HTML](https://yew.rs/docs/concepts/html/introduction)
* [Functional Components](https://yew.rs/docs/concepts/function-components/introduction)
* [Agents](https://yew.rs/docs/concepts/agents/introduction)
* [Agents](https://yew.rs/docs/concepts/agents)
* [Contexts](https://yew.rs/docs/concepts/context)
* [Router](https://yew.rs/docs/concepts/router)
