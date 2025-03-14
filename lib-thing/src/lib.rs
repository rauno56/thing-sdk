mod error;

pub use self::error::{Error, Result};

use polars::{prelude::NamedFrom, series::Series};
use serde::Deserialize;

/* Probably a bad practice re-exporting dep types? */
pub mod pl {
	pub use polars::error::PolarsResult;
	pub use polars::series::Series;
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	a: u32,
	b: u32,
}

impl Config {
	pub fn new(a: u32, b: u32) -> Self {
		Config { a, b }
	}
}

#[derive(Debug, Clone)]
pub struct Model {
	pub a: u32,
	pub b: u32,
	pub c: u32,
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

	pub fn from_json(config: &str) -> Result<Self> {
		let config: Config = serde_json::from_str(config)?;
		let series = Series::new("values".into(), &[config.a, config.b]);
		Ok(Model {
			a: config.a,
			b: config.b,
			c: config.a + config.b,
			series,
		})
	}

	pub fn calc(&self, y: u32) -> u32 {
		self.a + self.c + y
	}

	pub fn calc_series(&self, y: u32) -> Series {
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
