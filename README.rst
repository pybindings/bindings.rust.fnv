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



Usage
-----

Import the module from the `bindings.rust <https://pypi.python.org/pypi/bindings.rust>`_
namespace:

.. code-block:: python

   >>> from bindings.rust import fnv

Create a ``FnvHasher`` object, with an optional integer key to initialize
the hasher state with.

.. code-block:: python

   >>> default_hasher = fnv.FnvHasher()
   >>> hasher_with_key = fnv.FnvHasher(key=255)

Then, use it as any `hash object <https://docs.python.org/3/library/hashlib.html#hash-algorithms>`_
from the ``hashlib`` module:

.. code-block:: python

  >>> default_hasher.update(b'hash this text')
  >>> default_hasher.update(bytearray(b'hash this as well'))
  >>> bytearray(default_hasher.digest())
  bytearray(b'\x02\xe5\xd2\x84\xae\x0f\xb9\xd3')
  >>> default_hasher.hexdigest()
  'd3b90fae84d2e502'
