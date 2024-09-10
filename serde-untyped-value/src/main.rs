use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Object {
    pub name: String,
    pub value: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    pub objects: Vec<Object>,
}

fn main() {
    let config: &str = r#"{
        "objects": [
        {
            "name": "value_with_str",
            "value": "some-value"
        },
        {
            "name": "value_with_int",
            "value": 1 
        },
        {
            "name": "value_with_bool",
            "value": true 
        },
        {
            "name": "value_with_null",
            "value": null
        }
        ]
    }"#;

    let res = serde_json::from_str::<Configuration>(config);
    if let Err(ref e) = res {
        eprintln!("{}", e);
    }
    assert!(res.is_ok());

    let config_obj = res.unwrap();

    for obj in config_obj.objects.iter() {
        println!("object: {:?}", obj);
    }
}
