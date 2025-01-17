mod components;
mod models;

use crate::components::app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}