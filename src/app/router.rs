// use yew::{function_component, Html, html};
// use yew_router::{Routable, Switch};
// use crate::app::{Home};
//
// #[derive(Clone, Routable, PartialEq)]
// pub enum Route {
//     #[at("/")]
//     Home,
//     #[not_found]
//     #[at("/404")]
//     NotFound,
// }
//
// pub fn route_match(route: Route) -> Html {
//     match route {
//         Route::Home => html! { <Home /> },
//         Route::NotFound => html! { <div class="text-red-500">{"404"}</div> },
//     }
// }
//
// #[function_component(AppRouter)]
// pub fn app_router() -> Html {
//     html! ( <Switch<Route> render={route_match} /> )
// }