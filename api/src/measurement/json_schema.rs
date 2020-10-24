// Reexporting this so callers don't have to add schemars to their Cargo.toml. (This is the object
// schemars::schema_for!() returns.
//
pub use schemars::{schema::RootSchema, schema_for};

use super::Measurement;
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Metadata, Schema, SchemaObject},
    JsonSchema,
};

const DESCRIPTION: &'static str =
    "A measurement consists of a value and a unit, where the unit must be a valid UCUM unit string";

/// This gets you:
///
/// ```
/// use wise_units::{Measurement, measurement::json_schema::schema_for};
///
/// let schema = schema_for!(Measurement);
/// let json = serde_json::to_value(&schema).unwrap();
/// let expected = serde_json::json!({
///     "$schema": "http://json-schema.org/draft-07/schema#",
///     "description": "A measurement consists of a value and a unit, where the unit must be a valid UCUM unit string",
///     "title": "Measurement",
///     "type": "object",
///     "format": "measurement",
///     "required": [
///         "unit",
///         "value"
///     ],
///     "properties": {
///         "unit": {
///             "type": "string"
///         },
///         "value": {
///             "type": "number",
///             "format": "double"
///         }
///     }
/// });
///
/// pretty_assertions::assert_eq!(json, expected);
/// ```
///
impl JsonSchema for Measurement {
    fn schema_name() -> String {
        "Measurement".to_owned()
    }

    #[allow(box_pointers)]
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema = SchemaObject::default();
        schema.instance_type = Some(InstanceType::Object.into());

        let obj = schema.object();

        // Add the required (non-Option) types
        let _ = obj.required.insert("value".to_owned());
        let _ = obj.required.insert("unit".to_owned());

        // Add properties for all fields
        let _ = obj
            .properties
            .insert("value".to_owned(), <f64>::json_schema(gen));
        let _ = obj
            .properties
            .insert("unit".to_owned(), <String>::json_schema(gen));

        schema.metadata = {
            let mut meta = Metadata::default();
            meta.description = Some(DESCRIPTION.to_string());
            Some(Box::new(meta))
        };

        schema.format = Some("measurement".to_string());

        Schema::Object(schema)
    }
}
