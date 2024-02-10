#!forbid(unsafe_code)

mod components;
mod layout;
mod routes;

use yew::{AttrValue, function_component, Html, html, Properties};
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::{BrowserRouter, Routable, Router, Switch};
use crate::layout::Layout;
use crate::routes::Home;

// Routing
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn route_match(route: Route) -> Html {
  match route {
    Route::Home => html! { <Home /> },
    Route::NotFound => html! { <div class="text-red-500">{"404"}</div> },
  }
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
  html! ( <Switch<Route> render={route_match} /> )
}




// Client Side Rendering
#[function_component(App)]
pub fn client_app() -> Html {
  html! {
      <BrowserRouter>
          <Layout />
      </BrowserRouter>
  }
}




// Server Side Rendering
#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
  pub url: AttrValue,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
  let history = AnyHistory::from(MemoryHistory::new());
  history.push(&*props.url);

  return html! {
      <Router history={history}>
          <Layout />
      </Router>
  };
}
