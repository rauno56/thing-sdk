use lib_thing::{self as thing, pl::PolarsResult};
use pyo3::{exceptions::PyValueError, prelude::*};

trait IntoPyResult<T> {
	fn into_pyresult(self) -> PyResult<T>;
}

impl<T> IntoPyResult<T> for thing::Result<T> {
	fn into_pyresult(self) -> PyResult<T> {
		match self {
			Self::Ok(val) => Ok(val),
			Self::Err(err) => Err(PyValueError::new_err(format!("{}", err))),
		}
	}
}

impl<T> IntoPyResult<T> for PolarsResult<T> {
	fn into_pyresult(self) -> PyResult<T> {
		match self {
			Self::Ok(val) => Ok(val),
			Self::Err(err) => Err(PyValueError::new_err(format!("{}", err))),
		}
	}
}

#[pyclass(name = "Model")]
struct Model {
	model: thing::Model,
}

#[pymethods]
impl Model {
	#[new]
	fn __new__(api_key: String, config: String) -> PyResult<Self> {
		// TODO: take api key from WASM config
		// TODO: Make this meaningful
		if api_key != "supersecret" {
			return Err(PyValueError::new_err("invalid api key"));
		}
		let model = thing::Model::from_json(&config).into_pyresult()?;
		Ok(Model { model })
	}

	fn calc(&self, y: u32) -> u32 {
		self.model.calc(y)
	}

	fn calc_series(&self, y: u32) -> PyResult<Vec<u32>> {
		let series = self.model.calc_series(y);
		let elements: Vec<u32> = series.u32().into_pyresult()?.into_no_null_iter().collect();
		Ok(elements)
	}
}

#[pyfunction]
fn throw_oops(_a: usize, _b: usize) -> PyResult<String> {
	// Simulate error
	Err(PyValueError::new_err("oops!"))
}

/// Setup the exported python module
#[pymodule]
fn thing_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<Model>()?;
	m.add_function(wrap_pyfunction!(throw_oops, m)?)?;
	Ok(())
}
