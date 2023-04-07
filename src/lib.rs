pub mod app;
use cfg_if::cfg_if;

pub mod model;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;
        use crate::model::*;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      // initializes logging using the `log` crate
      _ = console_log::init_with_level(log::Level::Trace);
      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}
