#[derive(Debug)]
pub struct Action {
    name: String,
    val: Result<bool, &'static str>,
}

impl Action {
    pub fn consume(&self) -> Result<bool, &'static str> {
        println!("consume: {}", self.name);
        self.val
    }
}

fn select_action(actions: Vec<Action>) -> Option<Result<Action, &'static str>> {
    actions
        .into_iter()
        .find_map(|action| match action.consume() {
            Ok(true) => Some(Ok(action)),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        })
}

fn main() {
    let actions = vec![
        Action {
            name: "a".to_string(),
            val: Ok(false),
        },
        Action {
            name: "b".to_string(),
            val: Ok(false),
        },
        Action {
            name: "c".to_string(),
            val: Err("some error"),
        },
        Action {
            name: "d".to_string(),
            val: Ok(true),
        },
        Action {
            name: "f".to_string(),
            val: Ok(true),
        },
    ];

    let selected_action = select_action(actions);
    println!(
        "expected action is c, selected action {:?}",
        selected_action
    );

    let actions = vec![
        Action {
            name: "a".to_string(),
            val: Ok(false),
        },
        Action {
            name: "b".to_string(),
            val: Ok(true),
        },
        Action {
            name: "c".to_string(),
            val: Err("some error"),
        },
        Action {
            name: "d".to_string(),
            val: Ok(true),
        },
        Action {
            name: "f".to_string(),
            val: Ok(true),
        },
    ];

    let selected_action = select_action(actions);
    println!(
        "expected action is b, selected action {:?}",
        selected_action
    );
}
