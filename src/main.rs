use yew::prelude::*;
mod nav;
use nav::Navbar;
#[function_component(App)]
fn app() -> Html {
    let zibidi:Html = html!{
        <h1>{"deneme"}</h1>
    };
    let a:u32 = 1;
    let hello = "Hello".to_string();
    let world = "World ".to_string();

    //creating a quick list
    let items = vec!["deneme","deneme2","listdeneme3"];
    let mut items_html: Vec<Html> = vec![html!{}];
    for item in items.iter(){
        let value = html!{
            <li>{item}</li>
        };
        items_html.push(value);
    };
    html! {
        <>
            <Navbar />
            <h1>{hello + " " + &world.as_str()}{a}</h1>
            <div>
                {zibidi}
            </div>
            <ul>
               {items_html}
            </ul>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
