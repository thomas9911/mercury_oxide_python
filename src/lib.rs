extern crate liquid;
extern crate pyo3;
#[macro_use]
extern crate serde;

use liquid::partials::{InMemorySource, LazyCompiler};
use liquid::value::{Array, Object, Scalar, Value};

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BridgeValue {
    /// A scalar value.
    Scalar(Scalar),
    /// A sequence of `Value`s.
    Array(Array),
    /// A sequence of key/`Value` pairs.
    Object(Object),
    /// Nothing.
    Nil,
    /// No content.
    Empty,
    /// Evaluates to empty string.
    Blank,
}

impl From<Value> for BridgeValue {
    fn from(value: Value) -> BridgeValue {
        match value {
            Value::Scalar(x) => BridgeValue::Scalar(x),
            Value::Array(x) => BridgeValue::Array(x),
            Value::Object(x) => BridgeValue::Object(x),
            Value::Nil => BridgeValue::Nil,
            Value::Empty => BridgeValue::Empty,
            Value::Blank => BridgeValue::Blank,
        }
    }
}

impl Into<Value> for BridgeValue {
    fn into(self) -> Value {
        match self {
            BridgeValue::Scalar(x) => Value::Scalar(x),
            BridgeValue::Array(x) => Value::Array(x),
            BridgeValue::Object(x) => Value::Object(x),
            BridgeValue::Nil => Value::Nil,
            BridgeValue::Empty => Value::Empty,
            BridgeValue::Blank => Value::Blank,
        }
    }
}

impl<'a> FromPyObject<'a> for BridgeValue {
    fn extract(any: &'a PyAny) -> PyResult<BridgeValue> {
        if let Ok(x) = any.extract::<i64>() {
            return Ok(liquid::value::to_value(x).unwrap().into());
        };
        if let Ok(x) = any.extract::<String>() {
            return Ok(liquid::value::to_value(x).unwrap().into());
        };
        if let Ok(x) = any.extract::<f64>() {
            return Ok(liquid::value::to_value(x).unwrap().into());
        };
        if let Ok(x) = any.extract::<bool>() {
            return Ok(liquid::value::to_value(x).unwrap().into());
        };
        if let Ok(x) = any.extract::<Vec<BridgeValue>>() {
            return Ok(liquid::value::to_value(x).unwrap().into());
        };
        if let Ok(x) = any.extract::<HashMap<String, BridgeValue>>() {
            return Ok(liquid::value::to_value(x).unwrap().into());
        };
        if let Ok(x) = any.extract::<HashMap<i64, BridgeValue>>() {
            return Ok(liquid::value::to_value(x).unwrap().into());
        };
        if let Ok(x) = any.extract::<Option<BridgeValue>>() {
            return Ok(liquid::value::to_value(x).unwrap().into());
        };
        Err(PyErr::new::<exceptions::TypeError, _>(
            "Unsupported type for Mercury Oxide",
        ))
    }
}

impl From<&PyAny> for BridgeValue {
    fn from(any: &PyAny) -> BridgeValue {
        any.extract().unwrap()
    }
}

// impl FromPyObject<'source> for Value{
//     fn extract(ob: &'source PyAny) -> PyResult<Self>{
//         Err(PyErr::new::<exceptions::TypeError, _>("invalid_template"))
//     }
// } HashMap<String, PyDict>

#[pyfunction]
fn render(template: String, var_list: &PyDict, partials: Option<HashMap<String, String>>) -> PyResult<String>{
// fn render(
//     template: String,
//     var_list: HashMap<String, String>,
//     partials: Option<HashMap<String, String>>,
// ) -> PyResult<String> {
    let mut partial_store = InMemorySource::new();
    for (name, partial) in partials.unwrap_or(HashMap::new()).iter() {
        partial_store.add(name, partial);
    }

    let template = match liquid::ParserBuilder::with_liquid()
        .partials(LazyCompiler::new(partial_store))
        .build()
        .unwrap()
        .parse(&template)
    {
        Ok(x) => x,
        Err(_e) => return Err(PyErr::new::<exceptions::TypeError, _>("invalid_template")),
    };

    // let mut globals = liquid::value::Object::new();
    // for (k, v) in var_list {
    //     // globals.insert(k.into(), Value::Scalar(v.into()));
    //     // globals.insert(k.into(), v.into());
    // }
    let global_value = dict_to_value(var_list);
    let empty_globals = Object::new();
    let globals = global_value.as_object().unwrap_or(&empty_globals);

    let output = match template.render(globals) {
        Ok(x) => x,
        Err(_e) => return Err(PyErr::new::<exceptions::TypeError, _>("render_error")),
    };

    Ok(output)
}

#[pymodule]
fn mercury_oxide(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(render))?;

    Ok(())
}

fn dict_to_value(t: &PyDict) -> Value {
    let mut map = Object::new();

    for (k, v) in t.iter() {
        let k: Value = BridgeValue::from(k).into();
        let v: Value = BridgeValue::from(v).into();

        map.insert(k.into_scalar().unwrap().into_string().into(), v);
    }

    Value::Object(map)
}

#[test]
fn dict_to_value_test() {
    use liquid::value::Object;
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    let gil = Python::acquire_gil();

    let sub_dict = PyDict::from_sequence(
        gil.python(),
        [
            ("a".to_object(gil.python()), 1.to_object(gil.python())),
            ("b".to_object(gil.python()), 2.to_object(gil.python())),
        ]
        .to_object(gil.python()),
    )
    .unwrap();

    let list = [
        ("a".to_object(gil.python()), 1.to_object(gil.python())),
        ("b".to_object(gil.python()), 2.to_object(gil.python())),
    ]
    .to_object(gil.python());

    let none: Option<String> = None;

    let dict = PyDict::from_sequence(
        gil.python(),
        [
            ("a".to_object(gil.python()), 1.to_object(gil.python())),
            ("b".to_object(gil.python()), 2.to_object(gil.python())),
            ("c".to_object(gil.python()), list.to_object(gil.python())),
            (
                "d".to_object(gil.python()),
                sub_dict.to_object(gil.python()),
            ),
            ("e".to_object(gil.python()), none.to_object(gil.python())),
        ]
        .to_object(gil.python()),
    )
    .unwrap();

    let map = dict_to_value(&dict);

    let mut sub_dict = Object::new();
    sub_dict.insert("a".into(), Value::scalar(1));
    sub_dict.insert("b".into(), Value::scalar(2));

    let list = vec![
        Value::array(vec![Value::scalar("a"), Value::scalar(1)]),
        Value::array(vec![Value::scalar("b"), Value::scalar(2)]),
    ];

    let mut expected_map = Object::new();
    expected_map.insert("a".into(), Value::scalar(1));
    expected_map.insert("b".into(), Value::scalar(2));
    expected_map.insert("c".into(), Value::array(list));
    expected_map.insert("d".into(), Value::Object(sub_dict));
    expected_map.insert("e".into(), Value::Nil);

    let expected_map = Value::Object(expected_map);

    println!("{:?}", map);
    println!("{:?}", expected_map);

    assert_eq!(map, expected_map)
}
