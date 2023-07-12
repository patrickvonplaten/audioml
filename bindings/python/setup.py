import re

from setuptools import setup
from setuptools_rust import Binding, RustExtension


# IMPORTANT:
# 1. all dependencies should be listed here with their version requirements if any
_deps = [
    "numpy"
]


deps = {b: a for a, b in (re.findall(r"^(([^!=<>~ ]+)(?:[!=<>~ ].*)?$)", x)[0] for x in _deps)}


def deps_list(*pkgs):
    return [deps[pkg] for pkg in pkgs]


extras = {}

with open("py_src/wavem/__init__.py", "r") as f:
    version = f.readline().split("=")[-1].strip().strip('"')

setup(
    name="wavem",
    version=version,
    description="Fast and Safe Audio Decoding and Processing Library",
    long_description=open("README.md", "r", encoding="utf-8").read(),
    long_description_content_type="text/markdown",
    keywords="",
    author="",
    author_email="",
    url="https://github.com/patrickvonplaten/wavem",
    license="Apache License 2.0",
    # rust_extensions=[RustExtension("wavem._wavem_rust", binding=Binding.PyO3, debug=False)],
    extras_require=extras,
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "Intended Audience :: Education",
        "Intended Audience :: Science/Research",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],
    package_dir={"": "py_src"},
    packages=[
        "wavem",
    ],
    package_data={},
    zip_safe=False,
)
