from requests import get
from rust_parser import extract_links
from time import perf_counter

def main(url):
    response = get(url)
    start = perf_counter()
    for i in range(1000):
        link = extract_links(response.text)
        print(len(link))
    print(perf_counter() - start)


if __name__ == "__main__":
    link = "https://en.wikipedia.org/wiki/Lists_of_books"
    main(link)
