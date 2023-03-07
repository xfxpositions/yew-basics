use yew::prelude::*;


#[derive(PartialEq, Properties)]
pub struct NavbarProps {}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    let NavbarProps {} = props;
    html! {
        <nav>
            <h1>{"A simple navbar in yew."}</h1>
        </nav>
    }
}