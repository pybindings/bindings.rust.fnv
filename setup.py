#!/usr/bin/env python
# coding: utf-8
import setuptools
import setuptools_rust as rust

setuptools.setup(
    rust_extensions=[
        rust.RustExtension(
            "bindings.rust.fnv",
            "bindings/rust/Cargo.toml",
            binding=rust.Binding.PyO3,
            strip=rust.Strip.Debug,
        )
    ]
)
