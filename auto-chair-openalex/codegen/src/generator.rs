use serde_json::Value;

macro_rules! w {
    ($out:expr) => { $out.push('\n') };
    ($out:expr, $($arg:tt)+) => {
        $out.push_str(&format!($($arg)+));
        $out.push('\n');
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidSchemaError {
    TitleMustBeString,
    PropertiesMustBeObject,
    RequiredMustBeArray,
    RequiredItemMustBeString { index: usize },
    NotImplemented,
}

pub fn generate_entity(schema: &Value) -> Result<String, InvalidSchemaError> {
    let entity_name = schema
        .get("title")
        .and_then(Value::as_str)
        .ok_or(InvalidSchemaError::TitleMustBeString)?;

    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .ok_or(InvalidSchemaError::PropertiesMustBeObject)?;
    
    let required: Vec<String> = match schema.get("required") {
        Some(Value::Array(arr)) => arr
            .iter()
            .enumerate()
            .map(|(index, value)| {
                value
                    .as_str()
                    .map(str::to_string)
                    .ok_or(InvalidSchemaError::RequiredItemMustBeString { index })
            })
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => return Err(InvalidSchemaError::RequiredMustBeArray),
        None => Vec::new(),
    };

    let mut out = String::with_capacity(64 * 1024);
    
    emit_header(&mut out);

    w!(out, "#[derive(Debug, Clone, Serialize, Deserialize)]");
    w!(out, "pub struct {entity_name} {{");
    for (field_name, field_schema) in properties {
        let mut field_type = get_rust_type(field_schema)?;
        
        if !required.contains(field_name) {
            w!(out, "  #[serde(skip_serializing_if = \"Option::is_none\")]");
            field_type = format!("Option<{}>", field_type);
        }

        w!(out, "  pub {}: {},", field_name, field_type);
    }
    w!(out, "}}");

    Ok(out)
    
}

fn emit_header(out: &mut String) {
    w!(out, "use chrono::{{DateTime, Utc}};");
    w!(out, "use reqwest::Url;");
    w!(out);
}

fn get_rust_type(field_schema: &Value) -> Result<String, InvalidSchemaError> {
    match field_schema.get("type"){
        Some(Value::String(type_name)) => return Ok(primitive_type(type_name, field_schema.get("format"))),
        _ => Err(InvalidSchemaError::NotImplemented)
    }
}

fn primitive_type(type_name: &str, format: Option<&Value>) -> String {
    match type_name {
        "string" => {
            if let Some(Value::String(format)) = format {
                match format.as_str() {
                    "date-time" => return "chrono::DateTime<chrono::Utc>".to_string(),
                    "uri" => return "reqwest::Url".to_string(),
                    _ => {}
                }
            }
            "str".to_string()
        }
        "integer" => "i64".to_string(),
        "number" => "f64".to_string(),
        "boolean" => "bool".to_string(),
        _ => panic!("unsupported primitive type `{type_name}`"),
    }
}
