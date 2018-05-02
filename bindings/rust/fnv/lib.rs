#![feature(proc_macro, specialization, const_fn)]

extern crate byteorder;
extern crate fnv;
extern crate pyo3;
extern crate regex;

use std::hash::Hasher;

use byteorder::ByteOrder;
use byteorder::NativeEndian;

use pyo3::py::*;
use pyo3::prelude::*;
use pyo3::exc::TypeError;
use pyo3::buffer::PyBuffer;

/// A hasher used to calculate the FNV checksum of a string of information.
///
/// Arguments:
///     key (`int`, *optional*): a key to initialize the hash object with.
///
/// Attributes:
///     block_size (`int`): number of bytes of a block read by this object.
///     name (`str`): the hash algorithm being used by this object.
///     digest_size (`int`): number of bytes in this hashes output.
///
/// See also:
///     `FNV article on Wikipedia
///     <https://en.wikipedia.org/wiki/Fowler-Noll-Vo_hash_function>`_
///
#[class]
struct FnvHasher {
    hasher: fnv::FnvHasher,
    token: PyToken,
}

impl FnvHasher {
    fn new(token: PyToken, key: Option<u64>) -> Self {
        FnvHasher {
            token,
            hasher: key.map_or_else(fnv::FnvHasher::default, |k| fnv::FnvHasher::with_key(k)),
        }
    }
}

#[methods]
impl FnvHasher {

    #[new]
    fn __init__(obj: &PyRawObject, key: Option<u64>) -> PyResult<()> {
        obj.init(|token| FnvHasher::new(token, key))
    }

    /// Return the digest value as an integer.
    fn finish(&self) -> PyResult<u64> {
        Ok(self.hasher.finish())
    }

    /// Return a copy of the hash object.
    fn copy(&self) -> PyResult<Py<FnvHasher>> {
        self.token.py().init(|token| FnvHasher {
            token,
            hasher: fnv::FnvHasher::with_key(self.hasher.finish()),
        })
    }

    /// Return the digest value as a string of binary data.
    fn digest(&self) -> PyResult<Py<PyBytes>> {
        let mut buffer: [u8; 8] = [0; 8];
        NativeEndian::write_u64(&mut buffer, self.hasher.finish());
        Ok(PyBytes::new(self.token.py(), &buffer))
    }

    /// Return the digest value as a string of hexadecimal digits.
    fn hexdigest(&self) -> PyResult<String> {
        Ok(format!("{:x}", self.hasher.finish()))
    }

    /// Update this hash object's state with the provided string.
    fn update(&mut self, data: &PyObjectRef) -> PyResult<()> {
        let buf = PyBuffer::get(self.token.py(), data)?;
        if let Some(s) = buf.as_slice::<u8>(self.token.py()) {
            let raw_data: &[u8];
            unsafe { raw_data = std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len()) }
            Ok(self.hasher.write(raw_data))
        } else {
            Err(TypeError::new("object supporting the buffer API required"))
        }
    }

    /// Alias of ``FnvHasher.update``
    fn write(&mut self, data: &PyObjectRef) -> PyResult<()> {
        self.update(data)
    }

    /// The hash algorithm being used by this object.
    #[getter]
    fn name(&self) -> PyResult<&str> {
        Ok("fnv")
    }

    /// Number of bytes in this hashes output.
    #[getter]
    fn digest_size(&self) -> PyResult<usize> {
        Ok(8)
    }

    /// Number of bytes of a block read by this object.
    #[getter]
    fn block_size(&self) -> PyResult<usize> {
        Ok(1)
    }
}

/// Bindings to the `fnv <https://crates.io/crates/fnv>`_ crate.
#[modinit(fnv)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    let authors_re = regex::Regex::new(r"(.*) <(.*)>").unwrap();

    if let Some(captures) = authors_re.captures(env!("CARGO_PKG_AUTHORS")) {
        m.add("__author__", captures.get(1).unwrap().as_str())?;
        m.add("__author_email__", captures.get(2).unwrap().as_str())?;
    } else {
        m.add("__author__", py.None())?;
        m.add("__author_email__", py.None())?;
    }
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<FnvHasher>()?;

    /// Return a fnv hash object.
    ///
    /// Arguments:
    ///     key (int, *optional*): a key to initialize the hash object with.
    ///
    #[pyfn(m, "fnv")]
    fn fnv_py(py: Python, key: Option<u64>) -> PyResult<Py<FnvHasher>> {
        py.init(|token| FnvHasher::new(token, key))
    }

    Ok(())
}
