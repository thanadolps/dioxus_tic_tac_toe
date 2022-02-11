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
    let (x_is_next, set_x_is_next) = use_state(&cx, || true);

    let handle_click = move |i| {
        let mut squares = squares.clone();
        if calculate_winner(&squares).is_some() || squares[i as usize].is_some() {
            return;
        }
        squares[i] = Some(if *x_is_next {"X"} else {"O"});
        set_squares(squares);
        set_x_is_next(!x_is_next);
    };

    let render_square = |i| rsx!(
        Square { value: squares[i], onclick: move |_| handle_click(i) }
    );


    let winner = calculate_winner(squares);
    let status = if let Some(winner) = winner {
        format!("Winner: {}", winner)
    } else {
        format!("Next player: {}", if *x_is_next {"X"} else {"O"})
    };

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

fn calculate_winner(squares: &[Option<&'static str>; 9]) -> Option<&'static str> {
    let lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for i in 0..lines.len() {
        let [a, b, c] = lines[i];
        if squares[a].is_some() && squares[a] == squares[b] && squares[a] == squares[c] {
            return squares[a];
        }
    }
    return None;
}