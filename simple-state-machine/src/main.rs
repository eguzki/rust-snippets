mod fsm {
    enum State {
        RequestHeaders,
        RequestBody,
    }

    pub struct FSM {
        state: State,
        left: Vec<super::Action>,
        right: Vec<super::Action>,
    }

    impl FSM {
        pub fn new() -> Self {
            Self {
                state: State::RequestHeaders,
                left: Vec::default(),
                right: Vec::default(),
            }
        }
        pub fn next(&mut self, action: super::Action) {
            match self.state {
                State::RequestHeaders => {
                    if action.val >= 10 {
                        self.right.push(action);
                        self.state = State::RequestBody;
                    } else {
                        self.left.push(action);
                    }
                }
                State::RequestBody => self.right.push(action),
            }
        }

        pub fn collect(self) -> (Vec<super::Action>, Vec<super::Action>) {
            (self.left, self.right)
        }
    }
}

use fsm::FSM;

#[derive(Debug)]
pub struct Action {
    pub val: u32,
}

fn action_partitioner(actions: Vec<Action>) -> (Vec<Action>, Vec<Action>) {
    let mut state_machine = FSM::new();
    for action in actions {
        state_machine.next(action);
    }
    state_machine.collect()
}

fn main() {
    let actions: Vec<Action> = vec![1, 2, 3, 10, 5, 6]
        .into_iter()
        .map(|x| Action { val: x })
        .collect();

    let (left, right) = action_partitioner(actions);
    println!("left {:?}", left);
    println!("right {:?}", right);
}
