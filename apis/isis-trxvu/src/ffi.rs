/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use radio_api::RadioError;

// So that we can copy the member names for the C
// structs/enums

pub const RX_MAX_SIZE: usize = 200;

/// The radio_telem is a union in the C source
/// The largest element in the union holds six uint16_t
/// For simplicity we will use a buffer of two uint8_t
#[repr(C)]
pub struct radio_telem(pub [u8; 12]);

/// Enum for selecting the telemetry type
#[repr(C)]
pub enum radio_telem_type {
    tx_telem_all,
    tx_telem_last,
    tx_uptime,
    tx_state,
    rx_telem_all,
    rx_uptime,
}

/// Enum for radio status
/// from radio-api/radio-struct.h
#[repr(C)]
#[derive(Debug)]
pub enum radio_status {
    radio_ok,
    radio_rx_empty,
    radio_error,
    radio_error_config,
}

impl From<radio_status> for RadioError {
    fn from(s: radio_status) -> Self {
        match s {
            _ => RadioError::HardwareError {
                message: format!("{:?}", s),
            },
        }
    }
}

// Helper function to convert radio status to radio error
pub fn radio_status_to_err(status: radio_status) -> Result<(), RadioError> {
    match status {
        radio_status::radio_ok => Ok(()),
        _ => Err(RadioError::HardwareError {
            message: format!("TRXVU radio error {:?}", status),
        }),
    }
}

#[repr(C)]
pub struct radio_rx_message {
    pub msg_size: u16,
    pub doppler_offset: u16,
    pub signal_strength: u16,
    pub message: [u8; RX_MAX_SIZE],
}

extern "C" {
    pub fn k_radio_init() -> radio_status;
    pub fn k_radio_watchdog_start() -> radio_status;
    pub fn k_radio_watchdog_stop() -> radio_status;
    pub fn k_radio_terminate();
    pub fn k_radio_get_telemetry(
        buffer: *mut radio_telem,
        telem_type: radio_telem_type,
    ) -> radio_status;

    pub fn k_radio_send(buffer: *const u8, len: i32, response: *mut u8) -> radio_status;

    pub fn k_radio_recv(buffer: *mut radio_rx_message, len: *mut u8) -> radio_status;

}