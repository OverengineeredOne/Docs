# Component Library

Now that we know how to create components we would like to build something that's more ascetically
pleasing. But creating high quality complex user interfaces from scratch is a big challenge. This
is where a Component Library can help. They establish the design language and as a developer we
get to spend less time thinking about this.

At this time there is not a feature complete component library for Yew. And out of the handful of
existing projects, [patternfly-yew](https://github.com/ctron/patternfly-yew) is the only one
recently maintained.

## Patternfly

Patternfly is an open source design system to create consistent and usable applications
experiences. Patternfly provides clear standards and guidelines to build efficiently. It was
designed as a React component library, however there is a community effort recreate the components
on an as needed basis for Yew.

Given the current state of the Rust project, it would be best to start to get to know the library
though the React documentation.

Getting started with Patternfly is pretty straightforward, you'll get immediate value from two
primary sections. Components and Layouts.

## Components

If we pull up the documentation for the [Alert](https://www.patternfly.org/v4/components/alert)
component we're greeted to all of the examples. The different examples show the implementation of
the component and different ways to use it.

You can take a look at the code under each example with the `</> JS` button. The very bottom of
the page contains the props, which tells you about all of the properties to control the behavior
of the component.

### Guidelines

The [Design Guidelines](https://www.patternfly.org/v4/components/alert/design-guidelines) exists
for most component and provides details what the expected behaviors and how the user interacts
with it. Many will also contain best practices and how they should be used with the other
Patternfly components.

### patternfly-rs

Let's look at the equivalent [Rust Doc](https://ctron.github.io/patternfly-yew-quickstart/) and
find the `Alert` component. patternfly-rs uses the same CSS as the React library so the
components should look identical. Like the upstream Patternfly documentation, it should contain
the code snippet, but currently nothing else. Unlike the React documentation, the code is just
simply positioned below.

At the current stage there is not a lot of components and the example page does not meet parity
with React, but should be a good place to start.

To see all of the components that have been completed, a better resource is avalible on the
[Rust Docs](https://docs.rs/patternfly-yew/latest/patternfly_yew/index.html).

## Using patternfly-rs

patternfly-rs also includes a [quickstart template](https://github.com/ctron/patternfly-yew) that
ships with the entire patternfly-rs documentation page. Normally from the github page you click
the use `Use this template` button, clone the repo and install everything with npm.

But the Overengineered CLI can take care of this for us. We can do all of this with a single
command. You don't have to do this now but you can use the following command, replacing `<name>`
with the name of the project name. This will clone the repository using the name as the name of
the repo.

```rust,ignore
oecli pwa --new <name>
```

Just like our previous examples, run `trunk serve` and head to your browser to check it out.

**Note**: `pwa` here stands for Progressive Web App. Progressive web apps are apps that load a
skeleton and hydrate components as they are ready. At this current state, Overengineered does not
offer server side rendering, or this progressive hydration. These are goals further down the
roadmap.
