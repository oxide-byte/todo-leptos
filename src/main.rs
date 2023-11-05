mod components;
mod models;

use crate::components::app::App;
use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}