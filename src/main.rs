use yew::prelude::*;
mod input_example;
mod todolist;
use todolist::TodoList;
mod todotype;
use yew_router::prelude::*;
mod nav;
use nav::Navbar;
mod pages;
use crate::pages::posts::Posts;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/todolist")]
    Todolist,
    #[at("/posts")]
    Posts,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Todolist => html!(<TodoList />),
        Route::Posts => html!(<Posts />)
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <div class="p-3">
                    <Navbar />
                    <Switch<Route> render={switch} />
                </div>
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
