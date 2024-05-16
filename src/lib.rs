pub mod prelude;
mod psk;
pub use psk::PskModulation;

mod traits;

#[cfg(feature = "radio-python")]
use pyo3::prelude::*;

#[cfg(feature = "radio-python")]
#[pymodule]
pub fn radio_modulation(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PskModulation>()?;
    Ok(())
}

#[cfg(test)]
mod tests {}
