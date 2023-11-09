use leptos::{Callback, component, create_rw_signal, IntoView, SignalGet, view};
use crate::components::TodoSignal;
use crate::models::Todo;

#[component]
pub fn TodoListItem(todo:Todo, #[prop(into)] on_edit: Callback<Todo>, #[prop(into)] on_delete: Callback<Todo>) -> impl IntoView {

    let todo_item: TodoSignal = create_rw_signal(todo);

    let on_edit_event = move |_| {on_edit(todo_item.get())};
    let on_delete_event = move |_| {on_delete(todo_item.get())};

    view! {
          <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-row">

                <div class="basis-11/12">
                    <p class="text-lg text-gray-900">
                        {todo_item.get().title}
                    </p>

                    <textarea class="text-left text-gray-500 w-full" rows=3>
                        {todo_item.get().description}
                    </textarea>
                </div>

                <div class="basis-1/12 flex items-center justify-center">
                   <div class="flex flex-row-reverse space-x-4 space-x-reverse">
                        <button on:click=on_edit_event
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2">
                             <i class="fa-solid fa-edit"></i>
                        </button>
                        <button on:click=on_delete_event
                            class="text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2">
                             <i class="fa-solid fa-minus"></i>
                        </button>
                   </div>
                </div>
          </div>
    }
}