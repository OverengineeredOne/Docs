# Component Library

Now that we know how to create components, creating high quality complex user interfaces from 
scratch is a big challenge. This is where a Component Library can help.

At this time there is not a feature complete component library for Yew. Of the handful of existing
projects [patternfly-yew](https://github.com/ctron/patternfly-yew) is the only one recently 
maintained.

## Patternfly

Patternfly is an open source design system to create consistent and usable applications
experiences. Pattternfly provides clear standards and guidelines to build efficently. It was 
designed as a React component library, however there is a community effort recreate the components
on an as needed basis.

Given the current state it would be best to start to get to know the library though the React 
documentation.

## Components

If we pull up the documentation for the [Alert](https://www.patternfly.org/v4/components/alert)
component we're greeted to all of the examples. The different examples show the design of the 
component and how it can be used. 

The `</> JS` button on each example allows you to view what the code looks like. The bottom of the
page contains the props, which tells you about all of the properties to control the behavior of
the component.

### Guidelines

The [Design Guidelines](https://www.patternfly.org/v4/components/alert/design-guidelines) exists 
for every component and provides details what the expected behaviors and how the user interacts 
with it.

### patternfly-rs

Let's look at the equivalent [Alert](https://ctron.github.io/components/alert) component 
example. patternfly-rs uses the same styles sheets as the React library so the components should 
look identical. Like the upstream Patternfly documentation, it should contain the code snippit.

At the current stage there is not a lot of componenets and the example page does not meet parity
but should be a good place to start.

There are also more components completed than the ones on the example. A better resource for an up
to date list of components avalible the 
[Rust Docs](https://docs.rs/patternfly-yew/latest/patternfly_yew/index.html) will be in sync with 
the upstream repository.

## Using patternfly-rs

patternfly-rs also includes a [quickstart template](https://github.com/ctron/patternfly-yew) that
ships with the patternfly-rs documentaiton page. Normally from the github page you click the use
`Use this template` button, clone the repo and install everything with npm.

But the Overengineered CLI that comes pre-installed in the development environment. We can do all 
of this with one step with the following command, replacing `<name>` with the name of the project 
name. This clones the repo using the name as the name of the repo.

```rust,ignore
oecli pwa --new <name>
```

Just like our previous examples, run `trunk serve` and head to your browser to check it out. 

