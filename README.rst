``bindings.rust.fnv``
=====================

|Source| |PyPI| |Crate| |Travis| |Codecov| |Versions| |Format| |License|

.. |Source| image:: https://img.shields.io/badge/source-GitHub-303030.svg?style=flat-square&maxAge=300
   :target: https://github.com/pybindings/bindings.rust.fnv

.. |Travis| image:: https://img.shields.io/travis/pybindings/bindings.rust.fnv/master.svg?style=flat-square&maxAge=300
   :target: https://travis-ci.org/pybindings/bindings.rust.fnv

.. |Codecov| image:: https://img.shields.io/codecov/c/github/pybindings/bindings.rust.fnv/master.svg?style=flat-square&maxAge=300
   :target: https://codecov.io/gh/pybindings/bindings.rust.fnv

.. |PyPI| image:: https://img.shields.io/pypi/v/bindings.rust.fnv.svg?style=flat-square&maxAge=300
   :target: https://pypi.python.org/pypi/bindings.rust.fnv

.. |Crate| image:: https://img.shields.io/crates/v/fnv.svg?style=flat-square&maxAge=300
   :target: https://crates.io/crates/fnv

.. |Format| image:: https://img.shields.io/pypi/format/bindings.rust.fnv.svg?style=flat-square&maxAge=300
   :target: https://pypi.python.org/pypi/bindings.rust.fnv

.. |Versions| image:: https://img.shields.io/pypi/pyversions/bindings.rust.fnv.svg?style=flat-square&maxAge=300
   :target: https://travis-ci.org/pybindings/bindings.rust.fnv

.. |License| image:: https://img.shields.io/pypi/l/bindings.rust.fnv.svg?style=flat-square&maxAge=300
   :target: https://choosealicense.com/licenses/mit


Python bindings to the `fnv <https://crates.io/crates/fnv>`_ Rust crate.

Installation
------------

Install from `wheel package <https://wheel.rtfd.io>`_
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Wheels are available for the following OS:

* Linux x86_64 / i686  (based on the `manylinux specification <https://www.python.org/dev/peps/pep-0513/>`_)
* Mac OSX

Simply run a recent ``pip``, and it will download a wheel if it is available for
your platform::

    $ pip install --user bindings.rust.fnv


Install from source distribution
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If your platform is not directly supported, you can still compile the library
yourself but you'll need the Rust *nightly* toolchain for this. You can use
``rustup`` to install the toolchain. See `rustup.rs <https://rustup.rs/>`_.

Then, install the `setuptools-rust <https://pypi.python.org/pypi/setuptools-rust>`_
and `wheel <https://pypi.python.org/pypi/wheel>`_ Python modules with
``pip``::

  $ pip install --user setuptools-rust wheel

Finally, use pip to install the library. It will download the source distribution,
and start compiling (this can take a long time)::

  $ pip install --user bindings.rust.fnv


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
