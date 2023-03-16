use yew::prelude::*;
mod input_example;
use input_example::InputExample;
mod todolist;
use todolist::TodoList;
mod todotype;
use todotype::Todo;
#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="p-3">
            <h1 class="text-3xl font-bold">{ "Hello World" }</h1>
           // {a_html}
            <div>
            <TodoList />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
