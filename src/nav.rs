use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;


#[derive(PartialEq, Properties)]
pub struct NavbarProps {}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    let NavbarProps {} = props;
    html! {
        <nav class="flex gap-3 items-center">
            <h1 class="text-xl font-bold mr-8">{"Yew basics."}</h1>
            <div class="flex gap-3">
                <Link<Route> classes="underline" to={Route::Home}>{ "Home" }</Link<Route>>
                <Link<Route> classes="underline" to={Route::Todolist}>{ "Todolist" }</Link<Route>>
                <Link<Route> classes="underline" to={Route::Posts}>{ "Posts" }</Link<Route>>
                <a class="underline" href="https://github.com/xfxpositions/yew-basics/">{"Source Code"}</a>
            </div>
        </nav>
    }
}