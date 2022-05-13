#!/usr/bin/env python

from setuptools_rust import Binding, RustExtension
from setuptools import setup
from setuptools import dist
dist.Distribution().fetch_build_eggs(['setuptools_rust'])

setup(
    name="fliton-fib-rs",
    version="0.1"
    rust_extensions=[
        RustExtension(".fliton_fib_rs.fliton_fib_rs",
                      path="Cargo.toml",
                      binding=Binding.PyO3)
    ],
    packages=["fliton_fib_rs"],
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    zip_safe=False,
)
