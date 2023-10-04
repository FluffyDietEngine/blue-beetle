# Background

I am working on a huge web scraping project, with a target to scrape thousands of URLs in a day. Below is our tech stack:

**Programming language** - Python 3.8 🐍

**Packages** 
- requests
- beautifulsoup4
- html5lib

At one point, our team started observing a lag in the entire process and started investigating the entire process. Outcome of the investigation was that we are spending 1-2 seconds on average per URL, which is definitely significant 😮‍💨.

# Rustifying the problem 💡

On exploring libraries like  [polars](https://github.com/pola-rs/polars), [pydantic](https://github.com/pydantic/pydantic-core), [ruff](https://github.com/astral-sh/ruff) and much more [libraries](https://github.com/PyO3/pyo3#examples), we got motivated to test the "Rustification of Python". Two ways to do that, either migrate the entire code to rust or write the parser bindings in rust(thanks to maturin and PyO3). The latter seems more viable than the former as it will be a huge technical debt and making the entire team experts in Rust.

# Testing before acting 🤔

But, are we jumping right in before analysing enough? Is there an alternative within the same eco system? There are other parsers in beautifulsoup4 itself like [html.parser](https://docs.python.org/3/library/html.parser.html) and [lxml](https://pypi.org/project/lxml/). What will be the outcome? Is any crate in Rust worthy enough to compete with the html5lib alternatives? 

This repository is trying to find answers for these questions. 

# Tests

**Test case 1** - Finding all <a> tags in a given [html](https://en.wikipedia.org/wiki/Lists_of_books). 

**Results**
- [BeautifulSoup with html5lib](https://github.com/FluffyDietEngine/blue-beetle/blob/main/python-bs4/bs4_html5lib.py) -> 0.05966750695899827 s (~60 ms) 😞
- [BeautifulSoup with html.parser](https://github.com/FluffyDietEngine/blue-beetle/blob/main/python-bs4/bs4_default.py) -> 0.026284454375039786 s (~26 ms) 🙂
- [BeautifulSoup with lxml](https://github.com/FluffyDietEngine/blue-beetle/blob/main/python-bs4/bs4_lxml.py) -> 0.01857255920796888 s (~18 ms) 😃

There is a significant difference between lxml and html5lib, as mentioned in the official [documentation](https://www.crummy.com/software/BeautifulSoup/bs4/doc/#installing-a-parser). But still I wanted to see, how much Rust can improvise this. 
- [Rust with reqwest and tl](https://github.com/FluffyDietEngine/blue-beetle/blob/main/rust-tl/src/main.rs) -> 7 ms! 😎

Will be adding more complicated test cases like finding element based on string, find a tag and modify the string inside, modify the DOM much more

# Where do I need help? 🤞
- Code reviews from Rustaceans are important whether my rust codes are valid enough for test cases.
- More complicated test cases in DOM parsing.
- Feel free to point out if I have missed any alternatives in Python only (No Xpath please!).
