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
    neighbourhood: Option<Vec<Position>>,
}

type Position = (usize, usize);
struct Algo {
    current_position: Position,
    neighbours: Vec<Position>,
    evaluated: Vec<Position>,
    width: usize,
    size: usize,
    done: bool,
}

enum CellStatus {
    CurrentSolution,
    Evaluating,
    Evaluated,
    Neighbour,
    None,
}

impl Algo {
    fn neighbours((x, y): Position, w: usize, s: usize) -> Vec<Position> {
        let mut neighbours = vec![];
        let x_min = x.checked_sub(w).unwrap_or(0);
        let x_max = usize::min(x + w + 1, s);
        let y_min = y.checked_sub(w).unwrap_or(0);
        let y_max = usize::min(y + w + 1, s - 1);
        for i in x_min..x_max {
            for j in y_min..y_max {
                if (i != x) || (j != y) {
                    neighbours.push((i, j));
                }
            }
        }
        neighbours
    }

    fn new(p: Position, w: usize, s: usize) -> Self {
        Algo {
            current_position: p,
            width: w,
            size: s,
            neighbours: Algo::neighbours(p, w, s),
            done: false,
            evaluated: vec![],
        }
    }

    fn status(&self, (i, j): Position) -> CellStatus {
        if (i, j) == self.current_position {
            CellStatus::CurrentSolution
        } else if self.evaluated.iter().any(|(x, y)| (i == *x) && (j == *y)) {
            CellStatus::Evaluated
        } else if Some(&(i, j)) == self.neighbours.last() {
            CellStatus::Evaluating
        } else if self.neighbours.iter().any(|(x, y)| (i == *x) && (j == *y)) {
            CellStatus::Neighbour
        } else {
            CellStatus::None
        }
    }

    fn next(&mut self, problem: &Problem) {
        if self.done {
            return;
        }
        match self.neighbours.pop() {
            Some(p) => self.evaluated.push(p),
            None => {
                let mut evaluated = vec![];
                std::mem::swap(&mut evaluated, &mut self.evaluated);
                match evaluated.iter().min_by_key(|(x, y)| problem.values[*x][*y]) {
                    Some((x, y)) => {
                        let (i, j) = self.current_position;
                        if problem.values[*x][*y] < problem.values[i][j] {
                            // a better solution is found!
                            self.current_position = (*x, *y);
                            self.neighbours =
                                Algo::neighbours(self.current_position, self.width, self.size);
                        } else {
                            // local optimum is found
                            self.done = true
                        }
                    }
                    None => unreachable!(),
                }
            }
        }
    }
}

pub struct Model {
    problem: Problem,
    algo: Algo,
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
        let algo = Algo::new((30, 30), 2, size);
        Model { problem, algo }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Next => self.algo.next(&self.problem),
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
                  { for (0..self.problem.size).map(|i: usize| view_row(i, &self.problem, &self.algo)) }
                </div>
            </div>
        }
    }
}

fn view_row(i: usize, prb: &Problem, algo: &Algo) -> Html<Model> {
    let style = |i: usize, j: usize| match algo.status((i, j)) {
        CellStatus::CurrentSolution => "background-color: green".to_owned(),
        CellStatus::Evaluated => format!(
            "background-color: hsla({}, 100%, 50%, 1);",
            color(prb.values[i][j])
        ),
        CellStatus::Evaluating => "background-color: red".to_owned(),
        CellStatus::Neighbour => "background-color: #80FF33;".to_owned(),
        CellStatus::None => "background-color: #D3D3D3;".to_owned(),
    };
    html! {
        <div class="row",>
           { for (0..prb.size).map(|j| html! { <div class="cell", style={ style(i, j) },></div> }) }
        </div>
    }
}
