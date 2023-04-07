use crate::model::fetch_apps;
use crate::model::AppConfigInfo;
use crate::model::FetchAppConfig;
use futures_util::future::{select, Either};
use futures_util::TryFutureExt;
use gloo_timers::future::TimeoutFuture;
use leptos::html::Li;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::{debug, info};
use std::future::Future;
use std::num::ParseIntError;
use std::time::Duration;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

type ResFetchAppConfigs = Result<Vec<AppConfigInfo>, ServerFnError>;

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let act_update: Action<(), ResFetchAppConfigs> = create_action(cx, |_: &()| fetch_apps());
    let on_click = move |_| act_update.dispatch(());

    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li>{format!("{:?}", e)}</li>})
                    .collect::<Vec<_>>()
            })
        };

        view! { cx,
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let apps_view = move || {
        debug!("apps_view()");
        act_update
            .value()
            .with(|opt_res| -> Result<Vec<_>, ServerFnError> {
                debug!("act_update.value()! {:?}", opt_res);
                match opt_res {
                    Some(Ok(appcfgs)) => Ok(appcfgs
                        .iter()
                        .map(|item| {
                            let cat = format!("{} {:#?}", act_update.version().get(), item);
                            view! { cx, <li><pre>{cat}</pre></li> }
                        })
                        .collect::<Vec<_>>()),
                    Some(Err(err)) => Err(err.clone()),
                    None => Ok(vec![]),
                }
            })
    };

    // initial load
    #[cfg(target_family = "wasm")]
    {
        set_timeout(
            move || {
                info!("timeout dispatching load...");
                act_update.dispatch(());
            },
            Duration::from_millis(500),
        );
    }

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Update!"</button>
        <ErrorBoundary fallback>
            <Transition fallback=move || view! { cx, <div>"Loading (Suspense Fallback)..."</div>}>
                <ul>
                    {apps_view}
                </ul>
            </Transition>
        </ErrorBoundary>
    }
}
