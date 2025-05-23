windows_targets::link!("ndfapi.dll" "system" fn NdfCancelIncident(handle : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCloseIncident(handle : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateConnectivityIncident(handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateDNSIncident(hostname : windows_sys::core::PCWSTR, querytype : u16, handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Networking_WinSock")]
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateGroupingIncident(cloudname : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, identity : windows_sys::core::PCWSTR, invitation : windows_sys::core::PCWSTR, addresses : *const super::super::Networking::WinSock:: SOCKET_ADDRESS_LIST, appid : windows_sys::core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateIncident(helperclassname : windows_sys::core::PCWSTR, celt : u32, attributes : *const HELPER_ATTRIBUTE, handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateNetConnectionIncident(handle : *mut *mut core::ffi::c_void, id : windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCreatePnrpIncident(cloudname : windows_sys::core::PCWSTR, peername : windows_sys::core::PCWSTR, diagnosepublish : windows_sys::core::BOOL, appid : windows_sys::core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateSharingIncident(uncpath : windows_sys::core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateWebIncident(url : windows_sys::core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateWebIncidentEx(url : windows_sys::core::PCWSTR, usewinhttp : windows_sys::core::BOOL, modulename : windows_sys::core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
windows_targets::link!("ndfapi.dll" "system" fn NdfCreateWinSockIncident(sock : super::super::Networking::WinSock:: SOCKET, host : windows_sys::core::PCWSTR, port : u16, appid : windows_sys::core::PCWSTR, userid : *const super::super::Security:: SID, handle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfDiagnoseIncident(handle : *const core::ffi::c_void, rootcausecount : *mut u32, rootcauses : *mut *mut RootCauseInfo, dwwait : u32, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfExecuteDiagnosis(handle : *const core::ffi::c_void, hwnd : super::super::Foundation:: HWND) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfGetTraceFile(handle : *const core::ffi::c_void, tracefilelocation : *mut windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("ndfapi.dll" "system" fn NdfRepairIncident(handle : *const core::ffi::c_void, repairex : *const RepairInfoEx, dwwait : u32) -> windows_sys::core::HRESULT);
pub type ATTRIBUTE_TYPE = i32;
pub const AT_BOOLEAN: ATTRIBUTE_TYPE = 1i32;
pub const AT_GUID: ATTRIBUTE_TYPE = 11i32;
pub const AT_INT16: ATTRIBUTE_TYPE = 4i32;
pub const AT_INT32: ATTRIBUTE_TYPE = 6i32;
pub const AT_INT64: ATTRIBUTE_TYPE = 8i32;
pub const AT_INT8: ATTRIBUTE_TYPE = 2i32;
pub const AT_INVALID: ATTRIBUTE_TYPE = 0i32;
pub const AT_LIFE_TIME: ATTRIBUTE_TYPE = 12i32;
pub const AT_OCTET_STRING: ATTRIBUTE_TYPE = 14i32;
pub const AT_SOCKADDR: ATTRIBUTE_TYPE = 13i32;
pub const AT_STRING: ATTRIBUTE_TYPE = 10i32;
pub const AT_UINT16: ATTRIBUTE_TYPE = 5i32;
pub const AT_UINT32: ATTRIBUTE_TYPE = 7i32;
pub const AT_UINT64: ATTRIBUTE_TYPE = 9i32;
pub const AT_UINT8: ATTRIBUTE_TYPE = 3i32;
pub const DF_IMPERSONATION: u32 = 2147483648u32;
pub const DF_TRACELESS: u32 = 1073741824u32;
pub type DIAGNOSIS_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIAG_SOCKADDR {
    pub family: u16,
    pub data: [i8; 126],
}
impl Default for DIAG_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_CONFIRMED: DIAGNOSIS_STATUS = 1i32;
pub const DS_DEFERRED: DIAGNOSIS_STATUS = 4i32;
pub const DS_INDETERMINATE: DIAGNOSIS_STATUS = 3i32;
pub const DS_NOT_IMPLEMENTED: DIAGNOSIS_STATUS = 0i32;
pub const DS_PASSTHROUGH: DIAGNOSIS_STATUS = 5i32;
pub const DS_REJECTED: DIAGNOSIS_STATUS = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DiagnosticsInfo {
    pub cost: i32,
    pub flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HELPER_ATTRIBUTE {
    pub pwszName: windows_sys::core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
    pub Anonymous: HELPER_ATTRIBUTE_0,
}
impl Default for HELPER_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HELPER_ATTRIBUTE_0 {
    pub Boolean: windows_sys::core::BOOL,
    pub Char: u8,
    pub Byte: u8,
    pub Short: i16,
    pub Word: u16,
    pub Int: i32,
    pub DWord: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub PWStr: windows_sys::core::PWSTR,
    pub Guid: windows_sys::core::GUID,
    pub LifeTime: LIFE_TIME,
    pub Address: DIAG_SOCKADDR,
    pub OctetString: OCTET_STRING,
}
impl Default for HELPER_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HYPOTHESIS {
    pub pwszClassName: windows_sys::core::PWSTR,
    pub pwszDescription: windows_sys::core::PWSTR,
    pub celt: u32,
    pub rgAttributes: *mut HELPER_ATTRIBUTE,
}
impl Default for HYPOTHESIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HelperAttributeInfo {
    pub pwszName: windows_sys::core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
}
impl Default for HelperAttributeInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HypothesisResult {
    pub hypothesis: HYPOTHESIS,
    pub pathStatus: DIAGNOSIS_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LIFE_TIME {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
}
pub const NDF_ADD_CAPTURE_TRACE: u32 = 1u32;
pub const NDF_APPLY_INCLUSION_LIST_FILTER: u32 = 2u32;
pub const NDF_ERROR_START: u32 = 63744u32;
pub const NDF_E_BAD_PARAM: windows_sys::core::HRESULT = 0x8008F905_u32 as _;
pub const NDF_E_CANCELLED: windows_sys::core::HRESULT = 0x8008F902_u32 as _;
pub const NDF_E_DISABLED: windows_sys::core::HRESULT = 0x8008F904_u32 as _;
pub const NDF_E_LENGTH_EXCEEDED: windows_sys::core::HRESULT = 0x8008F900_u32 as _;
pub const NDF_E_NOHELPERCLASS: windows_sys::core::HRESULT = 0x8008F901_u32 as _;
pub const NDF_E_PROBLEM_PRESENT: windows_sys::core::HRESULT = 0x8008F908_u32 as _;
pub const NDF_E_UNKNOWN: windows_sys::core::HRESULT = 0x8008F907_u32 as _;
pub const NDF_E_VALIDATION: windows_sys::core::HRESULT = 0x8008F906_u32 as _;
pub const NDF_INBOUND_FLAG_EDGETRAVERSAL: u32 = 1u32;
pub const NDF_INBOUND_FLAG_HEALTHCHECK: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl Default for OCTET_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROBLEM_TYPE = i32;
pub const PT_DOWN_STREAM_HEALTH: PROBLEM_TYPE = 4i32;
pub const PT_HIGHER_UTILIZATION: PROBLEM_TYPE = 16i32;
pub const PT_HIGH_UTILIZATION: PROBLEM_TYPE = 8i32;
pub const PT_INVALID: PROBLEM_TYPE = 0i32;
pub const PT_LOWER_HEALTH: PROBLEM_TYPE = 2i32;
pub const PT_LOW_HEALTH: PROBLEM_TYPE = 1i32;
pub const PT_UP_STREAM_UTILIZATION: PROBLEM_TYPE = 32i32;
pub const RCF_ISCONFIRMED: u32 = 2u32;
pub const RCF_ISLEAF: u32 = 1u32;
pub const RCF_ISTHIRDPARTY: u32 = 4u32;
pub type REPAIR_RISK = i32;
pub type REPAIR_SCOPE = i32;
pub type REPAIR_STATUS = i32;
pub const RF_CONTACT_ADMIN: u32 = 131072u32;
pub const RF_INFORMATION_ONLY: u32 = 33554432u32;
pub const RF_REPRO: u32 = 2097152u32;
pub const RF_RESERVED: u32 = 1073741824u32;
pub const RF_RESERVED_CA: u32 = 2147483648u32;
pub const RF_RESERVED_LNI: u32 = 65536u32;
pub const RF_SHOW_EVENTS: u32 = 8388608u32;
pub const RF_UI_ONLY: u32 = 16777216u32;
pub const RF_USER_ACTION: u32 = 268435456u32;
pub const RF_USER_CONFIRMATION: u32 = 134217728u32;
pub const RF_VALIDATE_HELPTOPIC: u32 = 4194304u32;
pub const RF_WORKAROUND: u32 = 536870912u32;
pub const RR_NORISK: REPAIR_RISK = 2i32;
pub const RR_NOROLLBACK: REPAIR_RISK = 0i32;
pub const RR_ROLLBACK: REPAIR_RISK = 1i32;
pub const RS_APPLICATION: REPAIR_SCOPE = 2i32;
pub const RS_DEFERRED: REPAIR_STATUS = 3i32;
pub const RS_NOT_IMPLEMENTED: REPAIR_STATUS = 0i32;
pub const RS_PROCESS: REPAIR_SCOPE = 3i32;
pub const RS_REPAIRED: REPAIR_STATUS = 1i32;
pub const RS_SYSTEM: REPAIR_SCOPE = 0i32;
pub const RS_UNREPAIRED: REPAIR_STATUS = 2i32;
pub const RS_USER: REPAIR_SCOPE = 1i32;
pub const RS_USER_ACTION: REPAIR_STATUS = 4i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RepairInfo {
    pub guid: windows_sys::core::GUID,
    pub pwszClassName: windows_sys::core::PWSTR,
    pub pwszDescription: windows_sys::core::PWSTR,
    pub sidType: u32,
    pub cost: i32,
    pub flags: u32,
    pub scope: REPAIR_SCOPE,
    pub risk: REPAIR_RISK,
    pub UiInfo: UiInfo,
    pub rootCauseIndex: i32,
}
impl Default for RepairInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RepairInfoEx {
    pub repair: RepairInfo,
    pub repairRank: u16,
}
impl Default for RepairInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RootCauseInfo {
    pub pwszDescription: windows_sys::core::PWSTR,
    pub rootCauseID: windows_sys::core::GUID,
    pub rootCauseFlags: u32,
    pub networkInterfaceID: windows_sys::core::GUID,
    pub pRepairs: *mut RepairInfoEx,
    pub repairCount: u16,
}
impl Default for RootCauseInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShellCommandInfo {
    pub pwszOperation: windows_sys::core::PWSTR,
    pub pwszFile: windows_sys::core::PWSTR,
    pub pwszParameters: windows_sys::core::PWSTR,
    pub pwszDirectory: windows_sys::core::PWSTR,
    pub nShowCmd: u32,
}
impl Default for ShellCommandInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UIT_DUI: UI_INFO_TYPE = 4i32;
pub const UIT_HELP_PANE: UI_INFO_TYPE = 3i32;
pub const UIT_INVALID: UI_INFO_TYPE = 0i32;
pub const UIT_NONE: UI_INFO_TYPE = 1i32;
pub const UIT_SHELL_COMMAND: UI_INFO_TYPE = 2i32;
pub type UI_INFO_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UiInfo {
    pub r#type: UI_INFO_TYPE,
    pub Anonymous: UiInfo_0,
}
impl Default for UiInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union UiInfo_0 {
    pub pwzNull: windows_sys::core::PWSTR,
    pub ShellInfo: ShellCommandInfo,
    pub pwzHelpUrl: windows_sys::core::PWSTR,
    pub pwzDui: windows_sys::core::PWSTR,
}
impl Default for UiInfo_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
