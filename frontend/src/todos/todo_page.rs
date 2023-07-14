use leptos::{component, create_signal, view, IntoView, ReadSignal, Scope, WriteSignal};

use crate::todos::{add_todo_component::AddTodoComponent, todo_list_component::TodoListComponent};

#[component]
pub fn TodoPage(cx: Scope) -> impl IntoView {
    let (todos, set_todos): (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>) =
        create_signal(cx, vec![]);
    view! {cx,
        <AddTodoComponent set_todos=set_todos/>
        <TodoListComponent todos=todos set_todos=set_todos />
    }
}
