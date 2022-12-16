use patternfly_yew::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct TodoListItemProps {
    /// Index of the todo list item
    pub idx: usize,
    /// Callback used when a Todo is closed or marked as completed. Uses index from the vec.
    pub onclose: Callback<usize>,
    /// Title of the Todo item.
    pub todo: String,
}

/// A component that renders a single todo. This component consists of two sub components, one to
/// render the title of the Todo and a button on the far right hand to close the Todo out.
#[function_component(TodoListItem)]
pub fn todo_list_item(props: &TodoListItemProps) -> Html {
    let TodoListItemProps { idx, onclose, todo } = props;
    let close_cb = onclose.clone();
    let close = {
        let idx = idx.clone();
        Callback::from(move |_| close_cb.emit(idx.clone()))
    };
    html! {
        <Split gutter=true>
            <SplitItem fill=true>
                <Text>
                    {todo.clone()}
                </Text>
            </SplitItem>
            <SplitItem>
                <Button
                    icon={Icon::MinusCircleIcon}
                    onclick={close}
                    variant={Variant::Plain}
                />
            </SplitItem>
        </Split>
    }
}
