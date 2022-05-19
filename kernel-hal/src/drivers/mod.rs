//use crate::sync::Condvar;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use smoltcp::socket::SocketSet;
use spin::Mutex;
use spin::RwLock;

//pub use self::virtio::*;
pub use block::BlockDriver;
pub use net::NetDriver;

/*
/// virtio device
pub mod virtio;
*/
pub mod block;

/// Network controller
pub mod net;

#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    Net,
    Gpu,
    Input,
    Block,
    Rtc,
    Serial,
    Intc,
}

use downcast_rs::DowncastSync;

pub trait Driver: DowncastSync + Send + Sync {
    // if interrupt belongs to this driver, handle it and return true
    // return false otherwise
    // irq number is provided when available
    // driver should skip handling when irq number is mismatched
    fn try_handle_interrupt(&self, irq: Option<usize>) -> bool;

    // return the correspondent device type, see DeviceType
    fn device_type(&self) -> DeviceType;

    // get unique identifier for this device
    // should be different for each instance
    fn get_id(&self) -> String;

    // trait casting
    fn as_net(&self) -> Option<&dyn NetDriver> {
        None
    }

    fn as_block(&self) -> Option<&dyn BlockDriver> {
        None
    }

    /*
    fn as_rtc(&self) -> Option<&dyn RtcDriver> {
        None
    }
    */
}

lazy_static! {
    // NOTE: RwLock only write when initializing drivers
    pub static ref DRIVERS: RwLock<Vec<Arc<dyn Driver>>> = RwLock::new(Vec::new());
    pub static ref NET_DRIVERS: RwLock<Vec<Arc<dyn NetDriver>>> = RwLock::new(Vec::new());
    pub static ref BLK_DRIVERS: RwLock<Vec<Arc<dyn BlockDriver>>> = RwLock::new(Vec::new());
}

lazy_static! {
    /// Global SocketSet in smoltcp.
    ///
    /// Because smoltcp is a single thread network stack,
    /// every socket operation needs to lock this.
    pub static ref SOCKETS: Arc<Mutex<SocketSet<'static>>> =
    Arc::new(Mutex::new(SocketSet::new(vec![])));
}

/*
lazy_static! {
    //pub static ref SOCKET_ACTIVITY: Condvar = Condvar::new();
}
*/

#[allow(warnings)]
#[export_name = "hal_get_driver"]
#[no_mangle]
pub extern "C" fn get_net_driver() -> Vec<Arc<dyn NetDriver>> {
    NET_DRIVERS.read().clone()
}

#[export_name = "hal_get_net_sockets"]
pub fn get_net_sockets() -> Arc<Mutex<SocketSet<'static>>> {
    SOCKETS.clone()
}