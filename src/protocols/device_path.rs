/* SPDX-License-Identifier: GPL-3.0-only
 *
 * RawUEFI: UEFI Specification Definitions
 * Copyright (C) 2022 HTGAzureX1212.
 *
 * RawUEFI is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * RawUEFI is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with RawUEFI.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::types::{BOOLEAN, UINT16, UINT8, UINTN};

#[repr(C)]
pub struct EFI_DEVICE_PATH_PROTOCOL {
    pub Type: UINT8,
    pub SubType: UINT8,
    pub Length: [UINT8; 2],
}

#[repr(C)]
pub struct EFI_DEVICE_PATH_UTILITIES_PROTOCOL {
    pub GetDevicePathSize: EFI_DEVICE_PATH_UTILS_GET_DEVICE_PATH_SIZE,
    pub DuplicateDevicePath: EFI_DEVICE_PATH_UTILS_DUP_DEVICE_PATH,
    pub AppendDevicePath: EFI_DEVICE_PATH_UTILS_APPEND_PATH,
    pub AppendDeviceNode: EFI_DEVICE_PATH_UTILS_APPEND_NODE,
    pub AppendDevicePathInstance: EFI_DEVICE_PATH_UTILS_APPEND_INSTANCE,
    pub GetNextDevicePathInstance: EFI_DEVICE_PATH_UTILS_GET_NEXT_INSTANCE,
    pub CreateDeviceNode: EFI_DEVICE_PATH_UTILS_CREATE_NODE,
    pub IsDevicePathMultiInstance: EFI_DEVICE_PATH_UTILS_IS_MULTI_INSTANCE,
}

pub type EFI_DEVICE_PATH_UTILS_GET_DEVICE_PATH_SIZE = unsafe extern "efiapi" fn(
    DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
) -> UINTN;

pub type EFI_DEVICE_PATH_UTILS_DUP_DEVICE_PATH = unsafe extern "efiapi" fn(
    DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
) -> *mut EFI_DEVICE_PATH_PROTOCOL;

pub type EFI_DEVICE_PATH_UTILS_APPEND_PATH = unsafe extern "efiapi" fn(
    Src1: *const EFI_DEVICE_PATH_PROTOCOL,
    Src2: *const EFI_DEVICE_PATH_PROTOCOL,
) -> *mut EFI_DEVICE_PATH_PROTOCOL;

pub type EFI_DEVICE_PATH_UTILS_APPEND_NODE = unsafe extern "efiapi" fn(
    DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
    DeviceNode: *const EFI_DEVICE_PATH_PROTOCOL,
) -> *mut EFI_DEVICE_PATH_PROTOCOL;

pub type EFI_DEVICE_PATH_UTILS_APPEND_INSTANCE = unsafe extern "efiapi" fn(
    DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
    DevicePathInstance: *const EFI_DEVICE_PATH_PROTOCOL,
) -> *mut EFI_DEVICE_PATH_PROTOCOL;

pub type EFI_DEVICE_PATH_UTILS_GET_NEXT_INSTANCE = unsafe extern "efiapi" fn(
    DevicePathInstance: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
    DevicePathInstanceSize: *mut UINTN,
) -> *mut EFI_DEVICE_PATH_PROTOCOL;

pub type EFI_DEVICE_PATH_UTILS_CREATE_NODE = unsafe extern "efiapi" fn(
    NodeType: UINT8,
    NodeSubType: UINT8,
    NodeLength: UINT16,
) -> *mut EFI_DEVICE_PATH_PROTOCOL;

pub type EFI_DEVICE_PATH_UTILS_IS_MULTI_INSTANCE = unsafe extern "efiapi" fn(
    DevicePath: *const EFI_DEVICE_PATH_PROTOCOL
) -> BOOLEAN;
