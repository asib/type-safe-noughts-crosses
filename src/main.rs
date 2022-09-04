#![feature(generic_associated_types, specialization)]
#![allow(incomplete_features)]

use either::*;
use yew::prelude::*;

mod game;

#[function_component(App)]
fn app() -> Html {
    let game_state = use_state(|| Either::Left(game::State::default()));
    let error = use_state(|| None);

    let on_click = {
        let game_state = game_state.clone();
        let error = error.clone();

        Callback::from(move |(row, col)| {
            for_both!(game_state.as_ref(), state => match state.next((row, col)) {
                Ok(s) => game_state.set(s.into()),
                Err(err) => error.set(Some(format!("{err:?}"))),
            })
        })
    };

    html! {
        <>
            <Board board={for_both!(game_state.as_ref(), s => s.board.clone())} on_click={on_click} />
        </>
    }
}

#[derive(Properties, PartialEq)]
struct BoardProps {
    board: game::Board,
    on_click: Callback<(usize, usize)>,
}

#[function_component(Board)]
fn board(BoardProps { board, on_click }: &BoardProps) -> Html {
    let spots = board
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            let spots = row
                .iter()
                .enumerate()
                .map(|(column_index, spot)| {
                    let on_click = on_click.clone();
                    html! {
                        <Spot spot={*spot} on_click={Callback::from(move |_| on_click.emit((row_index, column_index)))} />
                    }
                })
                .collect::<Html>();

            html! {
                <div class="col-4">
                    { spots }
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="d-flex">
            { spots }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SpotProps {
    spot: game::Spot,
    on_click: Callback<()>,
}
#[function_component(Spot)]
fn spot(SpotProps { spot, on_click }: &SpotProps) -> Html {
    let on_click = on_click.clone();
    html! {
        <div onclick={move |_| on_click.emit(())} class="m-2 border w-25 h-100 border-primary">{ match spot {
            game::Spot::Player(p) => format!("{p:?}"),
            game::Spot::Empty => "".into()
        } }</div>
    }
}

fn main() {
    yew::start_app::<App>();
}
