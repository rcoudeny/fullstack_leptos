mod todos;
use leptos::{mount_to_body, view};
use todos::todo_page::TodoPage;

fn main() {
    mount_to_body(|cx| view! { cx,  <TodoPage/> })
}
