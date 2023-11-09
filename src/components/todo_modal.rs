use leptos::*;
use leptos::ev::MouseEvent;
use leptos::html::{Input, Textarea};
use uuid::Uuid;
use crate::components::EditTodoSignal;
use crate::models::Todo;

#[component]
pub fn TodoModal(#[prop(into)] on_add: Callback<Todo>, #[prop(into)] on_cancel: Callback<MouseEvent>, todo:Option<Todo>) -> impl IntoView {

    let todo_item: EditTodoSignal = create_rw_signal(todo);

    let input_title: NodeRef<Input> = create_node_ref();
    let input_description: NodeRef<Textarea> = create_node_ref();

    let on_add_event = move |_| {

        let title = input_title().expect("<input> to exist").value();
        let description = input_description().expect("<textarea> to exist").value();
        let id = if todo_item.get().is_some() {todo_item.get().unwrap().id} else {Uuid::new_v4().to_string()};
        let created = if todo_item.get().is_some() {todo_item.get().unwrap().created} else {instant::Instant::now()};

        let new_item = Todo{
            id,
            title,
            description,
            created
        };

        on_add(new_item);
    };

view! {

<div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900 bg-opacity-60">

    <div
      class="block rounded-lg bg-white w-2/5 p-4 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)] z-70">

        <h5 class="mb-5 text-xl font-medium leading-tight text-neutral-800">
            Create new Todo
        </h5>

        <form>
            <div class="mb-5">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="title">
                    Title
                </label>
                <input
                    node_ref=input_title
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    id="title"
                    type="text"
                    value={move ||
                                if todo_item.get().is_some()
                                    {todo_item.get().unwrap().title}
                                else
                                    {String::new()}
                            }
                    placeholder="Title"/>
            </div>

            <div class="mb-5">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="description">
                    Description
                </label>
                <textarea
                    node_ref=input_description
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    rows="3"
                    id="description"
                    type="text"
                    placeholder="Description">{
                    move ||
                        if todo_item.get().is_some()
                            {todo_item.get().unwrap().description}
                        else
                            {String::new()}
                    }</textarea>
            </div>

            <div class="flex flex-row-reverse space-x-4 space-x-reverse">
                <button type="button"
                    class="bg-blue-700 hover:bg-blue-800 px-5 py-3 text-white rounded-lg"
                    on:click=on_add_event>
                    {move ||
                        if todo_item.get().is_some()
                            {String::from("EDIT")}
                        else
                            {String::from("ADD")}
                    }
                </button>
                <button type="cancel"
                    class="bg-gray-300 hover:bg-gray-400 px-5 py-3 text-white rounded-lg"
                    on:click=on_cancel>
                    Cancel
                </button>
            </div>
        </form>
    </div>
</div>
}}