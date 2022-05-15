use crate::todolistitem::TodoListItem;
use patternfly_yew::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct TodoListProps {
    /// Callback used when a Todo is closed or marked as completed. Uses index from the vec.
    pub onclose: Callback<usize>,
    pub todos: Vec<String>,
}

/// A PatternFly Card that renders all of the Todo's.
#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    html! {
        <Card>
            <Stack>
                {props.todos.iter().enumerate().map(|(idx, todo)| {
                    html! {
                        <StackItem>
                            <TodoListItem
                                onclose={props.onclose.clone()}
                                todo={todo.to_owned()}
                                {idx}
                            />
                        </StackItem>
                    }
                }).collect::<Html>()}
            </Stack>
        </Card>
    }
}
