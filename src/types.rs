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

use core::ffi::c_void;

pub type BOOLEAN = u8;

pub type INTN = isize;
pub type UINTN = usize;

pub type INT8 = i8;
pub type UINT8 = u8;

pub type INT16 = i16;
pub type UINT16 = u16;

pub type INT32 = i32;
pub type UINT32 = u32;

pub type INT64 = i64;
pub type UINT64 = u64;

pub type INT128 = i128;
pub type UINT128 = u128;

pub type CHAR8 = u8;
pub type CHAR16 = u16;

pub type VOID = c_void;

#[repr(C)]
pub struct EFI_GUID {
    pub Data1: UINT32,
    pub Data2: UINT16,
    pub Data3: UINT16,
    pub Data4: [UINT8; 8],
}

pub type EFI_STATUS = UINTN;

pub type EFI_HANDLE = *mut VOID;
pub type EFI_EVENT = *mut VOID;

pub type EFI_LBA = UINT64;

pub type EFI_TPL = UINTN;

#[repr(C)]
pub struct EFI_MAC_ADDRESS {
    pub Addr: [UINT8; 32],
}

#[repr(C)]
pub struct EFI_IPv4_ADDRESS {
    pub Addr: [UINT8; 4],
}

#[repr(C)]
pub struct EFI_IPv6_ADDRESS {
    pub Addr: [UINT8; 16],
}

#[repr(C)]
pub union EFI_IP_ADDRESS {
    pub Addr: [UINT32; 4],
    pub v4: EFI_IPv4_ADDRESS,
    pub v6: EFI_IPv6_ADDRESS,
}
