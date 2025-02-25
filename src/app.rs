use yew::prelude::*;
use crate::layout::DashLayout;
use crate::components::console::ConsolePre;

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
            <ConsolePre />
          </div>
        </div>
      </section>
    </DashLayout>
  }
}
