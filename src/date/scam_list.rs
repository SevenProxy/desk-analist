use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
  pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct List {
  pub name: String,
  pub icon: Html,
  pub item: Vec<Item>,
}

pub fn get_fetch_scam_list() -> Vec<List> {
  vec![
    List {
      name: String::from("Pasta"),
      icon: html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-folder-fill w-6 h-6" viewBox="0 0 16 16">
          <path d="M9.828 3h3.982a2 2 0 0 1 1.992 2.181l-.637 7A2 2 0 0 1 13.174 14H2.825a2 2 0 0 1-1.991-1.819l-.637-7a2 2 0 0 1 .342-1.31L.5 3a2 2 0 0 1 2-2h3.672a2 2 0 0 1 1.414.586l.828.828A2 2 0 0 0 9.828 3m-8.322.12q.322-.119.684-.12h5.396l-.707-.707A1 1 0 0 0 6.172 2H2.5a1 1 0 0 0-1 .981z"/>
        </svg>
      },
      item: vec![
        Item {
          name: String::from("temp"),
        },
        Item {
          name: String::from("system32")
        }
      ]
    }
  ]
}
