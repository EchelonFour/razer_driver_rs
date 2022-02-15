use crate::razer_report::*;
use bytes::{Buf, Bytes};
use hidapi::HidDevice;
use std::fmt::Display;

pub const RAZER_VENDOR_ID: u16 = 0x1532;

#[derive(Clone, Copy)]
pub struct RazerDeviceConnectInfo {
    pub interface_number: Option<i32>,
    pub usage: Option<u16>,
    pub usage_page: Option<u16>,
}

pub trait RazerDeviceKind {
    fn get_transaction_device(&self) -> RazerTransactionDevice;
}

pub struct RazerDevice<T>
where
    T: RazerDeviceKind,
{
    pub kind: T,
    pub name: String,
    pub(crate) hid_device: HidDevice,
}

pub struct RazerFirmwareVersion(u8, u8);

impl Display for RazerFirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}", self.0, self.1)
    }
}

impl<T> RazerDevice<T>
where
    T: RazerDeviceKind,
{
    pub fn new(kind: T, name: String, hid_device: HidDevice) -> Self {
        RazerDevice {
            kind,
            name,
            hid_device,
        }
    }

    pub fn get_firmware_version(&self) -> Result<RazerFirmwareVersion, RazerReportError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::FirmwareVersion,
            Bytes::new(),
            self.kind.get_transaction_device(),
        );
        let response_payload = report.send_and_receive_packet(&self.hid_device)?;
        Ok(RazerFirmwareVersion(
            *response_payload.get(0).unwrap(),
            *response_payload.get(1).unwrap(),
        ))
    }

    pub fn set_led_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
        percent: u8,
    ) -> Result<(), RazerReportError> {
        if percent > 100 {
            panic!("cannot set brightness to more than 100");
        }
        let report = RazerReport::new(
            RazerCommandDirection::HostToDevice,
            RazerCommand::LedBrightness,
            vec![store as u8, led as u8, percent].into(),
            self.kind.get_transaction_device(),
        );
        report.send_packet(&self.hid_device)?;
        Ok(())
    }

    pub fn get_led_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
    ) -> Result<u8, RazerReportError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::LedBrightness,
            vec![store as u8, led as u8].into(),
            self.kind.get_transaction_device(),
        );
        let mut response = report.send_and_receive_packet(&self.hid_device)?;
        response.advance(2);
        Ok(response.get_u8())
    }

    pub fn set_extended_matrix_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
        percent: u8,
    ) -> Result<(), RazerReportError> {
        if percent > 100 {
            panic!("cannot set brightness to more than 100");
        }
        let report = RazerReport::new(
            RazerCommandDirection::HostToDevice,
            RazerCommand::ExtendedMatrixBrightness,
            vec![store as u8, led as u8, percent].into(),
            self.kind.get_transaction_device(),
        );
        report.send_packet(&self.hid_device)?;
        Ok(())
    }

    pub fn get_extended_matrix_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
    ) -> Result<u8, RazerReportError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::ExtendedMatrixBrightness,
            vec![store as u8, led as u8].into(),
            self.kind.get_transaction_device(),
        );
        let mut response = report.send_and_receive_packet(&self.hid_device)?;
        response.advance(2);
        Ok(response.get_u8())
    }
}
