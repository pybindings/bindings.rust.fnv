#![feature(proc_macro, specialization, const_fn)]

extern crate byteorder;
extern crate fnv;
extern crate pyo3;

use std::hash::Hasher;

use byteorder::ByteOrder;
use byteorder::NativeEndian;

use pyo3::prelude::*;
use pyo3::exc::TypeError;
use pyo3::buffer::PyBuffer;

#[py::class]
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

#[py::methods]
impl FnvHasher {
    #[new]
    fn __new__(obj: &PyRawObject, key: Option<u64>) -> PyResult<()> {
        obj.init(|token| FnvHasher::new(token, key))
    }

    //
    fn copy(&self) -> PyResult<Py<FnvHasher>> {
        self.token.py().init(|token| FnvHasher {
            token,
            hasher: fnv::FnvHasher::with_key(self.hasher.finish()),
        })
    }

    fn digest(&self) -> PyResult<Py<PyBytes>> {
        let mut buffer: [u8; 8] = [0; 8];
        NativeEndian::write_u64(&mut buffer, self.hasher.finish());
        Ok(PyBytes::new(self.token.py(), &buffer))
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(format!("{:x}", self.hasher.finish()))
    }

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

    #[getter]
    fn name(&self) -> PyResult<&str> {
        Ok("fnv")
    }

    #[getter]
    fn digest_size(&self) -> PyResult<usize> {
        Ok(8)
    }

    #[getter]
    fn block_size(&self) -> PyResult<usize> {
        Ok(1)
    }
}

#[py::modinit(fnv)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FnvHasher>()?;

    #[pyfn(m, "fnv")]
    fn fnv_py(py: Python, key: Option<u64>) -> PyResult<Py<FnvHasher>> {
        py.init(|token| FnvHasher::new(token, key))
    }

    Ok(())
}
