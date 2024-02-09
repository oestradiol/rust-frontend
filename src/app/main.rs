use yew::prelude::*;
use yew::Renderer;

mod router;
pub mod routes;

pub mod components;
mod layout;

fn main() {
    Renderer::<App>::new().render();
}

#[inline]
#[allow(unused_braces)]
#[function_component(App)]
fn layout() -> Html { layout::layout() }