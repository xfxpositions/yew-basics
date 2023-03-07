use chrono::prelude::*;
use yew::{prelude::*, html::onclick};
use uuid::Uuid;
use web_sys::HtmlInputElement;

#[derive(Clone)]
struct Todo{
    id:Uuid,
    title:String,
    created_at: DateTime<Local>,
    completed:bool,
}
impl Todo {
    pub fn new(title:String) -> Todo{
        return Todo { id: Uuid::new_v4(), title: title, created_at: Local::now(), completed: false }
    }
}
#[derive(PartialEq, Properties)]
pub struct TodoListProps {}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let TodoListProps {} = props; //props
    //creating todolist
    let mut todos:Vec<Todo> = vec![];
    let todo = Todo::new("Deneme".to_string());
    todos.push(todo.clone());    
    //creating html list for rendering todolist
    let mut todos_html: Vec<Html> = vec![html!{}];
        for todo in todos.iter(){
            let value = html!{
                <>
                <hr />
                    <ul key={&*todo.id.to_string()}>
                        <li>{"toto id = "}{&todo.id.to_string()}</li>
                        <li>{"toto title = "}{&todo.title}</li>
                        <li>{"toto created_at = "}{&todo.created_at}</li>
                        <li>{"toto completed = "}{&todo.completed}</li>
                    </ul>
                    <button>{"complete the todo"}</button>
                </>
            };
            todos_html.push(value);
        };
        //interactivity
        let input_node_ref = use_node_ref();

        let onchange = {
            let input_node_ref = input_node_ref.clone();
            
            Callback::from(move |_| {
                if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                    let value = input.value();
                    println!("{}",value);
                     web_sys::console::log_1(&value.into());
                }
            })
        };
        let addtodo = {
            let input_node_ref = input_node_ref.clone();

            Callback::from(move |_| {
                let mut todos = todos.clone();
                if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                    let value = input.value();
                    web_sys::console::log_1(&value.clone().into());
                    //creating new todo
                    let title:String = value.to_string();
                    let todo = Todo::new(title);
                    todos.push(todo);
                    web_sys::console::log_1(&todos.len().clone().into());

                }
            })
        };
    html! {
        <div>
            <h1>{"TodoList"}</h1>
            <div>
                {todos_html}
            </div>
            <hr />
            <div>
                <h3>{"Create new todo"}</h3>
                <input type="text" placeholder="title" />
                <button onclick={addtodo}>{"Submit"}</button>
                <label for="my-input">
                { "My input:" }
                <input ref={input_node_ref}
                    {onchange}
                    id="my-input"
                    type="text"
                />
            </label>
            </div>
        </div>
    }
}