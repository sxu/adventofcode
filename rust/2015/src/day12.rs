use serde_json::Value;

fn sum_json(value: &Value, skip_red: bool) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| sum_json(v, skip_red)).sum(),
        Value::Object(o) => {
            for (_, v) in o.iter() {
                if let Value::String(s) = v
                    && s == "red"
                    && skip_red
                {
                    return 0;
                }
            }
            o.iter().map(|(_, v)| sum_json(v, skip_red)).sum()
        }
        _ => 0,
    }
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path)
        .expect("Failed to read input")
        .trim()
        .to_owned();
    let value = serde_json::from_str::<Value>(&input).expect("Failed to parse input");
    assert!(sum_json(&value, false) == 156366);
    assert!(sum_json(&value, true) == 96852);
}
