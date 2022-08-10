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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;
use crate::types::{EFI_EVENT, EFI_GUID, EFI_HANDLE, EFI_PHYSICAL_ADDRESS, EFI_STATUS, EFI_TPL, EFI_VIRTUAL_ADDRESS, UINT32, UINT64, UINTN, VOID};

#[repr(C)]
pub struct EFI_BOOT_SERVICES {
    pub CreateEvent: EFI_BOOT_CREATE_EVENT,
    pub CreateEventEx: EFI_BOOT_CREATE_EVENT_EX,
    pub CloseEvent: EFI_BOOT_CLOSE_EVENT,
    pub SignalEvent: EFI_BOOT_SIGNAL_EVENT,
    pub WaitForEvent: EFI_BOOT_WAIT_FOR_EVENT,
    pub CheckEvent: EFI_BOOT_CHECK_EVENT,

    pub SetTimer: EFI_BOOT_SET_TIMER,

    pub RaiseTpl: EFI_BOOT_RAISE_TPL,
    pub RestoreTpl: EFI_BOOT_RESTORE_TPL,

    pub AllocatePages: EFI_BOOT_ALLOCATE_PAGES,
    pub FreePages: EFI_BOOT_FREE_PAGES,
    pub GetMemoryMap: EFI_BOOT_GET_MEMORY_MAP,

    pub AllocatePool: EFI_BOOT_ALLOCATE_POOL,
    pub FreePool: EFI_BOOT_FREE_POOL,

    pub InstallProtocolInterface: EFI_BOOT_INSTALL_PROTOCOL_INTERFACE,
    pub UninstallProtocolInterface: EFI_BOOT_UNINSTALL_PROTOCOL_INTERFACE,
    pub ReinstallProtocolInterface: EFI_BOOT_REINSTALL_PROTOCOL_INTERFACE,

    pub RegisterProtocolVerify: EFI_BOOT_REGISTER_PROTOCOL_NOTIFY,

    pub LocateHandle: EFI_BOOT_LOCATE_HANDLE,
    pub HandleProtocol: EFI_BOOT_HANDLE_PROTOCOL,
    pub LocateDevicePath: EFI_BOOT_LOCATE_DEVICE_PATH,
}

#[repr(C)]
pub enum EFI_ALLOCATE_TYPE {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[repr(C)]
pub enum EFI_INTERFACE_TYPE {
    EFI_NATIVE_INTERFACE,
}

#[repr(C)]
pub enum EFI_LOCATE_SEARCH_TYPE {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

#[repr(C)]
pub enum EFI_MEMORY_TYPE {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}

#[repr(C)]
pub enum EFI_TIMER_DELAY {
    TimerCancel,
    TimerPeriodic,
    TimerRelative,
}

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    pub Type: UINT32,
    pub PhysicalStart: EFI_PHYSICAL_ADDRESS,
    pub VirtualStart: EFI_VIRTUAL_ADDRESS,
    pub NumberOfPages: UINT64,
    pub Attribute: UINT64,
}

pub type EFI_EVENT_NOTIFY = unsafe extern "efiapi" fn(
    Event: EFI_EVENT,
    Context: *mut VOID,
);

pub type EFI_BOOT_CREATE_EVENT = unsafe extern "efiapi" fn(
    Type: UINT32,
    NotifyTpl: EFI_TPL,
    NotifyFunction: EFI_EVENT_NOTIFY,
    NotifyContext: *mut VOID,
    Event: *mut EFI_EVENT
) -> EFI_STATUS;

pub type EFI_BOOT_CREATE_EVENT_EX = unsafe extern "efiapi" fn(
    Type: UINT32,
    NotifyTpl: EFI_TPL,
    NotifyFunction: EFI_EVENT_NOTIFY,
    NotifyContext: *mut VOID,
    EventGroup: *mut EFI_GUID,
    Event: *mut EFI_EVENT,
) -> EFI_STATUS;

pub type EFI_BOOT_CLOSE_EVENT = unsafe extern "efiapi" fn(
    Event: EFI_EVENT,
) -> EFI_STATUS;

pub type EFI_BOOT_SIGNAL_EVENT = unsafe extern "efiapi" fn(
    Event: EFI_EVENT,
) -> EFI_STATUS;

pub type EFI_BOOT_WAIT_FOR_EVENT = unsafe extern "efiapi" fn(
    NumberOfEvents: UINTN,
    Events: *mut EFI_EVENT,
    Index: *mut UINTN,
) -> EFI_STATUS;

pub type EFI_BOOT_CHECK_EVENT = unsafe extern "efiapi" fn(
    Event: EFI_EVENT,
) -> EFI_STATUS;

pub type EFI_BOOT_SET_TIMER = unsafe extern "efiapi" fn(
    Event: EFI_EVENT,
    Type: EFI_TIMER_DELAY,
    TriggerTime: UINT64,
) -> EFI_STATUS;

pub type EFI_BOOT_RAISE_TPL = unsafe extern "efiapi" fn(
    NewTpl: EFI_TPL,
) -> EFI_STATUS;

pub type EFI_BOOT_RESTORE_TPL = unsafe extern "efiapi" fn(
    OldTpl: EFI_TPL,
) -> EFI_STATUS;

pub type EFI_BOOT_ALLOCATE_PAGES = unsafe extern "efiapi" fn(
    Type: EFI_ALLOCATE_TYPE,
    MemoryType: EFI_MEMORY_TYPE,
    Pages: UINTN,
    Memory: *mut EFI_PHYSICAL_ADDRESS,
) -> EFI_STATUS;

pub type EFI_BOOT_FREE_PAGES = unsafe extern "efiapi" fn(
    Memory: EFI_PHYSICAL_ADDRESS,
    Pages: UINTN,
) -> EFI_STATUS;

pub type EFI_BOOT_GET_MEMORY_MAP = unsafe extern "efiapi" fn(
    MemoryMapSize: *mut UINTN,
    MemoryMap: *mut EFI_MEMORY_DESCRIPTOR,
    MapKey: *mut UINTN,
    DescriptorSize: *mut UINTN,
    DescriptorVersion: *mut UINT32,
) -> EFI_STATUS;

pub type EFI_BOOT_ALLOCATE_POOL = unsafe extern "efiapi" fn(
    PoolType: EFI_MEMORY_TYPE,
    Size: UINTN,
    Buffer: *mut *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_FREE_POOL = unsafe extern "efiapi" fn(
    Buffer: *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_INSTALL_PROTOCOL_INTERFACE = unsafe extern "efiapi" fn(
    Handle: *mut EFI_HANDLE,
    Protocol: *mut EFI_GUID,
    InterfaceType: EFI_INTERFACE_TYPE,
    Interface: *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_UNINSTALL_PROTOCOL_INTERFACE = unsafe extern "efiapi" fn(
    Handle: *mut EFI_HANDLE,
    Protocol: *mut EFI_GUID,
    Interface: *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_REINSTALL_PROTOCOL_INTERFACE = unsafe extern "efiapi" fn(
    Handle: *mut EFI_HANDLE,
    Protocol: *mut EFI_GUID,
    OldInterface: *mut VOID,
    NewInterface: *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_REGISTER_PROTOCOL_NOTIFY = unsafe extern "efiapi" fn(
    Protocol: *mut EFI_GUID,
    Event: EFI_EVENT,
    Registration: *mut *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_LOCATE_HANDLE = unsafe extern "efiapi" fn(
    SearchType: EFI_LOCATE_SEARCH_TYPE,
    Protocol: *mut EFI_GUID,
    SearchKey: *mut VOID,
    BufferSize: *mut UINTN,
    Buffer: *mut EFI_HANDLE,
) -> EFI_STATUS;

pub type EFI_BOOT_HANDLE_PROTOCOL = unsafe extern "efiapi" fn(
    Handle: EFI_HANDLE,
    Protocol: *mut EFI_GUID,
    Interface: *mut *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_LOCATE_DEVICE_PATH = unsafe extern "efiapi" fn(
    Protocol: *mut EFI_GUID,
    DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
    Device: *mut EFI_HANDLE,
) -> EFI_STATUS;
