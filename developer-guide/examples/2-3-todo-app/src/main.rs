#![recursion_limit = "1024"]

use patternfly_yew::*;
use yew::prelude::*;
mod newtodo;
mod todolist;
mod todolistitem;
use crate::newtodo::NewTodoBar;
use crate::todolist::TodoList;

use log::Level;
use wasm_bindgen::prelude::*;

pub enum Msg {
    Submit(String),
    Close(usize),
}

pub struct TodoPage {
    todos: Vec<String>,
}

impl yew::Component for TodoPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { todos: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit(value) => {
                self.todos.push(value);
            }
            Msg::Close(idx) => {
                self.todos.remove(idx);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Page>
                <PageSection>
                    <NewTodoBar
                        onsubmit={ctx.link().callback(|value: String| Msg::Submit(value))}
                    />
                </PageSection>
                <PageSection>
                    <TodoList
                        onclose={ctx.link().callback(|idx| Msg::Close(idx))}
                        todos={self.todos.clone()}
                    />
                </PageSection>
            </Page>
        }
    }
}

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(Level::Debug));
    yew::start_app::<TodoPage>();
    Ok(())
}
