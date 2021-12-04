# Fetches the input text file from https://adventofcode.com/
# for a given year/day

import os
import sys
import requests
from dotenv import load_dotenv

URL = "https://adventofcode.com/{year}/day/{day}/input"

if __name__ == "__main__":
    load_dotenv()

    script_name = sys.argv[0]
    try:
        year, day, *_ = sys.argv[1:]
    except ValueError:
        print(f"Usage: {script_name} <year> <day>", file=sys.stderr)
        exit(1)

    response = requests.get(
        URL.format(year=year, day=day),
        cookies={"session": os.getenv("AOC_SESSION")},
    )

    if response.ok:
        print(response.text)
    else:
        print("Error fetching input", file=sys.stderr)
