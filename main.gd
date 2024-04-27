extends Node

func _ready():
	var my_dictionary:Dictionary = {
			"Array":[1,2,3],
			"Dict":{"key":"value"},
			"Hello!":2,
		}

	var my_yaml:String = YAML.to_string(my_dictionary)
	var parsed_yaml:Dictionary = YAML.parse(my_yaml)
	print(my_yaml)
	print(my_dictionary)
	print(parsed_yaml)
	assert(my_dictionary == parsed_yaml)
