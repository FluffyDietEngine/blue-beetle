use pyo3::prelude::*;

use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;


#[pyfunction]
fn extract_links(body: &str) -> HashSet<String> {
    let document = Document::from(body);
    let links = document.find(Name("a"))
                        .filter_map(|n| n.attr("href"))
                        .map(String::from)
                        .collect::<HashSet<String>>();
    links
}


/// A Python module implemented in Rust.
#[pymodule]
fn rust_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_links, m)?)?;
    Ok(())
}

