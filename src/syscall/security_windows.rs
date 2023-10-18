// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2012 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::winerror::ERROR_INSUFFICIENT_BUFFER;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winbase::LookupAccountSidW;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::LPWSTR;
use winapi::um::winnt::TOKEN_QUERY;

use super::utf16_from_string;
use super::utf16_to_string;

// package syscall

// import (
// 	"unsafe"
// )

// const (
// 	STANDARD_RIGHTS_REQUIRED = 0xf0000
// 	STANDARD_RIGHTS_READ     = 0x20000
// 	STANDARD_RIGHTS_WRITE    = 0x20000
// 	STANDARD_RIGHTS_EXECUTE  = 0x20000
// 	STANDARD_RIGHTS_ALL      = 0x1F0000
// )

// const (
// 	NameUnknown          = 0
// 	NameFullyQualifiedDN = 1
pub const NAME_SAM_COMPATIBLE: u32 = 2;
pub const NAME_DISPLAY: u32 = 3;
// 	NameUniqueId         = 6
// 	NameCanonical        = 7
// 	NameUserPrincipal    = 8
// 	NameCanonicalEx      = 9
// 	NameServicePrincipal = 10
// 	NameDnsDomain        = 12
// )

// // This function returns 1 byte BOOLEAN rather than the 4 byte BOOL.
// // https://blogs.msdn.com/b/drnick/archive/2007/12/19/windows-and-upn-format-credentials.aspx
// //sys	TranslateName(accName *uint16, accNameFormat u32, desiredNameFormat u32, translatedName *uint16, nSize *u32) (err error) [failretval&0xff==0] = secur32.TranslateNameW
extern "system" {
    pub fn TranslateNameW(
        lpAccountName: winapi::um::winnt::LPCWSTR,
        AccountNameFormat: u32,
        DesiredNameFormat: u32,
        lpTranslatedName: winapi::um::winnt::LPWSTR,
        nSize: *mut u32,
    ) -> winapi::um::winnt::BOOLEAN;
}

// //sys	GetUserNameEx(nameFormat u32, nameBuffre *uint16, nSize *u32) (err error) [failretval&0xff==0] = secur32.GetUserNameExW

/// translate_account_name converts a directory service
/// object name from one format to another.
pub fn translate_account_name(
    username: &str,
    from_format: u32,
    to_format: u32,
) -> std::io::Result<String> {
    let u = utf16_from_string(username);
    let mut n = 50_u32;
    let mut b = vec![0_u16; n as usize];
    loop {
        let e =
            unsafe { TranslateNameW(u.as_ptr(), from_format, to_format, b.as_mut_ptr(), &mut n) };
        if e == 0 {
            let last_err = std::io::Error::last_os_error();
            if last_err.raw_os_error().unwrap() == ERROR_INSUFFICIENT_BUFFER as i32 {
                b.resize(n as usize, 0);
                continue;
            }
            return Err(last_err);
        }
        return Ok(utf16_to_string(&b[..n as usize]));
    }
}

// const (
// 	// do not reorder
// 	NetSetupUnknownStatus = iota
// 	NetSetupUnjoined
// 	NetSetupWorkgroupName
// 	NetSetupDomainName
// )

// type UserInfo10 struct {
// 	Name       *uint16
// 	Comment    *uint16
// 	UsrComment *uint16
// 	FullName   *uint16
// }

// //sys	NetUserGetInfo(serverName *uint16, userName *uint16, level u32, buf **byte) (neterr error) = netapi32.NetUserGetInfo
// //sys	NetGetJoinInformation(server *uint16, name **uint16, bufType *u32) (neterr error) = netapi32.NetGetJoinInformation
// //sys	NetApiBufferFree(buf *byte) (neterr error) = netapi32.NetApiBufferFree

// const (
// 	// do not reorder
// 	SidTypeUser = 1 + iota
// 	SidTypeGroup
// 	SidTypeDomain
// 	SidTypeAlias
// 	SidTypeWellKnownGroup
// 	SidTypeDeletedAccount
// 	SidTypeInvalid
// 	SidTypeUnknown
// 	SidTypeComputer
// 	SidTypeLabel
// )

// //sys	LookupAccountSid(systemName *uint16, sid *SID, name *uint16, nameLen *u32, refdDomainName *uint16, refdDomainNameLen *u32, use *u32) (err error) = advapi32.LookupAccountSidW
// //sys	LookupAccountName(systemName *uint16, accountName *uint16, sid *SID, sidLen *u32, refdDomainName *uint16, refdDomainNameLen *u32, use *u32) (err error) = advapi32.LookupAccountNameW
// //sys	ConvertSidToStringSid(sid *SID, stringSid **uint16) (err error) = advapi32.ConvertSidToStringSidW
// //sys	ConvertStringSidToSid(stringSid *uint16, sid **SID) (err error) = advapi32.ConvertStringSidToSidW
// //sys	GetLengthSid(sid *SID) (len u32) = advapi32.GetLengthSid
// //sys	CopySid(destSidLen u32, destSid *SID, srcSid *SID) (err error) = advapi32.CopySid

// /// The security identifier (SID) structure is a variable-length
// /// structure used to uniquely identify users or groups.
// struct SID {}

// // StringToSid converts a string-format security identifier
// // sid into a valid, functional sid.
// fn StringToSid(s string) (*SID, error) {
// 	var sid *SID
// 	p, e := UTF16PtrFromString(s)
// 	if e != nil {
// 		return nil, e
// 	}
// 	e = ConvertStringSidToSid(p, &sid)
// 	if e != nil {
// 		return nil, e
// 	}
// 	defer LocalFree((Handle)(unsafe.Pointer(sid)))
// 	return sid.Copy()
// }

// // LookupSID retrieves a security identifier sid for the account
// // and the name of the domain on which the account was found.
// // System specify target computer to search.
// fn LookupSID(system, account string) (sid *SID, domain string, accType u32, err error) {
// 	if len(account) == 0 {
// 		return nil, "", 0, EINVAL
// 	}
// 	acc, e := UTF16PtrFromString(account)
// 	if e != nil {
// 		return nil, "", 0, e
// 	}
// 	var sys *uint16
// 	if len(system) > 0 {
// 		sys, e = UTF16PtrFromString(system)
// 		if e != nil {
// 			return nil, "", 0, e
// 		}
// 	}
// 	n := u32(50)
// 	dn := u32(50)
// 	for {
// 		b := make([]byte, n)
// 		db := make([]uint16, dn)
// 		sid = (*SID)(unsafe.Pointer(&b[0]))
// 		e = LookupAccountName(sys, acc, sid, &n, &db[0], &dn, &accType)
// 		if e == nil {
// 			return sid, UTF16ToString(db), accType, nil
// 		}
// 		if e != ERROR_INSUFFICIENT_BUFFER {
// 			return nil, "", 0, e
// 		}
// 		if n <= u32(len(b)) {
// 			return nil, "", 0, e
// 		}
// 	}
// }

// impl std::fmt::Display for winapi::um::winnt::PSID {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

/// String converts sid to a string format
/// suitable for display, storage, or transmission.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn sid_to_string(sid: winapi::um::winnt::PSID) -> std::io::Result<String> {
    // fn (sid *SID) String() (string, error) {

    let mut raw_string: LPWSTR = std::ptr::null_mut();
    let res = unsafe { winapi::shared::sddl::ConvertSidToStringSidW(sid, &mut raw_string) };
    if res == 0 {
        return Err(std::io::Error::last_os_error());
    }
    let s = super::utf16_ptr_to_string(raw_string);
    unsafe { winapi::um::winbase::LocalFree(raw_string as *mut winapi::ctypes::c_void) };
    Ok(s)
}

// // Len returns the length, in bytes, of a valid security identifier sid.
// fn (sid *SID) Len() isize {
// 	return isize(GetLengthSid(sid))
// }

// // Copy creates a duplicate of security identifier sid.
// fn (sid *SID) Copy() (*SID, error) {
// 	b := make([]byte, sid.Len())
// 	sid2 := (*SID)(unsafe.Pointer(&b[0]))
// 	e := CopySid(u32(len(b)), sid2, sid)
// 	if e != nil {
// 		return nil, e
// 	}
// 	return sid2, nil
// }

#[derive(Debug)]
pub struct AccontLookupResult {
    pub account: String,
    pub domain: String,
    pub acc_type: u32,
}

/// lookup_account retrieves the name of the account for this sid
/// and the name of the first domain on which this sid is found.
/// System specify target computer to search for.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn lookup_account(
    sid: winapi::um::winnt::PSID,
    system: &str,
) -> std::io::Result<AccontLookupResult> {
    let system_wide_str: Vec<u16>;
    let mut system_wide_str_ptr = std::ptr::null::<u16>();
    if !system.is_empty() {
        system_wide_str = super::utf16_from_string(system);
        system_wide_str_ptr = system_wide_str.as_ptr();
    }
    let mut n = 50_u32;
    let mut dn = 50_u32;
    let mut b = vec![0_u16; n as usize];
    let mut db = vec![0_u16; dn as usize];
    let mut acc_type = 0;
    loop {
        let ret = unsafe {
            LookupAccountSidW(
                system_wide_str_ptr,
                sid,
                b.as_mut_ptr(),
                &mut n,
                db.as_mut_ptr(),
                &mut dn,
                &mut acc_type,
            )
        };
        if ret == 0 {
            let last_err = std::io::Error::last_os_error();
            if last_err.raw_os_error().unwrap() == ERROR_INSUFFICIENT_BUFFER as i32 {
                if n as usize > b.len() || dn as usize > db.len() {
                    b.resize(n as usize, 0);
                    db.resize(dn as usize, 0);
                    continue;
                } else {
                    return Err(last_err);
                }
            }
            return Err(last_err);
        }
        return Ok(AccontLookupResult {
            account: super::utf16_to_string(&b),
            domain: super::utf16_to_string(&db),
            acc_type,
        });
    }
}

// const (
// 	// do not reorder
// 	TOKEN_ASSIGN_PRIMARY = 1 << iota
// 	TOKEN_DUPLICATE
// 	TOKEN_IMPERSONATE
// 	TOKEN_QUERY
// 	TOKEN_QUERY_SOURCE
// 	TOKEN_ADJUST_PRIVILEGES
// 	TOKEN_ADJUST_GROUPS
// 	TOKEN_ADJUST_DEFAULT
// 	TOKEN_ADJUST_SESSIONID

// 	TOKEN_ALL_ACCESS = STANDARD_RIGHTS_REQUIRED |
// 		TOKEN_ASSIGN_PRIMARY |
// 		TOKEN_DUPLICATE |
// 		TOKEN_IMPERSONATE |
// 		TOKEN_QUERY |
// 		TOKEN_QUERY_SOURCE |
// 		TOKEN_ADJUST_PRIVILEGES |
// 		TOKEN_ADJUST_GROUPS |
// 		TOKEN_ADJUST_DEFAULT |
// 		TOKEN_ADJUST_SESSIONID
// 	TOKEN_READ  = STANDARD_RIGHTS_READ | TOKEN_QUERY
// 	TOKEN_WRITE = STANDARD_RIGHTS_WRITE |
// 		TOKEN_ADJUST_PRIVILEGES |
// 		TOKEN_ADJUST_GROUPS |
// 		TOKEN_ADJUST_DEFAULT
// 	TOKEN_EXECUTE = STANDARD_RIGHTS_EXECUTE
// )

// const (
// 	// do not reorder
// 	TokenUser = 1 + iota
// 	TokenGroups
// 	TokenPrivileges
// 	TokenOwner
// 	TokenPrimaryGroup
// 	TokenDefaultDacl
// 	TokenSource
// 	TokenType
// 	TokenImpersonationLevel
// 	TokenStatistics
// 	TokenRestrictedSids
// 	TokenSessionId
// 	TokenGroupsAndPrivileges
// 	TokenSessionReference
// 	TokenSandBoxInert
// 	TokenAuditPolicy
// 	TokenOrigin
// 	TokenElevationType
// 	TokenLinkedToken
// 	TokenElevation
// 	TokenHasRestrictions
// 	TokenAccessInformation
// 	TokenVirtualizationAllowed
// 	TokenVirtualizationEnabled
// 	TokenIntegrityLevel
// 	TokenUIAccess
// 	TokenMandatoryPolicy
// 	TokenLogonSid
// 	MaxTokenInfoClass
// )

// type SIDAndAttributes struct {
// 	Sid        *SID
// 	Attributes u32
// }

#[derive(Debug)]
pub struct Tokenuser {
    raw_data: Vec<u8>,
    // actual data is winapi::um::winnt::TOKEN_USER
}

impl Tokenuser {
    fn get(&self) -> *const winapi::um::winnt::TOKEN_USER {
        self.raw_data.as_ptr() as winapi::um::winnt::PTOKEN_USER
    }

    pub fn get_sid(&self) -> winapi::um::winnt::PSID {
        let p = self.get();
        unsafe { (*p).User.Sid }
    }
}

#[derive(Debug)]
pub struct Tokenprimarygroup {
    raw_data: Vec<u8>,
    // 	PrimaryGroup *SID
    // actual data is winapi::um::winnt::TokenUser
}

impl Tokenprimarygroup {
    fn get(&self) -> *const winapi::um::winnt::TOKEN_PRIMARY_GROUP {
        self.raw_data.as_ptr() as winapi::um::winnt::PTOKEN_PRIMARY_GROUP
    }

    pub fn get_sid(&self) -> winapi::um::winnt::PSID {
        let p = self.get();
        unsafe { (*p).PrimaryGroup }
    }
}

// //sys	OpenProcessToken(h Handle, access u32, token *Token) (err error) = advapi32.OpenProcessToken
// //sys	GetTokenInformation(t Token, infoClass u32, info *byte, infoLen u32, returnedLen *u32) (err error) = advapi32.GetTokenInformation
// //sys	get_user_profile_directory(t Token, dir *uint16, dirLen *u32) (err error) = userenv.get_user_profile_directoryW

/// An access token contains the security information for a logon session.
/// The system creates an access token when a user logs on, and every
/// process executed on behalf of the user has a copy of the token.
/// The token identifies the user, the user's groups, and the user's
/// privileges. The system uses the token to control access to securable
/// objects and to control the ability of the user to perform various
/// system-related operations on the local computer.
#[derive(Debug)]
pub struct Token {
    pub h: HANDLE,
}

impl Drop for Token {
    fn drop(&mut self) {
        _ = Token::close(self);
    }
}

/// open_current_process_token opens the access token
/// associated with current process.
pub fn open_current_process_token() -> std::io::Result<Token> {
    unsafe {
        let p = GetCurrentProcess();
        let mut token: Token = Token {
            h: std::ptr::null_mut(),
        };
        let result = OpenProcessToken(p, TOKEN_QUERY, &mut token.h);
        get_error(result)?;
        Ok(token)
    }
}

impl Token {
    /// close releases access to access token.
    pub fn close(&mut self) -> std::io::Result<()> {
        unsafe { get_error(CloseHandle(self.h)) }
    }

    /// get_info retrieves a specified type of information about an access token.
    fn get_info(&self, class: u32, init_size: usize) -> std::io::Result<Vec<u8>> {
        let mut n = init_size as u32;
        let mut b = vec![0_u8; n as usize];
        loop {
            let e =
                unsafe { GetTokenInformation(self.h, class, b.as_mut_ptr() as LPVOID, n, &mut n) };
            if e == 0 {
                let last_err = std::io::Error::last_os_error();
                if last_err.raw_os_error().unwrap() == ERROR_INSUFFICIENT_BUFFER as i32 {
                    b.resize(n as usize, 0);
                    continue;
                }
                return Err(last_err);
            }
            return Ok(b);
        }
    }

    /// get_token_user retrieves access token t user account information.
    pub fn get_token_user(&self) -> std::io::Result<Tokenuser> {
        Ok(Tokenuser {
            raw_data: self.get_info(winapi::um::winnt::TokenUser, 50)?,
        })
    }

    /// get_token_primary_group retrieves access token t primary group information.
    /// A pointer to a SID structure representing a group that will become
    /// the primary group of any objects created by a process using this access token.
    pub fn get_token_primary_group(&self) -> std::io::Result<Tokenprimarygroup> {
        Ok(Tokenprimarygroup {
            raw_data: self.get_info(winapi::um::winnt::TokenPrimaryGroup, 50)?,
        })
    }

    /// get_user_profile_directory retrieves path to the
    /// root directory of the access token t user's profile.
    pub fn get_user_profile_directory(&self) -> std::io::Result<String> {
        let mut n = 100_u32;
        let mut b = vec![0_u16; n as usize];
        loop {
            let e = unsafe {
                winapi::um::userenv::GetUserProfileDirectoryW(self.h, b.as_mut_ptr(), &mut n)
            };
            if e == 0 {
                let last_err = std::io::Error::last_os_error();
                if last_err.raw_os_error().unwrap() == ERROR_INSUFFICIENT_BUFFER as i32 {
                    b.resize(n as usize, 0);
                    continue;
                }
                return Err(last_err);
            }
            return Ok(super::utf16_to_string(&b));
        }
    }
}

fn get_error(res: BOOL) -> std::io::Result<()> {
    if res == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}
