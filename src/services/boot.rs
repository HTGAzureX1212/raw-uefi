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
use crate::tables::system::EFI_SYSTEM_TABLE;
use crate::types::{BOOLEAN, CHAR16, EFI_EVENT, EFI_GUID, EFI_HANDLE, EFI_PHYSICAL_ADDRESS, EFI_STATUS, EFI_TPL, EFI_VIRTUAL_ADDRESS, UINT32, UINT64, UINT8, UINTN, VOID};

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

    pub OpenProtocol: EFI_BOOT_OPEN_PROTOCOL,
    pub CloseProtocol: EFI_BOOT_CLOSE_PROTOCOL,
    pub OpenProtocolInformation: EFI_BOOT_OPEN_PROTOCOL_INFORMATION,

    pub ConnectController: EFI_BOOT_CONNECT_CONTROLLER,
    pub DisconnectController: EFI_BOOT_DISCONNECT_CONTROLLER,

    pub ProtocolsPerHandle: EFI_BOOT_PROTOCOLS_PER_HANDLE,
    pub LocateHandleBuffer: EFI_BOOT_LOCATE_HANDLE_BUFFER,

    pub LocateProtocol: EFI_BOOT_LOCATE_PROTOCOL,
    pub InstallMultipleProtocolInterfaces: EFI_BOOT_INSTALL_MULTIPLE_PROTOCOL_INTERFACES,
    pub UnnstallMultipleProtocolInterfaces: EFI_BOOT_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES,

    pub LoadImage: EFI_BOOT_IMAGE_LOAD,
    pub StartImage: EFI_BOOT_IMAGE_START,
    pub UnloadImage: EFI_BOOT_IMAGE_UNLOAD,

    pub Exit: EFI_BOOT_EXIT,
    pub ExitBootServices: EFI_BOOT_EXIT_BOOT_SERVICES,

    pub SetWatchdogTimer: EFI_BOOT_SET_WATCHDOG_TIMER,
    pub Stall: EFI_BOOT_STALL,

    pub CopyMem: EFI_BOOT_COPY_MEM,
    pub SetMem: EFI_BOOT_SET_MEM,

    pub GetNextMonotonicCount: EFI_BOOT_GET_NEXT_MONOTONIC_COUNT,

    pub InstallConfigurationTable: EFI_BOOT_INSTALL_CONFIGURATION_TABLE,

    pub CalculateCrc32: EFI_BOOT_CALCULATE_CRC32,
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

#[repr(C)]
pub struct EFI_OPEN_PROTOCOL_INFORMATION_ENTRY {
    pub AgentHandle: EFI_HANDLE,
    pub ControllerHandle: EFI_HANDLE,
    pub Attributes: UINT32,
    pub OpenCount: UINT32,
}

pub type EFI_IMAGE_ENTRY_POINT = unsafe extern "efiapi" fn(
    ImageHandle: EFI_HANDLE,
    SystemTable: EFI_SYSTEM_TABLE,
);

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

pub type EFI_BOOT_OPEN_PROTOCOL = unsafe extern "efiapi" fn(
    Handle: EFI_HANDLE,
    Protocol: *mut EFI_GUID,
    Interface: *mut *mut VOID,
    AgentHandle: EFI_HANDLE,
    ControllerHandle: EFI_HANDLE,
    Attributes: UINT32,
) -> EFI_STATUS;

pub type EFI_BOOT_CLOSE_PROTOCOL = unsafe extern "efiapi" fn(
    Handle: EFI_HANDLE,
    Protocol: *mut EFI_GUID,
    AgentHandle: EFI_HANDLE,
    ControllerHandle: EFI_HANDLE,
) -> EFI_STATUS;

pub type EFI_BOOT_OPEN_PROTOCOL_INFORMATION = unsafe extern "efiapi" fn(
    Handle: EFI_HANDLE,
    Protocol: *mut EFI_GUID,
    EntryBuffer: *mut *mut EFI_OPEN_PROTOCOL_INFORMATION_ENTRY,
    EntryCount: *mut UINTN,
) -> EFI_STATUS;

pub type EFI_BOOT_CONNECT_CONTROLLER = unsafe extern "efiapi" fn(
    ControllerHandle: EFI_HANDLE,
    DriverImageHandle: *mut EFI_HANDLE,
    RemainingDevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
    Recursive: BOOLEAN
) -> EFI_STATUS;

pub type EFI_BOOT_DISCONNECT_CONTROLLER = unsafe extern "efiapi" fn(
    ControllerHandle: EFI_HANDLE,
    DriverImageHandle: EFI_HANDLE,
    ChildHandle: EFI_HANDLE,
) -> EFI_STATUS;

pub type EFI_BOOT_PROTOCOLS_PER_HANDLE = unsafe extern "efiapi" fn(
    Handle: EFI_HANDLE,
    ProtocolBuffer: *mut *mut *mut EFI_GUID,
    ProtocolBufferCount: *mut UINTN,
) -> EFI_STATUS;

pub type EFI_BOOT_LOCATE_HANDLE_BUFFER = unsafe extern "efiapi" fn(
    SearchType: EFI_LOCATE_SEARCH_TYPE,
    Protocol: *mut EFI_GUID,
    SearchKey: *mut VOID,
    NoHandles: *mut UINTN,
    Buffer: *mut *mut EFI_HANDLE,
) -> EFI_STATUS;

pub type EFI_BOOT_LOCATE_PROTOCOL = unsafe extern "efiapi" fn(
    Protocol: *mut EFI_GUID,
    Registration: *mut VOID,
    Interface: *mut *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_INSTALL_MULTIPLE_PROTOCOL_INTERFACES = unsafe extern "efiapi" fn(
    Handle: *mut EFI_HANDLE,
    ...
) -> EFI_STATUS;


pub type EFI_BOOT_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES = unsafe extern "efiapi" fn(
    Handle: *mut EFI_HANDLE,
    ...
) -> EFI_STATUS;

pub type EFI_BOOT_IMAGE_LOAD = unsafe extern "efiapi" fn(
    BootPolicy: BOOLEAN,
    ParentImageHandle: EFI_HANDLE,
    DevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
    SourceBuffer: *mut VOID,
    SourceSize: UINTN,
    ImageHandle: *mut EFI_HANDLE,
) -> EFI_STATUS;

pub type EFI_BOOT_IMAGE_START = unsafe extern "efiapi" fn(
    ImageHandle: EFI_HANDLE,
    ExitDataSize: *mut UINTN,
    ExitData: *mut *mut CHAR16,
) -> EFI_STATUS;

pub type EFI_BOOT_IMAGE_UNLOAD = unsafe extern "efiapi" fn(
    ImageHandle: EFI_HANDLE
) -> EFI_STATUS;

pub type EFI_BOOT_EXIT = unsafe extern "efiapi" fn(
    ImageHandle: EFI_HANDLE,
    ExitStatus: EFI_STATUS,
    ExitDataSize: UINTN,
    ExitData: *mut CHAR16,
) -> EFI_STATUS;

pub type EFI_BOOT_EXIT_BOOT_SERVICES = unsafe extern "efiapi" fn(
    ImageHandle: EFI_HANDLE,
    MapKey: UINTN,
) -> EFI_STATUS;

pub type EFI_BOOT_SET_WATCHDOG_TIMER = unsafe extern "efiapi" fn(
    Timeout: UINTN,
    WatchdogCode: UINT64,
    DataSize: UINTN,
    WatchdogData: *mut CHAR16,
) -> EFI_STATUS;

pub type EFI_BOOT_STALL = unsafe extern "efiapi" fn(
    Microseconds: UINTN,
) -> EFI_STATUS;

pub type EFI_BOOT_COPY_MEM = unsafe extern "efiapi" fn(
    Destination: *mut VOID,
    Source: *mut VOID,
    Length: UINTN,
) -> EFI_STATUS;

pub type EFI_BOOT_SET_MEM = unsafe extern "efiapi" fn(
    Buffer: *mut VOID,
    Size: UINTN,
    Value: UINT8,
) -> EFI_STATUS;

pub type EFI_BOOT_GET_NEXT_MONOTONIC_COUNT = unsafe extern "efiapi" fn(
    Count: *mut UINT64
) -> EFI_STATUS;

pub type EFI_BOOT_INSTALL_CONFIGURATION_TABLE = unsafe extern "efiapi" fn(
    Guid: *mut EFI_GUID,
    Table: *mut VOID,
) -> EFI_STATUS;

pub type EFI_BOOT_CALCULATE_CRC32 = unsafe extern "efiapi" fn(
    Data: *mut VOID,
    DataSize: UINTN,
    Crc32: *mut UINT32
) -> EFI_STATUS;
