use yew::prelude::*;

pub struct NavBarItems;

impl Component for NavBarItems {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="text-white w-full my-4 px-1">
        <menu class="w-full flex items-center gap-2 relative group">
          <span class="w-[10%] h-[3px] relative">
            <span class="absolute left-0 top-0 h-full w-0 bg-white transition-all duration-300 group-hover:w-full"></span>
          </span>
          <div class="font-bold text-2xl select-none eventmouse-none uppercase">
            <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-folder-fill w-6 h-6" viewBox="0 0 16 16">
              <path d="M9.828 3h3.982a2 2 0 0 1 1.992 2.181l-.637 7A2 2 0 0 1 13.174 14H2.825a2 2 0 0 1-1.991-1.819l-.637-7a2 2 0 0 1 .342-1.31L.5 3a2 2 0 0 1 2-2h3.672a2 2 0 0 1 1.414.586l.828.828A2 2 0 0 0 9.828 3m-8.322.12q.322-.119.684-.12h5.396l-.707-.707A1 1 0 0 0 6.172 2H2.5a1 1 0 0 0-1 .981z"/>
            </svg>
          </div>
          <span class="w-full h-[3px] relative">
            <span class="absolute right-0 top-0 h-full w-0 bg-white transition-all duration-300 group-hover:w-full"></span>
          </span>
        </menu>
        <div class="my-2 flex items-center gap-1">
          <div class="h-full max-w-[10%] rotate-90 flex items-center gap-2 whitespace-nowrap">
            <span class="font-bold textbase">{"///"}</span>
            <p class="font-bold uppercase text-base">{"Pasta"}</p>
          </div>
          <ul class="min-h-[140px] w-full my-2 mr-2 bg-[--foreground] rounded-xl text-white">
            <li>
              <button class="outline-0 w-full relative px-6 py-3 text-white rounded-xl overflow-hidden group">
                <p class="uppercase font-bold text-xl relative z-10">
                  {"Temp"}
                </p>
                <span class="absolute left-0 top-0 h-0.5 w-0 bg-white transition-all duration-300 group-hover:w-full"></span>
                <span class="absolute right-0 bottom-0 h-0.5 w-0 bg-white transition-all duration-300 group-hover:w-full"></span>
                <span class="absolute left-0 bottom-0 h-0 w-0.5 bg-white transition-all duration-300 group-hover:h-full"></span>
                <span class="absolute right-0 top-0 h-0 w-0.5 bg-white transition-all duration-300 group-hover:h-full"></span>
              </button>
            </li>
          </ul>
        </div>
      </div>
    }
  }

}