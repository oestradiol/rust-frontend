use yew::{function_component, Html, html};
use crate::AppRouter;
use crate::components::Nav;

#[function_component(Layout)]
pub fn layout() -> Html {
  html! {
      <>
        <Nav />
        <main>
            <AppRouter />
        </main>
      </>
    }
}
