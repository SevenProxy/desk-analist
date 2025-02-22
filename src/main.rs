/*
 - Copyright @SevenProxy (github)
*/

mod app;
mod layout;
mod components;
mod ui;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
