use leptos::*;
use crate::components::{EditTodoSignal, ShowTodoModalSignal, TodoListSignal};
use crate::models::Todo;
use crate::components::todo_modal::TodoModal;
use crate::components::todo_list_item::TodoListItem;

#[component]
pub fn App() -> impl IntoView {

    let todos:TodoListSignal = create_rw_signal(Vec::new());
    let show_modal: ShowTodoModalSignal = create_rw_signal(false);
    let edit_todo_item: EditTodoSignal = create_rw_signal(None);

    let on_add_todo_event = move |todo: Todo| {
        todos.update(|old|  {
            old.retain(|x| x.id != todo.id);
            old.push(todo);
            old.sort_by(|a, b| a.created.cmp(&b.created));
        });
        show_modal.set(false);
    };

    let on_cancel_add_event = move |_| {
        show_modal.set(false);
    };

    let on_show_modal_add_event = move |_| {
        edit_todo_item.set(None);
        show_modal.set(true);
    };

    let on_delete_todo_event = move |todo : Todo| {
        todos.update(|old| {
            old.retain(|x| x.id != todo.id);
        })
    };

    let on_edit_todo_event = move |todo : Todo| {
        edit_todo_item.set(Some(todo));
        show_modal.set(true);
    };

    view! {
        <div class="container mx-auto m-5 p-6">
        <h1 class="mb-4 text-4xl font-extrabold text-center text-gray-600">TODO LIST</h1>

        <div class="pb-5">
            Create a Todo:
            <button on:click=on_show_modal_add_event
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mx-2">
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
            <TodoListItem todo=child on_edit=on_edit_todo_event on_delete=on_delete_todo_event/>
            </For>
        </Show>
        </div>

        <Show when = move || show_modal.get()>
            <TodoModal
                on_add=on_add_todo_event
                on_cancel=on_cancel_add_event
                todo=edit_todo_item.get()>
            </TodoModal>
        </Show>
    }
}