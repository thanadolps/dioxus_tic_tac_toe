#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
struct SquareProps { value: u8 }
fn Square(cx: Scope<SquareProps>) -> Element {
    cx.render(rsx!(
        button { class: "square",
            [cx.props.value.to_string()]
        }
    ))
}

fn Board(cx: Scope) -> Element {
    let render_square = |i| rsx!(Square { value: i });

    let status = "Next player: X";

    cx.render(rsx!(
        div {
            div { class: "status", [status] }
            div { class: "board-row", [
                render_square(0),
                render_square(1),
                render_square(2)
            ]}
            div { class: "board-row", [
                render_square(3),
                render_square(4),
                render_square(5)
            ]}
            div { class: "board-row", [
                render_square(6),
                render_square(7),
                render_square(8)
            ]}
        }
    ))
}

fn Game(cx: Scope) -> Element {
    cx.render(rsx!(
        style { [include_str!("./main.css")] }
        div { class: "game",
            div { class: "game-board",
                Board {}
            }
        }
        div { class: "game-info",
            div { /* status */ }
            ol { /* TODO */ }
        }
    ))
}

// ========================================

fn main() {
    dioxus::desktop::launch(Game);
}
