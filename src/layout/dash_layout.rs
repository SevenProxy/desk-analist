use yew::prelude::*;
use crate::components::navbar::{NavabarRoot, NavbarLogo, NavbarItem};
use crate::components::header::{HeaderRoot, HeaderTitle};
use crate::date::get_fetch_scam_list;

#[derive(Properties, PartialEq)]
pub struct PropsDashLayout {
  #[prop_or_default]
  pub children: Children,
}

pub struct DashLayout;

impl Component for DashLayout  {
  type Message = ();
  type Properties = PropsDashLayout;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let fetch_scam_list = get_fetch_scam_list();

    html! {
      <main class="orbitron-font w-full h-full">
        <div class="flex items-top w-full h-full relative">
          <NavabarRoot>
            <NavbarLogo />
            { for fetch_scam_list.iter().map(|list| html! {
              <NavbarItem list={list.clone()} />
            })}
          </NavabarRoot>
          <div class="w-full z-20 bg-[--foreground]">
            <HeaderRoot>
              <HeaderTitle text={"Dashboard"} />
            </HeaderRoot>
            <slot class="text-white w-full h-full">
              <div class="py-4 px-10 h-full">
                { for ctx.props().children.iter() }
              </div>
            </slot>
          </div>
        </div>
      </main>
    }
  }

}