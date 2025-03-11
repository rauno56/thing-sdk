mod error;

pub use self::error::{Error, Result};

use polars::prelude::*;
use polars::series::Series;
use serde::Deserialize;

/* Probably a bad practice re-exporting dep types? */
pub mod pl {
	pub use polars::prelude::PolarsResult;
	pub use polars::series::Series;
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	a: i32,
	b: i32,
}

impl Config {
	pub fn new(a: i32, b: i32) -> Self {
		Config { a, b }
	}
}

#[derive(Debug, Clone)]
pub struct Model {
	pub a: i32,
	pub b: i32,
	pub c: i32,
	pub series: Series,
}

impl Model {
	pub fn new(config: Config) -> Self {
		let series = Series::new("values".into(), &[config.a, config.b]);
		Model {
			a: config.a,
			b: config.b,
			c: config.a + config.b,
			series,
		}
	}

	pub fn from_json(config: &String) -> Result<Self> {
		let config: Config = serde_json::from_str(config)?;
		let series = Series::new("values".into(), &[config.a, config.b]);
		Ok(Model {
			a: config.a,
			b: config.b,
			c: config.a + config.b,
			series,
		})
	}

	pub fn calc(&self, y: i32) -> i32 {
		self.a + self.c + y
	}

	pub fn calc_series(&self, y: i32) -> Series {
		self.series.clone() + y
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_create() {
		Model::new(Config::new(5, 6));
	}

	#[test]
	fn test_calc() {
		let m = Model::new(Config::new(5, 6));

		assert_eq!(m.calc(4), 20);
	}

	#[test]
	fn test_calc_series() {
		let m = Model::new(Config::new(5, 6));

		assert_eq!(m.calc_series(4), Series::new("values".into(), &[9, 10]));
	}
}
