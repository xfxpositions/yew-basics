use std::any::Any;

use yew::prelude::*;
use crate::todotype::Todo;
use web_sys::HtmlInputElement;
#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    //pub todolist: Vec<Todo>
}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let TodoListProps {} = props;

    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let zibidi_handle= use_state(String::default);
    let zibidi = (*zibidi_handle).clone();
    //creating todolist
    let todolist_state:UseStateHandle<Vec<Todo>> = use_state(|| vec![]); 
    let mut todolist: Vec<Todo> = (*todolist_state).clone(); 

    //initilazing todolist
    let first_todo = Todo::new("Deneme1".to_string());
    todolist.push(first_todo);

    //update input value
    fn input_bind(state:&UseStateHandle<String>)-> Callback<InputEvent>{
        return{
            let input_value_handle = state.clone();
            Callback::from(move |e:InputEvent| {
                let input = e.target_dyn_into::<HtmlInputElement>();
                if let Some(input) = input {
                    input_value_handle.set(input.value());
                }
            })
        }
    }
    let update_input_value = input_bind(&input_value_handle);
    let update_zibidi_value = input_bind(&zibidi_handle);
    //rendering todolist
    let mut a_html:Vec<Html> = vec![];
    for item in todolist.iter(){
        let value = 
        html!{
            <ul>
                <li>{"Todo title: "}{&item.title}</li>
                <li>{"Completed?: "}{&item.completed}</li>
                <button>{"Complete the todo"}</button>
                <hr />
            </ul>
        };
        a_html.push(value);
    }
    html! {
        <div>
            <h2>{"Todolist"}</h2>
            <hr />
            <input type="text" placeholder="enter todo title" oninput={update_input_value} value={input_value.clone()}/>
            <p>{input_value.clone()}</p>

            <input type="text" placeholder="enter todo title" oninput={update_zibidi_value} value={zibidi.clone()}/>
            <p>{zibidi.clone()}</p>

            <button>{"Add Todo"}</button>
            {a_html}
        </div>
    }
}