
use godot::engine::Json;
use godot::prelude::*;
use godot::builtin::GodotString;
struct GodotRustYAML;
#[gdextension]
unsafe impl ExtensionLibrary for GodotRustYAML {}

#[derive(GodotClass)]
struct YAML {
}
#[godot_api]
impl YAML {
    #[func]
    fn parse(str: GodotString) -> Variant {
        let input: String = str.to_string();
        let yaml_parse_result: Result<serde_json::Value, serde_yaml::Error> = serde_yaml::from_str(&input.replace("    ", "").replace("	", " "));
        let json_value = match yaml_parse_result {
            Ok(parsed) => parsed,
            Err(error) => return Variant::from(format!("Problem parsing the YAML: {:?}", error)),
        };
        let json_string = serde_json::to_string(&json_value).unwrap();
        return Json::parse_string(GodotString::from(json_string));
    }

    #[func]
    fn to_string(input: Dictionary) -> GodotString {
        let json_string = Json::stringify(Variant::from(input));
        let input: String = json_string.to_string();
        let json_parse_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&input.replace("    ", "").replace("	", " "));
        let yaml_value = match json_parse_result {
            Ok(parsed) => parsed,
            Err(error) => return GodotString::from(format!("Problem parsing the dictionary: {:?}", error)),
        };
        let yaml_string = serde_yaml::to_string(&yaml_value).unwrap();
        return GodotString::from(yaml_string);
    }
}
