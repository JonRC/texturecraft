//! A malformed colour param is an error, not the command's default colour, and changes nothing.

use serde_json::{Value, json};
use texturecraft_engine::geom::primitives;
use texturecraft_engine::{Session, execute, inspect};

/// A sphere project with a fill layer selected, so the layer and effect commands have a target.
fn session() -> Session {
    let mut s = Session::new_project("t", primitives::sphere(1.0, 24, 12), "builtin:sphere", None, 128).unwrap();
    execute(&mut s, "layer.add.fill", &json!({})).unwrap();
    s
}

/// What a command could have changed: the session as inspected, and the tool's brush material.
fn state(s: &Session) -> Value {
    let mut inspected = inspect::session(s);
    if let Some(o) = inspected.as_object_mut() {
        o.remove("memoryBytes");
        o.remove("pendingTiles");
    }
    json!({"session": inspected, "maskValue": s.tool.mask_value, "material": format!("{:?} {:?}", s.tool.values, s.tool.enabled)})
}

#[test]
fn a_malformed_colour_is_an_error() {
    let rows: [(&str, &str, Value, &str); 12] = [
        ("fill layer, short form", "layer.add.fill", json!({"color": "#fff"}), "color"),
        ("fill layer, a word for a component", "layer.add.fill", json!({"color": [1, "bad", 0]}), "color"),
        ("content fill effect, not hex", "effect.add.fill", json!({"target": "content", "color": "#zzzzzz"}), "color"),
        ("fill source, a name", "layer.fill.set", json!({"channel": "baseColor", "color": "white"}), "color"),
        ("procedural low, too short", "layer.fill.set", json!({"channel": "baseColor", "procedural": "clouds", "low": [0.5, 0.5]}), "low"),
        ("brush, five digits", "tool.material", json!({"channel": "baseColor", "color": "c8102"}), "color"),
        ("brush with a mask value, short form", "tool.material", json!({"maskValue": 0.25, "channel": "baseColor", "color": "#fff"}), "color"),
        ("ID colour mask, a name", "mask.add.colorSelection", json!({"color": "red"}), "color"),
        ("scatter, alpha", "display.set", json!({"scatterColor": "#c8102e80"}), "scatterColor"),
        ("scatter with an exposure, a null component", "display.set", json!({"exposure": 2, "scatterColor": [0.5, null, 0.5]}), "scatterColor"),
        ("replace colour filter, an object", "effect.add.filter", json!({"filter": "replaceColor", "to": {"r": 1}}), "to"),
        ("gradient map, not hex", "effect.add.filter", json!({"filter": "gradientMap", "colors": ["#000000", "#ffffzz"]}), "colors"),
    ];
    for (name, cmd, params, key) in rows {
        let mut s = session();
        let before = state(&s);
        match execute(&mut s, cmd, &params) {
            Ok(r) => panic!("{name}: {cmd} {params} gave {r}"),
            Err(e) => assert!(e.to_string().contains(&format!("`{key}`")), "{name}: {e}"),
        }
        assert_eq!(state(&s), before, "{name}: a failed {cmd} changed the session");
    }
}
