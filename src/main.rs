mod app;
mod models;
mod components;
mod services {
    pub mod api;
    pub mod config;
}

fn main() {
    yew::Renderer::<app::App>::new().render();
}
