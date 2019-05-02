//! Driver for AHCI
//!
//! Spec: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/serial-ata-ahci-spec-rev1-3-1.pdf

use alloc::string::String;
use alloc::sync::Arc;

use isomorphic_drivers::block::ahci::{AHCI, BLOCK_SIZE};

use crate::drivers::provider::Provider;
use crate::drivers::BlockDriver;
use crate::sync::SpinNoIrqLock as Mutex;

use super::super::{DeviceType, Driver, BLK_DRIVERS, DRIVERS};

pub struct AHCIDriver(Mutex<AHCI<Provider>>);
use rcore_fs::dev::DevError;

/// A specialized `Result` type for device.
pub type Result<T> = core::result::Result<T, DevError>;

impl Driver for AHCIDriver {
    fn try_handle_interrupt(&self, _irq: Option<u32>) -> bool {
        false
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Block
    }

    fn get_id(&self) -> String {
        format!("ahci")
    }

    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> Result<()>  {
        let mut driver = self.0.lock();
        driver.read_block(block_id, buf);
        Ok(())
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) -> Result<()>  {
        if buf.len() < BLOCK_SIZE {
            return Err(DevError);
        }
        let mut driver = self.0.lock();
        driver.write_block(block_id, buf);
        Ok(())
    }
}

pub fn init(_irq: Option<u32>, header: usize, size: usize) -> Arc<AHCIDriver> {
    let ahci = AHCI::new(header, size);
    let driver = Arc::new(AHCIDriver(Mutex::new(ahci)));
    DRIVERS.write().push(driver.clone());
    BLK_DRIVERS
        .write()
        .push(Arc::new(BlockDriver(driver.clone())));
    driver
}
