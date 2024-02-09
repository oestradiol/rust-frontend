use yew::{Html, html};
use yew_router::BrowserRouter;
use crate::components::Nav;
use crate::router::AppRouter;

pub fn layout() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <main>
                <AppRouter />
            </main>
        </BrowserRouter>
    }
}