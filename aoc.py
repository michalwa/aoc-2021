# Fetches the input text file from https://adventofcode.com/
# for a given year/day

import os
import sys
import re
import argparse
import requests
import colorama
from colorama import Fore
from shutil import copyfile
from bs4 import BeautifulSoup
from dotenv import load_dotenv

INPUT_URL = "https://adventofcode.com/{year}/day/{day}/input"
EXAMPLE_URL = "https://adventofcode.com/{year}/day/{day}"
DEFAULT_OUTPUT = "inputs/day{day}{suffix}.txt"

TEMPLATES = {"src/day_template.rs": "src/day_{day}.rs"}
INSERT_FILES = {
    "Cargo.toml": {
        "# Insert new day feature here": ["day_{day} = []"],
    },
    "src/main.rs": {
        "// Include new day mod here": ['#[cfg(feature = "day_{day}")]', "mod day_{day};"],
        "// Call new day here": ['#[cfg(feature = "day_{day}")]', "day_{day}::main(&input[..])?;"],
    },
}

parser = argparse.ArgumentParser(description="Bootstraps an AoC solution")
parser.add_argument("-v", "--verbose", action="store_true",
                    help="logs additional info")
parser.add_argument("-y", "--year", required=True, type=int,
                    help="specify the AoC edition")
parser.add_argument("-d", "--day", required=True, type=int,
                    help="specify the day")
parser.add_argument("-g", "--codegen", action="store_true",
                    help="generate code for the day")
parser.add_argument("-i", "--input", action="store_true",
                    help="fetch input")
parser.add_argument("-e", "--example", action="store_true",
                    help="fetch input example")
parser.add_argument("-o", "--output", type=argparse.FileType("w"),
                    help="specify the output file")


def write_output(output: str, args: argparse.Namespace, suffix: str = '') -> None:
    if args.output:
        args.output.write(output)
    else:
        filename = DEFAULT_OUTPUT.format(suffix=suffix, **args.__dict__)

        with open(filename, "w", encoding="utf-8") as f:
            f.write(output)


def log(message: str, *, kind: str = "", **print_args):
    print({
        "success": f"{Fore.GREEN}✓{Fore.RESET} {message}",
        "error": f"{Fore.RED}!{Fore.RESET} {message}",
    }.get(kind, message), **print_args)

    colorama.ansi.Back.BLACK


if __name__ == "__main__":
    load_dotenv()
    colorama.init()
    args = parser.parse_args()

    if args.verbose:
        log(f"Bootstrapping Advent of Code {args.year} - Day {args.day}")

    if args.codegen:
        for template, dest in TEMPLATES.items():
            dest = dest.format(**args.__dict__)
            copyfile(template, dest)

            if args.verbose:
                log(f"Copied {template} to {dest}", kind="success")

        for filename, marker_inserts in INSERT_FILES.items():
            with open(filename, "r") as f:
                main_lines = [line.rstrip() for line in f.readlines()]

            for marker, inserts in marker_inserts.items():
                for i, line in enumerate(main_lines):
                    try:
                        indent = line.index(marker)
                        main_lines[i:i] = [indent * ' ' + insert.format(**args.__dict__) for insert in inserts]
                        break
                    except ValueError:
                        pass

            with open(filename, "w") as f:
                for line in main_lines:
                    print(line, file=f)

            if args.verbose:
                log(f"Inserted lines into {filename}", kind="success")

    if args.input:
        response = requests.get(
            INPUT_URL.format(**args.__dict__),
            cookies={"session": os.getenv("AOC_SESSION")},
        )

        if not response.ok:
            log("Error fetching input", file=sys.stderr)
            exit(1)

        write_output(response.text, args)

        if args.verbose:
            log("Fetched input", kind="success")

    if args.example:
        response = requests.get(EXAMPLE_URL.format(**args.__dict__))

        if not response.ok:
            log("Error fetching example", kind="error", file=sys.stderr)
            exit(1)

        bs = BeautifulSoup(response.text, features="html.parser")

        try:
            example_label = bs.find(text=re.compile("example"))
            content = example_label.parent.find_next_sibling("pre").find("code").text
        except AttributeError:
            log("Error finding example", kind="error", file=sys.stderr)
            exit(1)

        write_output(content + '\n', args, '-example')

        if args.verbose:
            log("Fetched example input", kind="success")
