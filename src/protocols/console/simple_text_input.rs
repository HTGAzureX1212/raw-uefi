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

use crate::types::{BOOLEAN, CHAR16, EFI_EVENT, EFI_STATUS, UINT16};

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    pub Reset: EFI_INPUT_RESET,
    pub ReadKey: EFI_INPUT_READ_KEY,
    pub WaitForKey: EFI_EVENT,
}

#[repr(C)]
pub struct EFI_INPUT_KEY {
    pub ScanCode: UINT16,
    pub UnicodeChar: CHAR16,
}

pub type EFI_INPUT_RESET = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ExtendedVerification: BOOLEAN,
) -> EFI_STATUS;

pub type EFI_INPUT_READ_KEY = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    Key: *mut EFI_INPUT_KEY,
) -> EFI_STATUS;
