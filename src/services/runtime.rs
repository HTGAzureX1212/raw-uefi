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

use crate::types::{BOOLEAN, CHAR16, EFI_GUID, EFI_STATUS, INT16, UINT16, UINT32, UINT64, UINT8, UINTN, VOID};

#[repr(C)]
pub struct EFI_RUNTIME_SERVICES {
    pub GetVariable: EFI_RUNTIME_GET_VARIABLE,
    pub GetNextVariableName: EFI_RUNTIME_GET_NEXT_VARIABLE_NAME,
    pub SetVariable: EFI_RUNTIME_SET_VARIABLE,
    pub QueryVariableInfo: EFI_RUNTIME_QUERY_VARIABLE_INFO,

    pub GetTime: EFI_RUNTIME_GET_TIME,
    pub SetTime: EFI_RUNTIME_SET_TIME,
}

#[repr(C)]
pub struct EFI_TIME {
    pub Year: UINT16,
    pub Month: UINT8,
    pub Day: UINT8,
    pub Hour: UINT8,
    pub Minute: UINT8,
    pub Second: UINT8,
    pub Pad1: UINT8,
    pub Nanosecond: UINT32,
    pub TimeZone: INT16,
    pub Daylight: UINT8,
    pub Pad2: UINT8,
}

#[repr(C)]
pub struct EFI_TIME_CAPABILITIES {
    pub Resolution: UINT32,
    pub Accuracy: UINT32,
    pub SetsToZero: BOOLEAN,
}

pub type EFI_RUNTIME_GET_VARIABLE = unsafe extern "efiapi" fn(
    VariableName: *mut CHAR16,
    VendorGuid: *mut EFI_GUID,
    Attributes: *mut UINT32,
    DataSize: *mut UINTN,
    Data: *mut VOID,
) -> EFI_STATUS;

pub type EFI_RUNTIME_GET_NEXT_VARIABLE_NAME = unsafe extern "efiapi" fn(
    VariableNameSize: *mut UINTN,
    VariableName: *mut CHAR16,
    VendorGuid: *mut EFI_GUID,
) -> EFI_STATUS;

pub type EFI_RUNTIME_SET_VARIABLE = unsafe extern "efiapi" fn(
    VariableName: *mut CHAR16,
    VendorGuid: *mut EFI_GUID,
    Attributes: UINT32,
    DataSize: UINTN,
    Data: *mut VOID
) -> EFI_STATUS;

pub type EFI_RUNTIME_QUERY_VARIABLE_INFO = unsafe extern "efiapi" fn(
    Attributes: UINT32,
    MaximumVariableStorageSize: *mut UINT64,
    RemainingVariableStorageSize: *mut UINT64,
    MaximumVariableSize: *mut UINT64,
) -> EFI_STATUS;

pub type EFI_RUNTIME_GET_TIME = unsafe extern "efiapi" fn(
    Time: *mut EFI_TIME,
    Capabilities: *mut EFI_TIME_CAPABILITIES,
) -> EFI_STATUS;

pub type EFI_RUNTIME_SET_TIME = unsafe extern "efiapi" fn(
    Time: *mut EFI_TIME,
) -> EFI_STATUS;
