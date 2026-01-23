use yew::{function_component, html, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            <h1>{"hello world"}</h1>
        </main>
    }
}

// Run with `trunk serve`
fn main() {
    yew::Renderer::<App>::new().render();
}
