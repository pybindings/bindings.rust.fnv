``bindings.rust.fnv``
=====================

|Source| |Travis| |Codecov| |Crate|

.. |Source| image:: https://img.shields.io/badge/source-GitHub-303030.svg?style=flat-square
   :target: https://github.com/pybindings/bindings.rust.fnv

.. |Travis| image:: https://img.shields.io/travis/pybindings/bindings.rust.fnv/master.svg?style=flat-square
   :target: https://travis-ci.org/pybindings/bindings.rust.fnv

.. |Codecov| image:: https://img.shields.io/codecov/c/github/pybindings/bindings.rust.fnv/master.svg?style=flat-square
   :target: https://codecov.io/gh/pybindings/bindings.rust.fnv

.. |Crate| image:: https://img.shields.io/crates/v/fnv.svg?style=flat-square
   :target: https://crates.io/crates/fnv

Python bindings to the `fnv <https://crates.io/crates/fnv>`_ Rust crate.

Installation
------------

Build a `wheel package <https://wheel.rtfd.io>`_ from source
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Clone the repository locally and move there::

  git clone https://github.com/pybindings/bindings.rust.fnv
  cd bindings

Then, make sure the rust *nightly* toolchain is used to build the project
by running `rustup override` (if `rustup` is not installed, see the
`rustup.rs homepage <https://github.com/rust-lang-nursery/rustup.rs>`_)::

  rustup override set nightly

Then, install the `setuptools-rust <https://pypi.python.org/pypi/setuptools-rust>`_
and `wheel <https://pypi.python.org/pypi/wheel>`_ Python modules with
``pip``::

  pip install --user setuptools-rust wheel

Finally, compile the library into a wheel for your platform,
and install it::

  python setup.py bdist
