use pyo3::prelude::*;
use gytmir_lib::sync as rust_sync;

#[pyfunction]
fn sync(
    repo_dir: &str,
    remote: &str,
    key_file: &str,
    branch: &str,
) -> PyResult<()> {
    rust_sync(repo_dir, remote, key_file, branch);
    Ok(())
}


#[pymodule]
fn pygytmir(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sync, m)?)?;
    Ok(())
}
