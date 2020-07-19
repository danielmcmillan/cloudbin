#![recursion_limit = "1024"]
use std::mem::swap;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct TodoComponent {
    link: ComponentLink<Self>,
    text: String,
    items: Vec<String>,
    input_ref: NodeRef,
}

enum Msg {
    AddItem,
    RemoveItem(usize),
    UpdateText(String),
    None,
}

impl TodoComponent {
    fn render_item(&self, (index, text): (usize, &String)) -> Html {
        html! {
            <li>{text}<button onclick=self.link.callback(move |_| Msg::RemoveItem(index))>{"Remove"}</button></li>
        }
    }
}

impl Component for TodoComponent {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoComponent {
            link,
            text: String::new(),
            items: vec![],
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddItem => {
                if let Some(element) = self.input_ref.cast::<HtmlInputElement>() {
                    let _ = element.focus();
                }
                if !self.text.is_empty() {
                    let mut text = String::new();
                    swap(&mut self.text, &mut text);
                    self.items.push(text);
                    true
                } else {
                    false
                }
            }
            Msg::RemoveItem(index) => {
                self.items.remove(index);
                true
            }
            Msg::UpdateText(text) => {
                self.text = text;
                true
            }
            Msg::None => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input ref=self.input_ref.clone() value=self.text onchange=self.link.callback(|change| {
                    if let yew::html::ChangeData::Value(text) = change {
                        Msg::UpdateText(text)
                    } else {
                        Msg::None
                    }
                }) />
                <button onclick=self.link.callback(|_| Msg::AddItem)>{ "Add" }</button>
                <ul>{ for self.items.iter().enumerate().map(|item| TodoComponent::render_item(self, item)) }</ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<TodoComponent>::new().mount_to_body();
}
