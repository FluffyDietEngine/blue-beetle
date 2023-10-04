import timeit

def main():
    time_taken = timeit.timeit(
        stmt="""
soup = BeautifulSoup(resp.content, 'html5lib')
anchors = soup.find_all('a', href=True)
        """,
        setup="""
from bs4 import BeautifulSoup
from requests import get
url = 'https://en.wikipedia.org/wiki/Lists_of_books'
resp = get(url)
        """,
        number=1000
    )
    print(f"Average time: {time_taken / 1000} s")

if __name__ == "__main__":
    main()