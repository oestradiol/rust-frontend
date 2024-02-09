mod router;

pub mod routes;
use crate::routes::Home;

pub mod components;
use crate::components::Nav;

use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn layout() -> Html {
    html! {
        // <BrowserRouter>
            <div class="min-h-screen">
                <Nav />
                <main>
                    <Home />
                    // <AppRouter />
                </main>
            </div>
        // </BrowserRouter>
    }
}