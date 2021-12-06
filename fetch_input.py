# Fetches the input text file from https://adventofcode.com/
# for a given year/day

import os
import sys
import argparse
import requests
from bs4 import BeautifulSoup
from dotenv import load_dotenv

INPUT_URL = "https://adventofcode.com/{year}/day/{day}/input"
EXAMPLE_URL = "https://adventofcode.com/{year}/day/{day}"
DEFAULT_OUTPUT = "inputs/day{day}{suffix}.txt"

parser = argparse.ArgumentParser(description="Fetches input from AoC")
parser.add_argument("-y", "--year", required=True, type=int)
parser.add_argument("-d", "--day", required=True, type=int)
parser.add_argument("-i", "--input", action="store_true",
                    help="fetches the actual input")
parser.add_argument("-e", "--example", action="store_true",
                    help="fetches the example instead of actual input")
parser.add_argument("-o", "--output", type=argparse.FileType("w"),
                    help="specifies the output file")


def write_output(output: str, args: argparse.Namespace, suffix: str = '') -> None:
    if args.output:
        args.output.write(output)
    else:
        filename = DEFAULT_OUTPUT.format(suffix=suffix, **args.__dict__)

        with open(filename, "w", encoding="utf-8") as f:
            f.write(output)


if __name__ == "__main__":
    load_dotenv()
    args = parser.parse_args()

    if args.input:
        response = requests.get(
            INPUT_URL.format(**args.__dict__),
            cookies={"session": os.getenv("AOC_SESSION")},
        )

        if not response.ok:
            print("Error fetching input", file=sys.stderr)
            exit(1)

        write_output(response.text, args)

    if args.example:
        response = requests.get(EXAMPLE_URL.format(**args.__dict__))

        if not response.ok:
            print("Error fetching example", file=sys.stderr)
            exit(1)

        bs = BeautifulSoup(response.text, features="html.parser")
        # The example is usually contained within the first <pre> element
        content = bs.select_one("pre > code").text

        write_output(content + '\n', args, '-example')
