use lazy_static::lazy_static;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::sync::Mutex;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod data;

const MAX_VALUE: u32 = 10000;

fn color(v: u32) -> u32 {
    (((MAX_VALUE - v) as f32 / MAX_VALUE as f32) * 240.0) as u32
}

lazy_static! {
    static ref RNG: Mutex<SmallRng> = Mutex::new(SmallRng::seed_from_u64(777));
}

struct Problem {
    size: usize,
    values: Vec<Vec<u32>>,
}

struct Grid {
    selected: Option<Position>,
}

type Position = (usize, usize);
struct Algo {
    current_position: Position,
    neighboors: Vec<Position>,
}

impl Algo {
    fn new(p: Position) -> Self {
        Algo {
            current_position: p,
            neighboors: vec![],
        }
    }

    fn next(&mut self, problem: &Problem) -> AlgoEvents {
        let mut rng = RNG.lock().unwrap();
        AlgoEvents::SelectedSolution((
            rng.gen_range(0, problem.size),
            rng.gen_range(0, problem.size),
        ))
    }
}

enum AlgoEvents {
    // The position is the current best known solution
    SelectedSolution(Position),
    // The algo will explore the neighborhood
    SelectedNeighborhood(Vec<Position>),
    // The algo evaluated the position
    Evaluated(Position),
}

pub struct Model {
    problem: Problem,
    algo: Algo,
    grid: Grid,
}

pub enum Msg {
    Next,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let size = 70;
        let problem = Problem {
            size,
            values: data::VALUES[0..size]
                .iter()
                .map(|v| v[0..size].to_vec())
                .collect(),
        };
        let grid = Grid { selected: None };
        let algo = Algo::new((0, 0));
        Model {
            problem,
            grid,
            algo,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Next => match self.algo.next(&self.problem) {
                AlgoEvents::SelectedSolution(p) => {
                    self.grid.selected = Some(p);
                }
                _ => unreachable!(),
            },
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div id="control",>
                  <button onclick=|_| Msg::Next,>{ "Next" }</button>
                </div>
                <div id="grid",>
                  { for (0..self.problem.size).map(|i: usize| view_row(i, &self.problem, &self.grid)) }
                </div>
            </div>
        }
    }
}

fn view_row(i: usize, prb: &Problem, grid: &Grid) -> Html<Model> {
    let style = |i: usize, j: usize| {
        if grid.selected == Some((i, j)) {
            format!(
                "background-color: hsla({}, 100%, 50%, 1);",
                color(prb.values[i][j])
            )
        } else {
            "background-color: #D3D3D3;".to_owned()
        }
    };
    html! {
        <div class="row",>
           { for (0..prb.size).map(|j| html! { <div class="cell", style={ style(i, j) },></div> }) }
        </div>
    }
}
