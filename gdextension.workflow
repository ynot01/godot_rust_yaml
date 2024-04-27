[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1

[libraries]
linux.debug.x86_64 = "res://addons/godot_rust_yaml/linux/libgodot_rust_yaml_debug.so"
linux.release.x86_64 = "res://addons/godot_rust_yaml/linux/libgodot_rust_yaml.so"
windows.debug.x86_64 = "res://addons/godot_rust_yaml/win64/godot_rust_yaml_debug.dll"
windows.release.x86_64 = "res://addons/godot_rust_yaml/win64/godot_rust_yaml.dll"
macos.debug = "res://addons/godot_rust_yaml/macos/libgodot_rust_yaml_debug.dylib"
macos.release = "res://addons/godot_rust_yaml/macos/libgodot_rust_yaml.dylib"
macos.debug.arm64 = "res://addons/godot_rust_yaml/macos_arm/libgodot_rust_yaml_debug.dylib"
macos.release.arm64 = "res://addons/godot_rust_yaml/macos_arm/libgodot_rust_yaml.dylib"
