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
  let path: Vec<String> = vec![
    String::from("C:\\Windows\\Temp"),
    String::from("C:\\Users\\SeuUsuario\\AppData\\Local\\Temp"),
  ];
  html! {
    <DashLayout>
      <section class="w-full">
        <div class="w-full h-full">
          <div class="w-full mt-2 mb-8">
            <p class="font-bold text-4xl text-zinc-300">{"Verificar Assinatura dos Processos"}</p>
          </div>
          <div class="my-4 text-xl">
            <p>{"Se o arquivo estiver em locais como:"}</p>
            <div class="w-full mt-4">
              <ul class="list-disc">
                { for path.iter().map(|i| html! {
                  <li class="text-base mb-1 bg-zinc-600 py-2 px-2 rounded-xl w-full">
                    <p>{&i.as_str()}</p>
                  </li>
                }) }
              </ul>
            </div>
            <p>{"Ele pode ser suspeito."}</p>
          </div>
          <div class="mt-8">
            <div class="w-full flex items-center gap-2 mb-2">
              <p>{"Log"}</p>
            </div>
            <pre class="w-full py-2 px-4 rounded-xl">
              <code class="font-bold text-lg w-full min-h-[400px]">
                <div class="mb-2">
                  <p class="text-zinc-500">{"// ✅ Start Build"}</p>
                </div>
                <div class="flex items-center justify-between mb-1">
                  <div class="flex items-center gap-2">
                    <span>{"#1"}</span>
                    <span class="text-[--color]">{"⏱️ 10/10/2025 | "}</span>
                    <span>{"File name"}</span>
                  </div>
                  <div class="bg-green-600 px-2 rounded flex items-center gap-2">
                    <span>
                      <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-memory w-6 h-6" viewBox="0 0 16 16">
                        <path d="M1 3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h4.586a1 1 0 0 0 .707-.293l.353-.353a.5.5 0 0 1 .708 0l.353.353a1 1 0 0 0 .707.293H15a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1zm.5 1h3a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-4a.5.5 0 0 1 .5-.5m5 0h3a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-4a.5.5 0 0 1 .5-.5m4.5.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5zM2 10v2H1v-2zm2 0v2H3v-2zm2 0v2H5v-2zm3 0v2H8v-2zm2 0v2h-1v-2zm2 0v2h-1v-2zm2 0v2h-1v-2z"/>
                      </svg>
                    </span>
                    <span class="">{"7%"}</span>
                  </div>
                  <div class="bg-red-600 px-2 rounded flex items-center gap-2">
                    <span>
                      <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-cpu-fill w-6 h-6" viewBox="0 0 16 16">
                        <path d="M6.5 6a.5.5 0 0 0-.5.5v3a.5.5 0 0 0 .5.5h3a.5.5 0 0 0 .5-.5v-3a.5.5 0 0 0-.5-.5z"/>
                        <path d="M5.5.5a.5.5 0 0 0-1 0V2A2.5 2.5 0 0 0 2 4.5H.5a.5.5 0 0 0 0 1H2v1H.5a.5.5 0 0 0 0 1H2v1H.5a.5.5 0 0 0 0 1H2v1H.5a.5.5 0 0 0 0 1H2A2.5 2.5 0 0 0 4.5 14v1.5a.5.5 0 0 0 1 0V14h1v1.5a.5.5 0 0 0 1 0V14h1v1.5a.5.5 0 0 0 1 0V14h1v1.5a.5.5 0 0 0 1 0V14a2.5 2.5 0 0 0 2.5-2.5h1.5a.5.5 0 0 0 0-1H14v-1h1.5a.5.5 0 0 0 0-1H14v-1h1.5a.5.5 0 0 0 0-1H14v-1h1.5a.5.5 0 0 0 0-1H14A2.5 2.5 0 0 0 11.5 2V.5a.5.5 0 0 0-1 0V2h-1V.5a.5.5 0 0 0-1 0V2h-1V.5a.5.5 0 0 0-1 0V2h-1zm1 4.5h3A1.5 1.5 0 0 1 11 6.5v3A1.5 1.5 0 0 1 9.5 11h-3A1.5 1.5 0 0 1 5 9.5v-3A1.5 1.5 0 0 1 6.5 5"/>
                      </svg>
                    </span>
                    <span class="">{"68%"}</span>
                  </div>
                </div>
                <div class="flex items-center">
                  <p class="text-zinc-300">{"Microsoft Windows sha256 quarta-feira, 10 de fevereiro"}</p>
                  <span class="h-[3px] w-full my-2 rounded-full bg-[--color]"></span>
                </div>
              </code>
            </pre>
          </div>
        </div>
      </section>
    </DashLayout>
  }
}
