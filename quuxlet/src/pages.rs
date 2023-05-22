pub use create::Create;
pub use discover::Discover;
pub use error::Error;
pub use index::Index;
pub use set::Set;

use quux::prelude::*;

pub mod create;
mod discover;
pub mod error;
mod index;
mod set;

#[derive(Serialize, Deserialize, Clone)]
pub struct Head {
    title: String,
}

impl Head {
    #[must_use]
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

impl component::Init for Head {
    type Props = String;

    fn init(props: Self::Props) -> Self {
        Self { title: props }
    }
}

impl Component for Head {
    fn render(self, _: Context<Self>) -> impl Item
    where
        Self: Sized,
    {
        head()
            .child(meta().attribute("charset", "UTF-8"))
            .child(
                meta()
                    .attribute("http-equiv", "X-UA-Compatible")
                    .attribute("content", "IE=edge"),
            )
            .child(
                meta()
                    .attribute("name", "viewport")
                    .attribute("content", "width=device-width, initial-scale=1.0"),
            )
            .child(style().text(include_str!("../dist/output.css")))
            .child(title().text(self.title))
    }
}

/*
<div class="navbar bg-base-100">
  <div class="navbar-start">
    <div class="dropdown">
      <label class="btn btn-ghost lg:hidden">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" /></svg>
      </label>
      <ul class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
        <li><a>Item 1</a></li>
        <li>
          <a class="justify-between">
            Parent
            <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8.59,16.58L13.17,12L8.59,7.41L10,6L16,12L10,18L8.59,16.58Z"/></svg>
          </a>
          <ul class="p-2">
            <li><a>Submenu 1</a></li>
            <li><a>Submenu 2</a></li>
          </ul>
        </li>
        <li><a>Item 3</a></li>
      </ul>
    </div>
    <a class="btn btn-ghost normal-case text-xl">daisyUI</a>
  </div>
  <div class="navbar-center hidden lg:flex">
    <ul class="menu menu-horizontal px-1">
      <li><a>Item 1</a></li>
      <li tabindex="0">
        <a>
          Parent
          <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"><path d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"/></svg>
        </a>
        <ul class="p-2">
          <li><a>Submenu 1</a></li>
          <li><a>Submenu 2</a></li>
        </ul>
      </li>
      <li><a>Item 3</a></li>
    </ul>
  </div>
  <div class="navbar-end">
    <a class="btn">Get started</a>
  </div>
</div>
 */

#[must_use]
pub fn nav_bar() -> impl Item {
    fn link(text: &str, href: &str) -> impl Item {
        a().class("btn btn-ghost normal-case text-xl")
            .attribute("href", href)
            .text(text)
    }

    nav()
        .class("navbar bg-primary text-primary-content")
        .child(link("Discover", "/discover"))
        .child(link("Create", "/create"))
}
