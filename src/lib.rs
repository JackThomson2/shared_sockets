use pyo3::prelude::*;

use socket2::{Domain, Protocol, Socket, Type};
use std::net::SocketAddr;

#[pyclass]
#[derive(Debug)]
pub struct SocketHeld {
    socket: Socket,
}

#[pymethods]
impl SocketHeld {
    #[new]
    pub fn new(address: String) -> PyResult<SocketHeld> {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

        let address: SocketAddr = address.parse()?;
        socket.set_reuse_address(true)?;
        socket.set_reuse_port(true)?;
        socket.bind(&address.into())?;
        socket.listen(1024)?;

        Ok(SocketHeld { socket })
    }

    pub fn tell_info(&self) -> PyResult<String> {
        println!("Socket {:?}", self.socket.local_addr()?);

        Ok("".to_string())
    }

    pub fn listen(&self, caller: String, function: Py<PyAny>) -> PyResult<String> {
        Python::with_gil(|py| {
            println!("Thread {} is ready for a connection", caller);
            let res = self.socket.accept()?;
            let calling = function.as_ref(py);

            println!("Thread {} got socket {:?}", caller, res.1);

            calling.call1((caller,))?;
            Ok("".to_string())
        })
    }

    pub fn try_clone(&self) -> PyResult<SocketHeld> {
        let copied = self.socket.try_clone()?;

        println!("socket cloned sucessfully");

        Ok(SocketHeld { socket: copied })
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn testmulti(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SocketHeld>()?;

    Ok(())
}
