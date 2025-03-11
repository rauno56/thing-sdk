import thing_py
import json

print(dir(thing_py.thing_py))

try:
	print(thing_py.bare_calc(12,21))
except ValueError as e:
	print(f"Bare_calc errored: {e}")

try:
	config = { "a": 1, "b": 3 }
	mymod = thing_py.Model("supersecret", json.dumps(config))
	print("Single", mymod.calc(2))
	print("Series", mymod.calc_series(2))
except Exception as e:
	print(f"Model errored: {e}")
