use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PostsProps {}

#[function_component]
pub fn Posts(props: &PostsProps) -> Html {
    let PostsProps {} = props;
    html! {
        <div>{"Posts"}</div>
    }
}