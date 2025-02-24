use yew::prelude::*;
use crate::date::List;

#[derive(Properties, PartialEq)]
pub struct PropsNavbarItem {
  pub list: List,
}

pub struct NavbarItem;

impl Component for NavbarItem {
  type Message = ();
  type Properties = PropsNavbarItem;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="text-white w-full my-4 px-1">
        <menu class="w-full flex items-center gap-2 relative group">
          <span class="w-[10%] h-[3px] relative">
            <span class="absolute left-0 top-0 h-full w-0 bg-white transition-all duration-300 group-hover:w-full"></span>
          </span>
          <div class="font-bold text-2xl select-none eventmouse-none uppercase">
            { ctx.props().list.icon.clone() }
          </div>
          <span class="w-full h-[3px] relative">
            <span class="absolute right-0 top-0 h-full w-0 bg-white transition-all duration-300 group-hover:w-full"></span>
          </span>
        </menu>
        <div class="my-2 flex items-center gap-1">
          <div class="h-full max-w-[10%] rotate-90 flex items-center gap-2 whitespace-nowrap">
            <span class="font-bold textbase">{"///"}</span>
            <p class="font-bold uppercase text-base">{ ctx.props().list.name.as_str() }</p>
          </div>
          <ul class="min-h-[140px] w-full my-2 mr-2 bg-[--foreground] rounded-xl text-white">
            <li>
              { for ctx.props().list.item.iter().map(|it| html! {
                <button class="outline-0 w-full relative px-6 py-3 text-white rounded-xl overflow-hidden group">
                  <p class="uppercase font-bold text-xl relative z-10">
                    { &it.name.as_str() }
                  </p>
                  <span class="absolute left-0 top-0 h-0.5 w-0 bg-white transition-all duration-300 group-hover:w-full"></span>
                  <span class="absolute right-0 bottom-0 h-0.5 w-0 bg-white transition-all duration-300 group-hover:w-full"></span>
                  <span class="absolute left-0 bottom-0 h-0 w-0.5 bg-white transition-all duration-300 group-hover:h-full"></span>
                  <span class="absolute right-0 top-0 h-0 w-0.5 bg-white transition-all duration-300 group-hover:h-full"></span>
                </button>
              }) }
            </li>
          </ul>
        </div>
      </div>
    }
  }

}