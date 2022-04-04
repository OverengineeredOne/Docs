# Todo App

Now let's apply what we have covered so far and build something useful. We want something that
can be viewed on desktop mobile, and be feature rich.

Let's create a TODO app. As a user I should be able to create a todo and assign it to myself or
another person. TODO's are automatically generated for daily tasks, and some are triggered from an
external integration, such as Alexa. And we will create some automation to automatically cross off
our TODOs. We'll add additional features like notifications, change history, and some analytical
pages or dashboards.

## Create the project

We want to clone the patternfly-rs template. If you recall we can easily do this with the `pwa`
subcommand of `oecli`. Again make sure you are logged into github
`gh auth login --with-token <token_file>`.

```rust,ignore
oecli pwa --new todo-client
```

Now you can navigate into the new directory and try serving the page with trunk. `trunk serve`
and accessing with your browser as before.

### Template

The default template comes with all the code for the example page. We don't really care about it.
Inside the `./src` folder, we can delete everything but `main.rs` and `app.rs`. If we do this, we
can also get rid of all of the structs for Yew Routing. The only one we should care about is the
struts `Model`

```rust,ignore
use patternfly_yew::*;
use yew::prelude::*;

pub struct Model {}

impl yew::Component for Model {
	type Message = ();
	type Properties = ();
	fn create(_: &Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, _: &Context<Self>) -> Html {
		<>

		</>
	}
}

```

You may also notice this is not a function_component, you can go ahead and convert that on your
own.

## Design

Now that we have a blank page we will start simple. We want to have some text box for entering in
a todo and a button to submit. We'll also need to see a list of todos and some way to cross them
off. We'll stick with these features for this chapter.

Let's break down some of the components we'll be using.

### Add a todo

For the new TODO entry, a simple design would be to just place a text box and a button next to
each other.

```txt,ignore
+---------------------+ +--------+
|    Text Box         | | Submit |
+---------------------+ +--------+
```

We care about the text in the box and when the button is submitted we want to "submit" the data
that was entered into the text box.

What Patternfly Components would we need and how would we lay them out? Well we can find the
Button in the patternfly-rs documentation but nothing that really fits the text box. The upstream
Patternfly React documentation does have the
[Text Input](https://www.patternfly.org/v4/components/text-input). But ah, it does exist in
the
[patternfly-rs docs](https://docs.rs/patternfly-yew/latest/patternfly_yew/struct.TextInput.html)

Ok, now how would we place them next to each other? I'll let you take look at the layouts to find
out what works best: [patternfly-rs](https://ctron.github.io/)

Given this, let's see what our component should start to look like.

```rust,ignore
#[function_component(NewTodoBar)]
fn new_todo() -> Html {
	html! {
		<Split gutter=true>
			<SplitItem>
				<TextInput placeholder={ "New TODO" } />
			</SplitItem>
			<SplitItem>
				<Button
					disabled={false}
					label={ "Create" }
					variant={Variant::Primary}
				/>
			</SplitItem>
		</Split>
	}
}
```

You can add this directly in the Model component and serve it to test it out.

```rust,ignore
impl yew::Component for Model {
...
	fn view(&self, _:&Context<Self>) -> Html {
		html! {
			<>
				<NewTodoBar />
			</>
		}
	}
```

Looks much better than the counter example earlier.

... Breakdown component ...
