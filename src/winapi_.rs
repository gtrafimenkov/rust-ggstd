// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright (c) 2015-2018 The winapi-rs Developers
// Licensed under the terms of [MIT License](https://opensource.org/license/mit/).

// Original content of this file was copied from https://github.com/retep998/winapi-rs

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub use std::os::raw::c_void;
pub type c_uchar = u8;
pub type c_int = i32;
pub type c_ulong = u32;
pub type wchar_t = u16;

macro_rules! ENUM {
    {enum $name:ident { $($variant:ident = $value:expr,)+ }} => {
        pub type $name = u32;
        $(pub const $variant: $name = $value;)+
    };
    {enum $name:ident { $variant:ident = $value:expr, $($rest:tt)* }} => {
        pub type $name = u32;
        pub const $variant: $name = $value;
        ENUM!{@gen $name $variant, $($rest)*}
    };
    {enum $name:ident { $variant:ident, $($rest:tt)* }} => {
        ENUM!{enum $name { $variant = 0, $($rest)* }}
    };
    {@gen $name:ident $base:ident,} => {};
    {@gen $name:ident $base:ident, $variant:ident = $value:expr, $($rest:tt)*} => {
        pub const $variant: $name = $value;
        ENUM!{@gen $name $variant, $($rest)*}
    };
    {@gen $name:ident $base:ident, $variant:ident, $($rest:tt)*} => {
        pub const $variant: $name = $base + 1u32;
        ENUM!{@gen $name $variant, $($rest)*}
    };
}

macro_rules! STRUCT {
    (#[debug] $($rest:tt)*) => (
        STRUCT!{#[cfg_attr(feature = "impl-debug", derive(Debug))] $($rest)*}
    );
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] #[derive(Copy)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
    );
}

// winapi::shared::minwindef

pub type BOOL = c_int;
pub type LPVOID = *mut c_void;
pub type DWORD = c_ulong;
pub type LPDWORD = *mut DWORD;
pub type PDWORD = *mut DWORD;
pub type BYTE = c_uchar;
pub type HLOCAL = HANDLE;
pub type LPBYTE = *mut BYTE;

// winapi::um::winnt

pub type HANDLE = *mut c_void;
pub type LPCWSTR = *const WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type PHANDLE = *mut HANDLE;
pub type PSID = PVOID;
pub type PVOID = *mut c_void;
pub type WCHAR = wchar_t;
pub type BOOLEAN = BYTE;

pub type PSID_NAME_USE = *mut SID_NAME_USE;
ENUM! {enum SID_NAME_USE {
    SidTypeUser = 1,
    _SidTypeGroup,
    _SidTypeDomain,
    _SidTypeAlias,
    _SidTypeWellKnownGroup,
    _SidTypeDeletedAccount,
    _SidTypeInvalid,
    _SidTypeUnknown,
    _SidTypeComputer,
    _SidTypeLabel,
    _SidTypeLogonSession,
}}

STRUCT! {struct SID_AND_ATTRIBUTES {
    Sid: PSID,
    Attributes: DWORD,
}}

STRUCT! {struct TOKEN_USER {
    User: SID_AND_ATTRIBUTES,
}}

pub type PTOKEN_USER = *mut TOKEN_USER;

STRUCT! {struct TOKEN_PRIMARY_GROUP {
    PrimaryGroup: PSID,
}}
pub type PTOKEN_PRIMARY_GROUP = *mut TOKEN_PRIMARY_GROUP;

pub const TOKEN_QUERY: DWORD = 0x0008;

// winapi::shared::winerror

pub const ERROR_INSUFFICIENT_BUFFER: DWORD = 122;

// winapi::um::handleapi

extern "system" {
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
}

// winapi::um::processthreadsapi

extern "system" {
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn OpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: DWORD,
        TokenHandle: PHANDLE,
    ) -> BOOL;
    pub fn GetTokenInformation(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: LPVOID,
        TokenInformationLength: DWORD,
        ReturnLength: PDWORD,
    ) -> BOOL;
}

// winapi::um::securitybaseapi

ENUM! {enum TOKEN_INFORMATION_CLASS {
    TokenUser = 1,
    TokenGroups,
    TokenPrivileges,
    TokenOwner,
    TokenPrimaryGroup,
    _TokenDefaultDacl,
    _TokenSource,
    _TokenType,
    _TokenImpersonationLevel,
    _TokenStatistics,
    _TokenRestrictedSids,
    _TokenSessionId,
    _TokenGroupsAndPrivileges,
    _TokenSessionReference,
    _TokenSandBoxInert,
    _TokenAuditPolicy,
    _TokenOrigin,
    _TokenElevationType,
    _TokenLinkedToken,
    _TokenElevation,
    _TokenHasRestrictions,
    _TokenAccessInformation,
    _TokenVirtualizationAllowed,
    _TokenVirtualizationEnabled,
    _TokenIntegrityLevel,
    _TokenUIAccess,
    _TokenMandatoryPolicy,
    _TokenLogonSid,
    _TokenIsAppContainer,
    _TokenCapabilities,
    _TokenAppContainerSid,
    _TokenAppContainerNumber,
    _TokenUserClaimAttributes,
    _TokenDeviceClaimAttributes,
    _TokenRestrictedUserClaimAttributes,
    _TokenRestrictedDeviceClaimAttributes,
    _TokenDeviceGroups,
    _TokenRestrictedDeviceGroups,
    _TokenSecurityAttributes,
    _TokenIsRestricted,
    _TokenProcessTrustLevel,
    _TokenPrivateNameSpace,
    _TokenSingletonAttributes,
    _TokenBnoIsolation,
    _MaxTokenInfoClass,
}}

// winapi::um::winbase

extern "system" {
    pub fn LookupAccountSidW(
        lpSystemName: LPCWSTR,
        Sid: PSID,
        Name: LPWSTR,
        cchName: LPDWORD,
        ReferencedDomainName: LPWSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE,
    ) -> BOOL;

    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
}

// winapi::um::userenv

extern "system" {
    pub fn GetUserProfileDirectoryW(
        hToken: HANDLE,
        lpProfileDir: LPWSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
}

// winapi::shared::sddl

extern "system" {
    pub fn ConvertSidToStringSidW(Sid: PSID, StringSid: *mut LPWSTR) -> BOOL;
}

// winapi::um::lmjoin

ENUM! {enum NETSETUP_JOIN_STATUS {
    NetSetupUnknownStatus = 0,
    NetSetupUnjoined,
    NetSetupWorkgroupName,
    NetSetupDomainName,
}}
pub type PNETSETUP_JOIN_STATUS = *mut NETSETUP_JOIN_STATUS;
extern "system" {
    pub fn NetGetJoinInformation(
        lpServer: LPCWSTR,
        lpNameBuffer: *mut LPWSTR,
        BufferType: PNETSETUP_JOIN_STATUS,
    ) -> NET_API_STATUS;
}

// winapi::shared::lmcons

pub type NET_API_STATUS = DWORD;

// winapi::um::lmapibuf

extern "system" {
    pub fn NetApiBufferFree(Buffer: LPVOID) -> NET_API_STATUS;
}

// winapi::um::lmaccess

extern "system" {
    pub fn NetUserGetInfo(
        servername: LPCWSTR,
        username: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
}

STRUCT! {struct USER_INFO_10 {
    usri10_name: LPWSTR,
    usri10_comment: LPWSTR,
    usri10_usr_comment: LPWSTR,
    usri10_full_name: LPWSTR,
}}
pub type PUSER_INFO_10 = *mut USER_INFO_10;
