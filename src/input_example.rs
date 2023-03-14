use yew::prelude::*;
use gloo_console::log;
use web_sys::HtmlInputElement;


#[derive(PartialEq, Properties)]
pub struct InputExampleProps {}

#[function_component]
pub fn InputExample(props: &InputExampleProps) -> Html {
    let InputExampleProps {} = props;
    let counter = use_state(|| 0);
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();
    let list = vec!["pol pot".to_string()];
    let list_handle = use_state(|| list.clone());   
    let oninput = {
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |e:InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };
    let mut list_html:Vec<Html> = vec![];
    for item in list_handle.iter(){
        let value = html!{
            <li>{item}</li>
        };
        list_html.push(value);
    };
    let onclick = {
        let counter = counter.clone();
        let list_handle = list_handle.clone();
        let input_value = input_value.clone();
        Callback::from(move |_| {
            counter.set(*counter+1);
            let mut a = list_handle.to_vec();
            a.push(input_value.to_string());
            list_handle.set(a);
            log!("Clicked!");      
        })
    };
    html! {
        <div>
                <input {oninput} value={input_value.clone()}/>
                <p> {input_value.clone()} </p>
                {
                    list_html
                }
                <button onclick={onclick}>{"Denemeememe"}</button>
                <p> {*counter} </p>
        </div>
    }
}