#![no_main]

use extism_pdk::*;
use lib_thing::{self as thing, Config};

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
	Ok(format!("Hello, {}!\n", name))
}

#[plugin_fn]
pub fn add_one(x: u64) -> FnResult<u64> {
	Ok(x + 1)
}

// impl Model {
// 	#[new]
// 	fn __new__(api_key: String, config: String) -> PyResult<Self> {
// 		// TODO: Make this meaningful
// 		let model = .into_pyresult()?;
// 		Ok(Model { model })
// 	}

// 	fn calc(&self, y: i32) -> i32 {
// 		self.model.calc(y)
// 	}

// 	fn calc_series(&self, y: i32) -> PyResult<Vec<i32>> {
// 		let series = self.model.calc_series(y);
// 		let elements: Vec<i32> = series.i32().into_pyresult()?.into_no_null_iter().collect();
// 		Ok(elements)
// 	}
// }

// Json(_config): Json<thing::Config>

#[derive(serde::Deserialize, FromBytes, Debug)]
#[encoding(Json)]
struct Payload {
	x: u32,
}

#[derive(serde::Serialize, ToBytes)]
#[encoding(Json)]
struct Sum {
	sum: u32,
}

fn config_get(key: impl AsRef<str>) -> FnResult<String> {
	let key = key.as_ref();
	config::get(key).map_or_else(
		|_| {
			Err(WithReturnCode::new(
				Error::msg(format!("failed to load '{:?}'", key)),
				0,
			))
		},
		|value| {
			value.map_or_else(
				|| {
					Err(WithReturnCode::new(
						Error::msg(format!("'{:?}' is unset in config", key)),
						0,
					))
				},
				|value| Ok(value),
			)
		},
	)
}

#[plugin_fn]
pub fn calc(payload: Payload) -> FnResult<Sum> {
	let api_key = config_get("api_key")?;
	let config = config_get("config")?;
	if api_key != "supersecret" {
		return Err(WithReturnCode::new(
			Error::msg(format!("invalid api key: '{api_key}'")),
			0,
		));
	}
	let model = thing::Model::from_json(&config).unwrap();
	println!("Model: {model:?}");
	println!("hello from plugin!");
	// println!("payload: {payload:?}");
	Ok(Sum {
		sum: model.calc(payload.x),
	})
}

#[plugin_fn]
pub fn calc_series(payload: Payload) -> FnResult<Sum> {
	let api_key = config_get("api_key")?;
	let config = config_get("config")?;
	if api_key != "supersecret" {
		return Err(WithReturnCode::new(
			Error::msg(format!("invalid api key: '{api_key}'")),
			0,
		));
	}
	let model = thing::Model::from_json(&config).unwrap();
	println!("Model: {model:?}");
	println!("hello from plugin!");
	let series = model.calc_series(payload.x);
	let elements: Vec<u32> = series.u32()?.into_no_null_iter().collect();
	Ok(Sum {
		sum: (6 as u32) + elements.iter().sum::<u32>(),
	})
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn it_works() {
// 		// let result = add_one(2, 2);
// 		assert_eq!(result, 4);
// 	}
// }
