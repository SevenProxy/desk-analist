use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::layout::DashLayout;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <DashLayout>
      <section class="text-white w-full h-full">
        <div class="py-4 px-10 h-full">
          {"aaaaaaaaaaa"}
        </div>
      </section>
    </DashLayout>
  }
}
