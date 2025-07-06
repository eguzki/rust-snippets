use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
struct Value {
    id: u32,
}

#[derive(Debug)]
struct Object {
    id: u32,
}

impl Object {
    fn get(&self) -> Value {
        // Very expensive operation
        println!("Expensive operation for task {}", self.id);
        Value { id: self.id }
    }
}

type Cache = HashMap<u32, Value>;

struct Task {
    id: u32,
    objets: Vec<Object>,
}

impl Task {
    fn run(&self, cache: &mut Cache) {
        for object in &self.objets {
            println!("Run task {}, object {} ", self.id, object.id);
            let value = cache.entry(object.id).or_insert_with(|| object.get());
            println!("Value for object {object:?}: {value:?}");
        }
    }
}

struct App {
    config: Vec<Task>,
    cache: Cache,
}

impl App {
    fn run(&mut self) {
        for task in &self.config {
            task.run(&mut self.cache);
        }
    }
}

fn main() {
    let tasks = vec![
        Task {
            id: 1,
            objets: vec![Object { id: 1 }, Object { id: 2 }],
        },
        Task {
            id: 2,
            objets: vec![Object { id: 2 }, Object { id: 3 }, Object { id: 1 }],
        },
        Task {
            id: 3,
            objets: vec![Object { id: 4 }, Object { id: 1 }, Object { id: 2 }],
        },
    ];

    let mut app = App {
        config: tasks,
        cache: Cache::new(),
    };

    app.run();
}
