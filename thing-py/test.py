import thing_py
import json

try:
	print(thing_py.throw_oops(12,21))
except ValueError as e:
	print(f"throw_oops errored: {e}")

try:
	config = { "a": 1, "b": 3 }
	mymod = thing_py.Model("supersecret", json.dumps(config))
	print("Single", mymod.calc(2))
	print("Series", mymod.calc_series(2))
except Exception as e:
	print(f"Model errored: {e}")
