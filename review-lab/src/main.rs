use serde_json::{json, Value};
use std::io::{self, Read};

fn consume(mode: &str, input: &Value) -> Result<Value, String> {
    if !input.is_object() {
        return Err("expected an object".into());
    }
    match mode {
        "event" => {
            if input["type"] != "activity.completed" {
                return Err("unknown event type".into());
            }
            let id = input["activityId"]
                .as_str()
                .filter(|s| !s.is_empty())
                .ok_or("missing activityId")?;
            Ok(json!({"processed": 1, "activityId": id}))
        }
        "options" => match input.get("replicas") {
            None => Ok(json!({"mode": "default"})),
            Some(value) => match value.as_u64() {
                Some(0) => Ok(json!({"mode": "disabled", "replicas": 0})),
                Some(n) => Ok(json!({"mode": "explicit", "replicas": n})),
                None => Err("replicas must be a nonnegative integer".into()),
            },
        },
        _ => Err("usage: review-lab event|options|deployment".into()),
    }
}

fn main() -> Result<(), String> {
    let mode = std::env::args().nth(1).ok_or("expected command")?;
    if mode == "deployment" {
        let manifest: Value = serde_json::from_str(include_str!("../deployment/workload.json"))
            .map_err(|e| e.to_string())?;
        println!("{}", manifest);
        return Ok(());
    }
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    let input = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    println!("{}", consume(&mode, &input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn completed_event() {
        assert_eq!(
            consume(
                "event",
                &json!({"type":"activity.completed","activityId":"fixture-1"})
            )
            .unwrap(),
            json!({"processed":1,"activityId":"fixture-1"})
        );
    }
    #[test]
    fn rejects_unknown_events() {
        assert!(consume("event", &json!({"type":"unknown","activityId":"fixture-1"})).is_err());
    }
    #[test]
    fn preserves_presence() {
        for (input, expected) in [
            (json!({}), json!({"mode":"default"})),
            (
                json!({"replicas":0}),
                json!({"mode":"disabled","replicas":0}),
            ),
            (
                json!({"replicas":3}),
                json!({"mode":"explicit","replicas":3}),
            ),
        ] {
            assert_eq!(consume("options", &input).unwrap(), expected);
        }
    }
    #[test]
    fn rejects_malformed_options() {
        for input in [
            json!({"replicas":null}),
            json!({"replicas":-1}),
            json!({"replicas":"0"}),
        ] {
            assert!(consume("options", &input).is_err());
        }
    }
}
