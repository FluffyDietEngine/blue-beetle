# Background

I am working on a huge web scraping project, with a target to scrape thousands of URLs in a day. Below is our tech stack:

**Programming language** - Python 3.8

**Packages** 
- requests
- beautifulsoup4
- html5lib

At one point, our team started observing a lag in the entire process and started investigating the entire process. Outcome of the investigation was that we are spending 2-3 seconds on average per URL, which is definitely significant.

# Rustifying the problem

On exploring libraries like  [polars](https://github.com/pola-rs/polars), [pydantic](https://github.com/pydantic/pydantic-core), [ruff](https://github.com/astral-sh/ruff) and much more [libraries](https://github.com/PyO3/pyo3#examples), we got motivated to test the "Rustification of Python". Two ways to do that, either migrate the entire code to rust or write the parser bindings in rust(thanks to maturin and PyO3). The latter seems more viable than the former as it will be a huge technical debt and making the entire team experts in Rust.

# Testing before acting

But, are we jumping right in before analysing enough? Is there an alternative within the same eco system? There are other parsers in beautifulsoup4 itself like [html.parser](https://docs.python.org/3/library/html.parser.html) and [lxml](https://pypi.org/project/lxml/). What will be the outcome? Is any crate in Rust worthy enough to compete with the html5lib alternatives? 

This repository is trying to find answers for these questions. 
