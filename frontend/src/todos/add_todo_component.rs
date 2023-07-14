use leptos::{
    component, create_node_ref, ev::SubmitEvent, html::Input, view, IntoView, NodeRef, Scope,
    SignalUpdate, WriteSignal,
};

#[component]
pub fn AddTodoComponent(cx: Scope, set_todos: WriteSignal<Vec<String>>) -> impl IntoView {
    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let new_value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_todos.update(|value| value.push(new_value));
        input_element()
            .expect("<input> to exist").set_value("");
    };

    view! { cx,
        <form on:submit=on_submit>
            <label>"Todo item" <input node_ref=input_element/></label>
            <button>"Add todo item"</button>
        sdlgkj
        </form>
    }
}
