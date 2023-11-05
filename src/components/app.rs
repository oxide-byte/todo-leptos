use leptos::*;
use crate::components::{ShowAddTodoModalSignal, TodoListSignal};
use crate::models::Todo;
use crate::components::add_todo_modal::AddTodoModal;

#[component]
pub fn App() -> impl IntoView {
    let todos:TodoListSignal = create_rw_signal(Vec::new());
    let show_add_modal: ShowAddTodoModalSignal = create_rw_signal(false);

    let on_add_task_event = move |todo: Todo| {
        todos.set({
            let mut old = todos.get();
            old.push(todo);
            old
        });

        show_add_modal.set(false);

    };

    let on_cancel_add_event = move |_| {
        show_add_modal.set(false);
    };

    let add_todo = move |_| {
        show_add_modal.set(true);
    };

    let delete_todo = move |id : String| {
        todos.set({
            let mut old = todos.get();
            old.retain(|x| x.id != id);
            old
        })
    };

    view! {
        <div class="container mx-auto m-5 p-6">
        <h1 class="mb-4 text-4xl font-extrabold text-center text-gray-600">TODO LIST</h1>

        <div class="pb-5">
            Create a Todo:
            <button on:click=add_todo class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mx-2">
                <i class="fa-solid fa-plus"></i>
            </button>
        </div>

        <Show
            when=move || { !todos.get().is_empty() }
            fallback=|| view! { <h2>Currently no Todos defined</h2> }>
            <h2>Start working</h2>

            <For
                each=todos
                key=|item| (item.id.clone(), item.description.clone())
                let:child
            >

            <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-row">

                <div class="basis-11/12">
                    <p class="text-lg text-gray-900">
                        {child.title}
                    </p>

                    <textarea class="text-left text-gray-500 w-full" rows=3>
                        {child.description}
                    </textarea>
                </div>

                <div class="basis-1/12 flex items-center justify-center">
                <button on:click=move |_| delete_todo(child.id.clone())
                    class="text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2">
                    <i class="fa-solid fa-minus"></i>
                </button>
                </div>
            </div>
            </For>
        </Show>
        </div>

        <Show when = move || { show_add_modal.get() }>
            <AddTodoModal
                on_add=on_add_task_event
                on_cancel=on_cancel_add_event>
            </AddTodoModal>
        </Show>
    }
}