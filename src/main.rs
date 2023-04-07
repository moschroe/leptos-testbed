use actix_web::HttpServer;
use leptos::leptos_server::LeptosServerFnRegistry;
use leptos::server_fn::ServerFunctionRegistry;
use leptos_testbed::model::register_server_functions;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_testbed::app::{App, AppProps};

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    register_server_functions();

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        let a=actix_web::App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", site_root))
        //.wrap(middleware::Compress::default())
		;

        dbg!(&LeptosServerFnRegistry::paths_registered());

        a
    })
    .workers(1)
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
