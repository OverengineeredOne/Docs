use patternfly_yew::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct NewTodoProps {
    pub onsubmit: Callback<String>,
}

#[function_component(NewTodoBar)]
pub fn new_todo(props: &NewTodoProps) -> Html {
    let text: UseStateHandle<String> = use_state(|| "".to_string());
    let on_text_change = {
        let text = text.clone();
        Callback::from(move |value: String| text.set(value.clone()))
    };
    let cb = props.onsubmit.clone();
    let onsubmit = {
        let text = text.clone();
        Callback::from(move |_| {
            cb.emit((*text).clone());
            text.set("".to_string());
        })
    };

    html! {
        <Split gutter=true>
            <SplitItem fill=true>
                <TextInput
                    placeholder={"New TODO Title"}
                    onchange={on_text_change}
                    r#type={"text"}
                    value={(*text).clone()}
                />
            </SplitItem>
            <SplitItem>
                <Button
                    disabled=false
                    label={"Create"}
                    onclick={onsubmit}
                    variant={Variant::Primary}
                />
            </SplitItem>
        </Split>
    }
}
