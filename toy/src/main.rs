use dioxus::prelude::*;

fn main() {
    dioxus_tui::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            width: "100%",
            height: "20px",
            background_color: "black",
            justify_content: "center",
            align_items: "center",

            "Hello world!"
        }
    })
}
