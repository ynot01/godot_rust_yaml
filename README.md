# godot_rust_yaml

### YAML GDExtension for [Godot 4.x](https://github.com/godotengine/godot)

Does not support object serialization. Objects will be parsed as a string "<ClassName#InstanceID>"

- Crates used:
  - [godot-rust/gdext](https://github.com/godot-rust/gdext)
  - [serde](https://github.com/serde-rs/serde)
  - [serde-yaml](https://github.com/dtolnay/serde-yaml)
  - [serde-json](https://github.com/serde-rs/json)

# Downloads

[Downloads are at releases](https://github.com/ynot01/godot_rust_yaml/releases)

# Usage

**Currently only pre-built windows builds are available, other platforms see [Compiling](#Compiling)**

Head to releases and place the `godot_rust_yaml` folder into your addons folder

## YAML.parse(str: String)

Returns a dictionary of parsed YAML or a string if an error had occured

```gdscript
var parsed_yaml = YAML.parse("foo: bar") # {"foo":"bar"}
if typeof(parsed_yaml) != TYPE_DICTIONARY:
  push_error(parsed_yaml)
  return
# do stuff with dictionary
```

## YAML.to_string(input: Dictionary)

Returns a string of YAML or an error if it had occured

```gdscript
print(YAML.to_string({"foo":"bar"})) # foo: bar
```

# Compiling

Requires [rust](https://www.rust-lang.org/tools/install)

1. Fork this repository
2. Navigate to `/addons/godot_rust_yaml/rust/`
3. To find a version of Godot 4, the library expects either an executable of name `godot4` in the PATH, or an environment variable `GODOT4_BIN` containing the path to the executable (including filename)
4. Run `cargo build` and/or `cargo build --release` for debug/release builds respectively
5. The project should now be runnable in the [godot editor](https://godotengine.org/download/) for debugging, your binary files will be in `res://addons/godot_rust_yaml/rust/target/`
