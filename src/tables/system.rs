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

use crate::types::{CHAR16, EFI_HANDLE, UINT32};
use crate::protocols::console::simple_text_input::EFI_SIMPLE_TEXT_INPUT_PROTOCOL;
use crate::protocols::console::simple_text_output::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;
use crate::services::runtime::EFI_RUNTIME_SERVICES;
use crate::tables::EFI_TABLE_HEADER;

#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    pub Hdr: EFI_TABLE_HEADER,
    pub FirmwareVendor: *mut CHAR16,
    pub FirmwareRevision: UINT32,
    pub ConsoleInHandle: EFI_HANDLE,
    pub ConIn: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    pub ConsoleOutHandle: EFI_HANDLE,
    pub ConOut: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub StandardErrorHandle: EFI_HANDLE,
    pub StdErr: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub RuntimeServices: *mut EFI_RUNTIME_SERVICES,
}
