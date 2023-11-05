use leptos::RwSignal;
use crate::models::Todo;

pub mod app;
mod add_todo_modal;

pub type TodoListSignal = RwSignal<Vec<Todo>>;
pub type ShowAddTodoModalSignal = RwSignal<bool>;