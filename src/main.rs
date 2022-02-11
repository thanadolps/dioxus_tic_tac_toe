#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
struct SquareProps<'a> { value: Option<&'static str>, onclick: EventHandler<'a, ()> }
fn Square<'a>(cx: Scope<'a, SquareProps<'a>>) -> Element {
    cx.render(rsx!(
        button { class: "square",
            onclick: move |_| cx.props.onclick.call(()),
            cx.props.value
        }
    ))
}

fn Board(cx: Scope) -> Element {
    let (squares, set_squares) = use_state(&cx, || [None; 9]);

    let handle_click = |i| {
        let mut squares = squares.clone();
        squares[i] = Some("X");
        set_squares(squares);
    };

    let render_square = |i| rsx!(
        Square { value: squares[i], onclick: move |_| handle_click(i) }
    );

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
