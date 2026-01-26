mod app;
mod models;
mod components;
mod services {
    pub mod api;
}

fn main() {
    yew::Renderer::<app::App>::new().render();
}
