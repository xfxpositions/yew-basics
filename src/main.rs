use yew::prelude::*;
mod nav;
use nav::Navbar;
mod todos;
use todos::TodoList;
#[function_component(App)]
fn app() -> Html {
    let zibidi:Html = html!{
        <h1>{"deneme"}</h1>
    };
    let a:u32 = 1;
 

    //creating a quick list
    let items = vec!["deneme","deneme2","listdeneme3"];
    let mut items_html: Vec<Html> = vec![];
    for item in items.iter(){
        let value = html!{
            <li>{item}</li>
        };
        items_html.push(value);
    };
    let a = use_state(|| vec![1,2,3]);
    let counter = use_state(|| 0);
    let list = use_state(|| vec!["Item 1".to_owned(), "Item 2".to_owned()]);
    let onclick = {
        let a = a.clone();
        let counter = counter.clone();
        Callback::from(move |_| {
            let mut x = a.to_vec().clone();
            x.push(4);
            a.set(x);
        })
    };
    html! {
        <>
            <Navbar />
            <div>
                {zibidi}
            </div>
            <ul>
               {items_html}
            </ul>
            <button onclick={onclick}>{"tikla"}</button>
            <div>{for a.iter()}</div>
            <hr />
            <TodoList />
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
