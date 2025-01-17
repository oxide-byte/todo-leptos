use leptos::prelude::*;
use crate::components::TodoSignal;
use crate::models::Todo;

#[component]
pub fn TodoListItem<E, D>(todo: Todo, on_edit: E, on_delete: D,
) -> impl IntoView
where
    D: Fn(Todo) + 'static,
    E: Fn(Todo) + 'static,
{
    let todo_item: TodoSignal = RwSignal::new(todo);

    let on_edit = move |_| {
        on_edit(todo_item.get());
    };

    let on_delete = move |_| {
        on_delete(todo_item.get());
    };

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
                        <button on:click=on_edit
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2">
                             <i class="fa-solid fa-edit"></i>
                        </button>
                        <button on:click=on_delete
                            class="text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2">
                             <i class="fa-solid fa-minus"></i>
                        </button>
                   </div>
                </div>
          </div>
    }
}