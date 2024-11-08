#[derive(Debug, Clone)]
pub struct Data {
    pub name: String,
}

pub struct Action {
    pub data: Data,
    pub predicates: String,
    pub scope: String,
    pub service: String,
}

pub struct ActionSet {
    pub actions: Vec<Action>,
}

#[derive(Debug)]
pub struct DataSet {
    pub data: Data,
    pub predicates: String,
}

#[derive(Debug)]
pub struct MergedAction {
    pub data_sets: Vec<DataSet>,
    pub scope: String,
    pub service: String,
}

#[derive(Debug)]
pub struct MergedActionSet {
    pub merged_actions: Vec<MergedAction>,
}

impl MergedActionSet {
    fn new() -> Self {
        Self {
            merged_actions: vec![],
        }
    }

    fn add(&mut self, action: &Action) {
        let data_set = DataSet {
            data: action.data.clone(),
            predicates: action.predicates.clone(),
        };
        match self.merged_actions.iter().position(|merged_action| {
            merged_action.scope == action.scope && merged_action.service == action.service
        }) {
            Some(idx) => self.merged_actions[idx].data_sets.push(data_set),
            None => self.merged_actions.push(MergedAction {
                data_sets: vec![data_set],
                scope: action.scope.clone(),
                service: action.service.clone(),
            }),
        }
    }
}

impl From<ActionSet> for MergedActionSet {
    fn from(action_set: ActionSet) -> Self {
        let mut merged_action_set = MergedActionSet::new();

        action_set
            .actions
            .iter()
            .for_each(|action| merged_action_set.add(action));

        merged_action_set
    }
}

fn main() {
    let source = ActionSet {
        actions: vec![
            Action {
                data: Data {
                    name: "data1".into(),
                },
                predicates: "predicates1".into(),
                scope: "scope1".into(),
                service: "service1".into(),
            },
            Action {
                data: Data {
                    name: "data2".into(),
                },
                predicates: "predicates2".into(),
                scope: "scope2".into(),
                service: "service2".into(),
            },
            Action {
                data: Data {
                    name: "data3".into(),
                },
                predicates: "predicates3".into(),
                scope: "scope1".into(),
                service: "service1".into(),
            },
        ],
    };
    println!("{:#?}", MergedActionSet::from(source));
}
