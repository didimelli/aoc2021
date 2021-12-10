from pathlib import Path
from typing import List

from typer import Abort
from typer import echo
from typer import run


def new(name: str) -> None:
    bin = Path("src/bin/")
    files = [str(f.name) for f in bin.iterdir()]
    if any([name in f for f in files]):
        Abort("Name {name} already present in directory!")
    match: List[str] = []
    for f in files:
        if name in f:
            match.append(f)
    if match:
        echo(
            (
                f"Name {name} already present in the following files: \n",
                "\n".join(match),
            )
        )
        raise Abort()
    with (bin / "template.txt").open() as f:  # type: ignore
        rust_template = f.read()  # type: ignore
    with (bin / f"{name}.rs").open("w+") as f:  # type: ignore
        f.write(rust_template)  # type: ignore
    (bin / f"{name}_example.txt").touch()
    (bin / f"{name}_input.txt").touch()
    echo(f"Generated files:\n{name}.rs\n{name}_example.txt\n{name}_input.txt")


def main():
    run(new)


if __name__ == "__main__":
    run(new)
