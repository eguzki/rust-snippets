use std::convert::identity;

pub struct GatewayAction {
    pub configurations: Option<Vec<i32>>,
}

pub struct RateLimitPolicy {
    pub gateway_actions: Vec<GatewayAction>,
}

fn main() {
    let rlp = RateLimitPolicy {
        gateway_actions: vec![
            GatewayAction {
                configurations: Some(vec![1, 2, 3]),
            },
            GatewayAction {
                configurations: None,
            },
            GatewayAction {
                configurations: Some(vec![4, 5, 6]),
            },
        ],
    };

    let configs = rlp
        .gateway_actions
        .iter()
        .map(|ga| &ga.configurations)
        .flat_map(identity)
        .collect::<Vec<_>>();
    println!("configs {:?}", configs); // -> configs [[1, 2, 3], [4, 5, 6]]

    let configs = rlp
        .gateway_actions
        .iter()
        .map(|ga| &ga.configurations)
        .collect::<Vec<_>>();
    println!("configs {:?}", configs); // -> configs [Some([1, 2, 3]), None, Some([4, 5, 6])]

    let configs = rlp
        .gateway_actions
        .iter()
        .flat_map(|ga| &ga.configurations)
        .collect::<Vec<_>>();
    println!("configs {:?}", configs); // ->

    let configs = rlp
        .gateway_actions
        .iter()
        .flat_map(|ga| &ga.configurations)
        .flat_map(identity)
        .collect::<Vec<_>>();
    println!("configs {:?}", configs); // -> configs [1, 2, 3, 4, 5, 6]
}
