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

#[derive(Props)]
struct BoardProps<'a> {
    squares: [Option<&'static str>; 9],
    onclick: EventHandler<'a, usize>,
}
fn Board<'a>(cx: Scope<'a, BoardProps<'a>>) -> Element {
    let render_square = |i| {
        rsx!(Square {
            value: cx.props.squares[i],
            onclick: move |_| cx.props.onclick.call(i)
        })
    };

    cx.render(rsx!(
        div {
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
    let (history, set_history) = use_state(&cx, || vec![History { squares: [None; 9] }]);
    let (x_is_next, set_x_is_next) = use_state(&cx, || true);

    let handle_click = move |i| {
        let current = &history[history.len() - 1];
        let mut squares = current.squares.clone();
        if calculate_winner(&squares).is_some() || squares[i as usize].is_some() {
            return;
        }
        squares[i] = Some(if *x_is_next { "X" } else { "O" });
        set_history.with_mut(|history| history.push(History { squares }));
        set_x_is_next(!x_is_next);
    };

    
    let current = &history[history.len() - 1];
    let winner = calculate_winner(&current.squares);
    let status = if let Some(winner) = winner {
        format!("Winner: {}", winner)
    } else {
        format!("Next player: {}", if *x_is_next { "X" } else { "O" })
    };

    cx.render(rsx!(
        style { [include_str!("./main.css")] }
        div { class: "game",
            div { class: "game-board",
                Board { squares: current.squares, onclick: handle_click}
            }
        }
        div { class: "game-info",
            div { [status] }
            ol { /* TODO */ }
        }
    ))
}

// ========================================

fn main() {
    dioxus::desktop::launch(Game);
}

#[derive(Clone)]
struct History {
    squares: [Option<&'static str>; 9],
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
