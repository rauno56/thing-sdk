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
		// TODO: Make this meaningful
		if api_key != "supersecret" {
			return Err(PyValueError::new_err("invalid api key"));
		}
		let model = thing::Model::from_json(&config).into_pyresult()?;
		Ok(Model { model })
	}

	fn calc(&self, y: i32) -> i32 {
		self.model.calc(y)
	}

	fn calc_series(&self, y: i32) -> PyResult<Vec<i32>> {
		let series = self.model.calc_series(y);
		let elements: Vec<i32> = series.i32().into_pyresult()?.into_no_null_iter().collect();
		Ok(elements)
	}
}

#[pyfunction]
fn bare_calc(_a: usize, _b: usize) -> PyResult<String> {
	// Simulate error
	Err(PyValueError::new_err("oops!"))
}

/// Setup the exported python module
#[pymodule]
fn thing_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<Model>()?;
	m.add_function(wrap_pyfunction!(bare_calc, m)?)?;
	Ok(())
}
