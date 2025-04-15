
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct ChordChar {
    chord: Option<String>,
    character: String,
}

#[wasm_bindgen]
impl ChordChar {
    #[wasm_bindgen(getter)]
    pub fn chord(&self) -> Option<String> {
        self.chord.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn character(&self) -> String {
        self.character.clone()
    }
}

#[wasm_bindgen]
pub fn parse(input: &str) -> js_sys::Array {
    let mut lines = js_sys::Array::new();

    for line in input.lines() {
        let mut result = js_sys::Array::new();
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '[' {
                let mut chord_name = String::new();
                while let Some(&next_char) = chars.peek() {
                    chars.next();
                    if next_char == ']' {
                        break;
                    }
                    chord_name.push(next_char);
                }

                if let Some(&next_char) = chars.peek() {
                    if next_char != '[' {
                        let cc = ChordChar {
                            chord: Some(chord_name),
                            character: next_char.to_string(),
                        };
                        chars.next();
                        result.push(&wasm_bindgen::JsValue::from(cc));
                    } else {
                        let cc = ChordChar {
                            chord: Some(chord_name),
                            character: " ".to_string(),
                        };
                        result.push(&wasm_bindgen::JsValue::from(cc));
                    }
                } else {
                    let cc = ChordChar {
                        chord: Some(chord_name),
                        character: " ".to_string(),
                    };
                    result.push(&wasm_bindgen::JsValue::from(cc));
                }
            } else {
                let cc = ChordChar {
                    chord: None,
                    character: c.to_string(),
                };
                result.push(&wasm_bindgen::JsValue::from(cc));
            }
        }

        lines.push(&result);
    }

    lines
}