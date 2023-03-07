use surrealdb::{sql::{self, Value, Object}, Response};

macro_rules! b_tree_map {
    (
        $(($name:expr, $value:expr)),*
        $(,)? // for trailing commas
    ) => {
        [
            $(
                ($name.into(), $value.into()),
            )*
        ].into()
    };
}

pub(crate) use b_tree_map;

pub fn query_result_into_objects(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object, String>>, String> {
    let r = responses.into_iter().next().map(|response| response.result);

    let Some(Ok(Value::Array(sql::Array(values)))) = r else {
        return Err("NO.".to_string())
    };

    Ok(values.into_iter().map(|value| match value {
        Value::Object(value) => Ok(value),
        _ => Err("lol".to_string())
    }))
}

pub fn extract_single_object(mut objects: impl Iterator<Item = Result<Object, String>>) -> Result<Object, String> {
    match objects.next() {
        Some(object) => {
            object.map_err(|error| error.to_string())
        },

        None => Err("not found".to_string())
    }
}