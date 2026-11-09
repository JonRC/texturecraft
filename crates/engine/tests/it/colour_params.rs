//! A malformed colour param is an error, not the command's default colour.

use serde_json::{Value, json};
use texturecraft_engine::geom::primitives;
use texturecraft_engine::{Session, execute};

/// A sphere project with a fill layer selected, so the layer and effect commands have a target.
fn session() -> Session {
    let mut s = Session::new_project("t", primitives::sphere(1.0, 24, 12), "builtin:sphere", None, 128).unwrap();
    execute(&mut s, "layer.add.fill", &json!({})).unwrap();
    s
}

#[test]
fn a_malformed_colour_is_an_error() {
    let rows: [(&str, &str, Value, &str); 9] = [
        ("fill layer, short form", "layer.add.fill", json!({"color": "#fff"}), "color"),
        ("content fill effect, not hex", "effect.add.fill", json!({"target": "content", "color": "#zzzzzz"}), "color"),
        ("fill source, a name", "layer.fill.set", json!({"channel": "baseColor", "color": "white"}), "color"),
        ("procedural low, too short", "layer.fill.set", json!({"channel": "baseColor", "procedural": "clouds", "low": [0.5, 0.5]}), "low"),
        ("brush, five digits", "tool.material", json!({"channel": "baseColor", "color": "c8102"}), "color"),
        ("ID colour mask, a name", "mask.add.colorSelection", json!({"color": "red"}), "color"),
        ("scatter, alpha", "display.set", json!({"scatterColor": "#c8102e80"}), "scatterColor"),
        ("replace colour filter, an object", "effect.add.filter", json!({"filter": "replaceColor", "to": {"r": 1}}), "to"),
        ("gradient map, not hex", "effect.add.filter", json!({"filter": "gradientMap", "colors": ["#000000", "#ffffzz"]}), "colors"),
    ];
    for (name, cmd, params, key) in rows {
        let mut s = session();
        match execute(&mut s, cmd, &params) {
            Ok(r) => panic!("{name}: {cmd} {params} gave {r}"),
            Err(e) => assert!(e.to_string().contains(&format!("`{key}`")), "{name}: {e}"),
        }
    }
}
