//! Prepare the pinned Ubicloud spec for Progenitor's response parser.
//!
//! Progenitor 0.15.1 cannot represent a success response plus a `default`
//! response (its success/error response type inference both include
//! `default`). The Ubicloud API uses a shared Error schema for default
//! responses, so express that response as the OpenAPI status ranges it
//! actually represents. This preserves the vendored source document and keeps
//! the generated HTTP behavior typed.

use serde_yaml::{Mapping, Value};

const METHODS: [&str; 8] = [
    "get", "post", "put", "patch", "delete", "options", "head", "trace",
];

/// Parse `source`, rewrite `default` responses and serialize the result.
pub fn prepare(source: &str) -> Result<String, String> {
    let mut document: Value =
        serde_yaml::from_str(source).map_err(|error| format!("cannot parse spec: {error}"))?;
    rewrite_default_responses(&mut document)?;
    lift_vm_response_schema(&mut document)?;
    serde_yaml::to_string(&document).map_err(|error| format!("cannot serialize spec: {error}"))
}

/// Flatten the VM response's inline `allOf` schema into a named object.
/// Progenitor 0.15.1 otherwise emits empty per-operation response enums for
/// this response, even though the response is a concrete VM object.
fn lift_vm_response_schema(document: &mut Value) -> Result<(), String> {
    let has_vm_response = document
        .get("components")
        .and_then(|value| value.get("responses"))
        .and_then(|value| value.get("Vm"))
        .is_some();
    if !has_vm_response {
        return Ok(());
    }
    let base_vm = document
        .get("components")
        .and_then(|value| value.get("schemas"))
        .and_then(|value| value.get("Vm"))
        .cloned()
        .ok_or("spec has no `components.schemas.Vm` schema")?;
    let components = document
        .get_mut("components")
        .and_then(Value::as_mapping_mut)
        .ok_or("spec has no `components` mapping")?;
    let responses = components
        .get_mut("responses")
        .and_then(Value::as_mapping_mut)
        .ok_or("spec has no `components.responses` mapping")?;
    let vm_response = responses
        .get_mut("Vm")
        .ok_or("spec has no `components.responses.Vm` response")?;
    let schema = vm_response
        .get_mut("content")
        .and_then(|content| content.get_mut("application/json"))
        .and_then(|json| json.get_mut("schema"))
        .ok_or("VM response has no application/json schema")?;

    let vm_schema = flatten_vm_response(&base_vm, schema)?;
    *schema = serde_yaml::from_str("$ref: '#/components/schemas/VmResponse'")
        .expect("static YAML reference is valid");
    let schemas = components
        .entry(Value::from("schemas"))
        .or_insert_with(|| Value::Mapping(Mapping::new()))
        .as_mapping_mut()
        .ok_or("spec `components.schemas` is not a mapping")?;
    schemas.insert(Value::from("VmResponse"), vm_schema);
    Ok(())
}

fn flatten_vm_response(base_vm: &Value, schema: &Value) -> Result<Value, String> {
    let all_of = schema
        .get("allOf")
        .and_then(Value::as_sequence)
        .ok_or("VM response schema has no `allOf` sequence")?;
    let mut properties = Mapping::new();
    let mut required = Vec::new();
    for part in all_of {
        if let Some(reference) = part.get("$ref").and_then(Value::as_str) {
            if reference != "#/components/schemas/Vm" {
                return Err(format!(
                    "unexpected VM response schema reference {reference:?}"
                ));
            }
            copy_object_fields(base_vm, &mut properties, &mut required)?;
        } else {
            copy_object_fields(part, &mut properties, &mut required)?;
        }
    }

    let mut flattened = Mapping::new();
    flattened.insert(Value::from("type"), Value::from("object"));
    flattened.insert(Value::from("additionalProperties"), Value::Bool(false));
    flattened.insert(Value::from("properties"), Value::Mapping(properties));
    flattened.insert(Value::from("required"), Value::Sequence(required));
    Ok(Value::Mapping(flattened))
}

fn copy_object_fields(
    object: &Value,
    properties: &mut Mapping,
    required: &mut Vec<Value>,
) -> Result<(), String> {
    let object = object
        .as_mapping()
        .ok_or("VM response `allOf` member is not an object")?;
    if let Some(Value::Mapping(fields)) = object.get(Value::from("properties")) {
        properties.extend(fields.clone());
    }
    if let Some(Value::Sequence(fields)) = object.get(Value::from("required")) {
        for field in fields {
            if !required.contains(field) {
                required.push(field.clone());
            }
        }
    }
    Ok(())
}

/// Replace every operation's `default` response with `4XX` and `5XX` copies.
pub fn rewrite_default_responses(document: &mut Value) -> Result<(), String> {
    let paths = document
        .get_mut("paths")
        .and_then(Value::as_mapping_mut)
        .ok_or("spec has no `paths` mapping")?;
    for path_item in paths.values_mut() {
        let Some(path_item) = path_item.as_mapping_mut() else {
            continue;
        };
        for (method, operation) in path_item.iter_mut() {
            if !method.as_str().is_some_and(|name| METHODS.contains(&name)) {
                continue;
            }
            if let Some(responses) = operation
                .get_mut("responses")
                .and_then(Value::as_mapping_mut)
            {
                split_default(responses);
            }
        }
    }
    Ok(())
}

fn split_default(responses: &mut Mapping) {
    // `shift_remove` keeps the remaining responses in document order.
    if let Some(response) = responses.shift_remove("default") {
        responses.insert(Value::from("4XX"), response.clone());
        responses.insert(Value::from("5XX"), response);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: &str = r#"
openapi: 3.0.3
paths:
  /project:
    parameters:
      - name: ignored
        in: query
    get:
      responses:
        default:
          $ref: '#/components/responses/Error'
        200:
          description: ok
        '404':
          description: missing
    post:
      responses:
        '201':
          description: created
"#;

    fn responses(document: &Value, method: &str) -> Vec<String> {
        document["paths"]["/project"][method]["responses"]
            .as_mapping()
            .unwrap()
            .keys()
            .map(|key| match key {
                Value::String(text) => text.clone(),
                other => serde_yaml::to_string(other).unwrap().trim().to_owned(),
            })
            .collect()
    }

    #[test]
    fn default_becomes_status_ranges_after_existing_responses() {
        let prepared: Value = serde_yaml::from_str(&prepare(SPEC).unwrap()).unwrap();
        assert_eq!(responses(&prepared, "get"), ["200", "404", "4XX", "5XX"]);
        let error = &prepared["paths"]["/project"]["get"]["responses"]["4XX"]["$ref"];
        assert_eq!(error.as_str(), Some("#/components/responses/Error"));
        assert_eq!(
            prepared["paths"]["/project"]["get"]["responses"]["4XX"],
            prepared["paths"]["/project"]["get"]["responses"]["5XX"]
        );
    }

    #[test]
    fn operations_without_default_and_non_operations_are_untouched() {
        let original: Value = serde_yaml::from_str(SPEC).unwrap();
        let prepared: Value = serde_yaml::from_str(&prepare(SPEC).unwrap()).unwrap();
        assert_eq!(responses(&prepared, "post"), ["201"]);
        assert_eq!(
            prepared["paths"]["/project"]["parameters"],
            original["paths"]["/project"]["parameters"]
        );
    }

    #[test]
    fn missing_paths_is_an_error() {
        assert!(prepare("openapi: 3.0.3\n").unwrap_err().contains("paths"));
    }

    #[test]
    fn vm_response_schema_is_lifted_to_a_named_component() {
        let spec = r##"
openapi: 3.0.3
components:
  schemas:
    Vm:
      type: object
      properties:
        id:
          type: string
      required:
        - id
  responses:
    Vm:
      content:
        application/json:
          schema:
            allOf:
              - $ref: '#/components/schemas/Vm'
              - properties:
                  unix_user:
                    type: string
                required:
                  - firewalls
                  - private_ipv4
                  - private_ipv6
                  - subnet
                  - unix_user
paths: {}
"##;
        let prepared: Value = serde_yaml::from_str(&prepare(spec).unwrap()).unwrap();
        assert_eq!(
            prepared["components"]["responses"]["Vm"]["content"]["application/json"]["schema"]["$ref"],
            "#/components/schemas/VmResponse"
        );
        assert_eq!(
            prepared["components"]["schemas"]["VmResponse"]["type"],
            "object"
        );
        assert_eq!(
            prepared["components"]["schemas"]["VmResponse"]["properties"]["unix_user"]["type"],
            "string"
        );
        assert_eq!(
            prepared["components"]["schemas"]["VmResponse"]["properties"]["id"]["type"],
            "string"
        );
        let required: Vec<_> = prepared["components"]["schemas"]["VmResponse"]["required"]
            .as_sequence()
            .unwrap()
            .iter()
            .map(Value::as_str)
            .collect();
        assert_eq!(
            required,
            [
                Some("id"),
                Some("firewalls"),
                Some("private_ipv4"),
                Some("private_ipv6"),
                Some("subnet"),
                Some("unix_user")
            ]
        );
    }
}
