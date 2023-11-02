import timeit

def main():
    time_taken = timeit.timeit(
        stmt="""
anchors = extract_links(body)
        """,
        setup="""
from rust_parser import extract_links
from requests import get
url = 'https://en.wikipedia.org/wiki/Lists_of_books'
resp = get(url)
body = resp.content.decode()
        """,
        number=1000
    )
    print(f"Average time: {time_taken / 1000} s")

if __name__ == "__main__":
    main()
