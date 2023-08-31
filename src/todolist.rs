use gloo_console::log;
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

    //creating todolist
    let todolist_state:UseStateHandle<Vec<Todo>> = use_state(|| vec![]); 

    //initilazing todolist
    

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
    //add todo
    let add_todo = {
        let todolist_state = todolist_state.clone();
        let input_value = input_value.clone();
        Callback::from(move |_| {
            let mut value = todolist_state.to_vec();
            value.push(Todo::new(input_value.to_string()));
            todolist_state.set(value);
            let format_str = format!("{:?}",todolist_state.to_vec());
            log!(format_str); 
        })
    };
    
    #[derive(PartialEq, Properties)]
    pub struct ButtonProps {
        id:uuid::Uuid,
        todolist_handle:UseStateHandle<Vec<Todo>>
    }
    
    #[function_component]
    pub fn Button(props: &ButtonProps) -> Html {
        let ButtonProps {id,todolist_handle} = props;
        let button_text = use_state(|| "Complete");
        //complete todo
        let complete_todo = {
            let todolist_handle = props.todolist_handle.clone();
            let id = props.id;
            let button_text = button_text.clone();
            Callback::from(move |_e:MouseEvent| {
                let mut value = todolist_handle.to_vec();
                for mut todo in value.iter_mut(){
                    if todo.id == id{
                        log!("BAM");
                        if todo.completed{
                            button_text.set("Complete");
                        }else{
                            button_text.set("Discomplete");
                        }
                        todo.completed = !todo.completed;
                    }
                }
                todolist_handle.set(value);
                
            })
        };
        html! {
            <button class="shadow-lg border-2 border-white bg-green-800 text-white p-4 rounded-md transition-all duration-300 ease-in-out hover:translate-y-1" onclick={complete_todo}>{*button_text}</button>
        }
    }
    //rendering todolist
    let mut a_html:Vec<Html> = vec![];
    let a = todolist_state.to_vec();
    for (index,item) in a.iter().enumerate(){
        let value = 
        html!{
            <ul key={index}>
                <li>{"Todo title: "}{&item.title}</li>
                <li>{"Completed?: "}{&item.completed}</li>
                <li>{"Id: "}{&item.id}</li>
                <Button id={item.id} todolist_handle={todolist_state.clone()}></Button>
                <hr />
            </ul>
        };
        a_html.push(value);
    }
    html! {
        <div>
            <h2>{"Todolist"}</h2>
            <hr class="mb-2"/>
            <input class="outline-none max-w-4 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg transition-all duration-250 ease-in-out focus:ring-blue-500 focus:border-blue-500 block p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" type="text" placeholder="enter todo title" oninput={update_input_value} value={input_value.clone()}/>
            <p>{input_value.clone()}</p>

          

            <button class="mt-3 focus:shadow-sm focus:shadow-gray-200 shadow-white border-white bg-green-800 text-[white] p-4 rounded-md border-[1px] transition-all duration-300 ease-in-out hover:translate-y-1"  onclick={add_todo}>{"Add Todo"}</button>
           
            {a_html}
        </div>
    }
}