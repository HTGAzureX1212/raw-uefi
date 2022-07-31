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

use crate::types::{BOOLEAN, CHAR16, EFI_STATUS, INT32, UINTN};

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: EFI_TEXT_RESET,
    pub OutputString: EFI_TEXT_STRING,
    pub TestString: EFI_TEXT_TEST_STRING,
    pub QueryMode: EFI_TEXT_QUERY_MODE,
    pub SetMode: EFI_TEXT_SET_MODE,
    pub SetAttribute: EFI_TEXT_SET_ATTRIBUTE,
    pub ClearScreen: EFI_TEXT_CLEAR_SCREEN,
    pub SetCursorPosition: EFI_TEXT_SET_CURSOR_POSITION,
    pub EnableCursor: EFI_TEXT_ENABLE_CURSOR,
    pub Mode: *mut SIMPLE_TEXT_OUTPUT_MODE,
}

#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_MODE {
    pub MaxMode: INT32,
    pub Mode: INT32,
    pub Attribute: INT32,
    pub CursorColumn: INT32,
    pub CursorRow: INT32,
    pub CursorVisible: BOOLEAN,
}

pub type EFI_TEXT_RESET = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ExtendedVerification: BOOLEAN,
) -> EFI_STATUS;

pub type EFI_TEXT_STRING = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    String: *mut CHAR16,
) -> EFI_STATUS;

pub type EFI_TEXT_TEST_STRING = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    String: *mut CHAR16,
) -> EFI_STATUS;

pub type EFI_TEXT_QUERY_MODE = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ModeNumber: UINTN,
    Columns: *mut UINTN,
    Rows: *mut UINTN,
) -> EFI_STATUS;

pub type EFI_TEXT_SET_MODE = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ModeNumber: UINTN,
) -> EFI_STATUS;

pub type EFI_TEXT_SET_ATTRIBUTE = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    Attribute: UINTN,
) -> EFI_STATUS;

pub type EFI_TEXT_CLEAR_SCREEN = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
) -> EFI_STATUS;

pub type EFI_TEXT_SET_CURSOR_POSITION = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    Column: UINTN,
    Row: UINTN,
) -> EFI_STATUS;

pub type EFI_TEXT_ENABLE_CURSOR = unsafe extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    Visible: BOOLEAN,
) -> EFI_STATUS;
