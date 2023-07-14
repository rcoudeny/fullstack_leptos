use leptos::{component, view, ForProps, IntoView, ReadSignal, Scope, WriteSignal};

#[component]
pub fn TodoListComponent(
    cx: Scope,
    todos: ReadSignal<Vec<String>>,
    set_todos: WriteSignal<Vec<String>>,
) -> impl IntoView {
    let todo_items = ForProps::builder()
        .each(todos)
        .key(|todo| todo.to_owned())
        .view(move |cx, todo: String| {
            view! { cx, <div>{todo}</div> }
        });
    view! { cx, <div>{todo_items.build()}</div> }
}
