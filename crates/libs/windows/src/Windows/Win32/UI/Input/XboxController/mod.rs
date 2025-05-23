#[inline]
pub unsafe fn XInputEnable(enable: bool) {
    windows_link::link!("xinput1_4.dll" "system" fn XInputEnable(enable : windows_core::BOOL));
    unsafe { XInputEnable(enable.into()) }
}
#[inline]
pub unsafe fn XInputGetAudioDeviceIds(dwuserindex: u32, prenderdeviceid: Option<windows_core::PWSTR>, prendercount: Option<*mut u32>, pcapturedeviceid: Option<windows_core::PWSTR>, pcapturecount: Option<*mut u32>) -> u32 {
    windows_link::link!("xinput1_4.dll" "system" fn XInputGetAudioDeviceIds(dwuserindex : u32, prenderdeviceid : windows_core::PWSTR, prendercount : *mut u32, pcapturedeviceid : windows_core::PWSTR, pcapturecount : *mut u32) -> u32);
    unsafe { XInputGetAudioDeviceIds(dwuserindex, prenderdeviceid.unwrap_or(core::mem::zeroed()) as _, prendercount.unwrap_or(core::mem::zeroed()) as _, pcapturedeviceid.unwrap_or(core::mem::zeroed()) as _, pcapturecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn XInputGetBatteryInformation(dwuserindex: u32, devtype: BATTERY_DEVTYPE, pbatteryinformation: *mut XINPUT_BATTERY_INFORMATION) -> u32 {
    windows_link::link!("xinput1_4.dll" "system" fn XInputGetBatteryInformation(dwuserindex : u32, devtype : BATTERY_DEVTYPE, pbatteryinformation : *mut XINPUT_BATTERY_INFORMATION) -> u32);
    unsafe { XInputGetBatteryInformation(dwuserindex, devtype, pbatteryinformation as _) }
}
#[inline]
pub unsafe fn XInputGetCapabilities(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32 {
    windows_link::link!("xinput1_4.dll" "system" fn XInputGetCapabilities(dwuserindex : u32, dwflags : XINPUT_FLAG, pcapabilities : *mut XINPUT_CAPABILITIES) -> u32);
    unsafe { XInputGetCapabilities(dwuserindex, dwflags, pcapabilities as _) }
}
#[inline]
pub unsafe fn XInputGetKeystroke(dwuserindex: u32, dwreserved: Option<u32>, pkeystroke: *mut XINPUT_KEYSTROKE) -> u32 {
    windows_link::link!("xinput1_4.dll" "system" fn XInputGetKeystroke(dwuserindex : u32, dwreserved : u32, pkeystroke : *mut XINPUT_KEYSTROKE) -> u32);
    unsafe { XInputGetKeystroke(dwuserindex, dwreserved.unwrap_or(core::mem::zeroed()) as _, pkeystroke as _) }
}
#[inline]
pub unsafe fn XInputGetState(dwuserindex: u32, pstate: *mut XINPUT_STATE) -> u32 {
    windows_link::link!("xinput1_4.dll" "system" fn XInputGetState(dwuserindex : u32, pstate : *mut XINPUT_STATE) -> u32);
    unsafe { XInputGetState(dwuserindex, pstate as _) }
}
#[inline]
pub unsafe fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32 {
    windows_link::link!("xinput1_4.dll" "system" fn XInputSetState(dwuserindex : u32, pvibration : *const XINPUT_VIBRATION) -> u32);
    unsafe { XInputSetState(dwuserindex, pvibration) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BATTERY_DEVTYPE(pub u8);
pub const BATTERY_DEVTYPE_GAMEPAD: BATTERY_DEVTYPE = BATTERY_DEVTYPE(0u8);
pub const BATTERY_DEVTYPE_HEADSET: BATTERY_DEVTYPE = BATTERY_DEVTYPE(1u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BATTERY_LEVEL(pub u8);
pub const BATTERY_LEVEL_EMPTY: BATTERY_LEVEL = BATTERY_LEVEL(0u8);
pub const BATTERY_LEVEL_FULL: BATTERY_LEVEL = BATTERY_LEVEL(3u8);
pub const BATTERY_LEVEL_LOW: BATTERY_LEVEL = BATTERY_LEVEL(1u8);
pub const BATTERY_LEVEL_MEDIUM: BATTERY_LEVEL = BATTERY_LEVEL(2u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BATTERY_TYPE(pub u8);
pub const BATTERY_TYPE_ALKALINE: BATTERY_TYPE = BATTERY_TYPE(2u8);
pub const BATTERY_TYPE_DISCONNECTED: BATTERY_TYPE = BATTERY_TYPE(0u8);
pub const BATTERY_TYPE_NIMH: BATTERY_TYPE = BATTERY_TYPE(3u8);
pub const BATTERY_TYPE_UNKNOWN: BATTERY_TYPE = BATTERY_TYPE(255u8);
pub const BATTERY_TYPE_WIRED: BATTERY_TYPE = BATTERY_TYPE(1u8);
pub const VK_PAD_A: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22528u16);
pub const VK_PAD_B: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22529u16);
pub const VK_PAD_BACK: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22549u16);
pub const VK_PAD_DPAD_DOWN: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22545u16);
pub const VK_PAD_DPAD_LEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22546u16);
pub const VK_PAD_DPAD_RIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22547u16);
pub const VK_PAD_DPAD_UP: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22544u16);
pub const VK_PAD_LSHOULDER: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22533u16);
pub const VK_PAD_LTHUMB_DOWN: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22561u16);
pub const VK_PAD_LTHUMB_DOWNLEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22567u16);
pub const VK_PAD_LTHUMB_DOWNRIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22566u16);
pub const VK_PAD_LTHUMB_LEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22563u16);
pub const VK_PAD_LTHUMB_PRESS: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22550u16);
pub const VK_PAD_LTHUMB_RIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22562u16);
pub const VK_PAD_LTHUMB_UP: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22560u16);
pub const VK_PAD_LTHUMB_UPLEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22564u16);
pub const VK_PAD_LTHUMB_UPRIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22565u16);
pub const VK_PAD_LTRIGGER: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22534u16);
pub const VK_PAD_RSHOULDER: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22532u16);
pub const VK_PAD_RTHUMB_DOWN: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22577u16);
pub const VK_PAD_RTHUMB_DOWNLEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22583u16);
pub const VK_PAD_RTHUMB_DOWNRIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22582u16);
pub const VK_PAD_RTHUMB_LEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22579u16);
pub const VK_PAD_RTHUMB_PRESS: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22551u16);
pub const VK_PAD_RTHUMB_RIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22578u16);
pub const VK_PAD_RTHUMB_UP: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22576u16);
pub const VK_PAD_RTHUMB_UPLEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22580u16);
pub const VK_PAD_RTHUMB_UPRIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22581u16);
pub const VK_PAD_RTRIGGER: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22535u16);
pub const VK_PAD_START: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22548u16);
pub const VK_PAD_X: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22530u16);
pub const VK_PAD_Y: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22531u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_BATTERY_INFORMATION {
    pub BatteryType: BATTERY_TYPE,
    pub BatteryLevel: BATTERY_LEVEL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_CAPABILITIES {
    pub Type: XINPUT_DEVTYPE,
    pub SubType: XINPUT_DEVSUBTYPE,
    pub Flags: XINPUT_CAPABILITIES_FLAGS,
    pub Gamepad: XINPUT_GAMEPAD,
    pub Vibration: XINPUT_VIBRATION,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XINPUT_CAPABILITIES_FLAGS(pub u16);
impl XINPUT_CAPABILITIES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for XINPUT_CAPABILITIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for XINPUT_CAPABILITIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const XINPUT_CAPS_FFB_SUPPORTED: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(1u16);
pub const XINPUT_CAPS_NO_NAVIGATION: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(16u16);
pub const XINPUT_CAPS_PMD_SUPPORTED: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(8u16);
pub const XINPUT_CAPS_VOICE_SUPPORTED: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(4u16);
pub const XINPUT_CAPS_WIRELESS: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(2u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XINPUT_DEVSUBTYPE(pub u8);
pub const XINPUT_DEVSUBTYPE_ARCADE_PAD: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(19u8);
pub const XINPUT_DEVSUBTYPE_ARCADE_STICK: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(3u8);
pub const XINPUT_DEVSUBTYPE_DANCE_PAD: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(5u8);
pub const XINPUT_DEVSUBTYPE_DRUM_KIT: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(8u8);
pub const XINPUT_DEVSUBTYPE_FLIGHT_STICK: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(4u8);
pub const XINPUT_DEVSUBTYPE_GAMEPAD: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(1u8);
pub const XINPUT_DEVSUBTYPE_GUITAR: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(6u8);
pub const XINPUT_DEVSUBTYPE_GUITAR_ALTERNATE: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(7u8);
pub const XINPUT_DEVSUBTYPE_GUITAR_BASS: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(11u8);
pub const XINPUT_DEVSUBTYPE_UNKNOWN: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(0u8);
pub const XINPUT_DEVSUBTYPE_WHEEL: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(2u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XINPUT_DEVTYPE(pub u8);
pub const XINPUT_DEVTYPE_GAMEPAD: XINPUT_DEVTYPE = XINPUT_DEVTYPE(1u8);
pub const XINPUT_DLL: windows_core::PCWSTR = windows_core::w!("xinput1_4.dll");
pub const XINPUT_DLL_A: windows_core::PCSTR = windows_core::s!("xinput1_4.dll");
pub const XINPUT_DLL_W: windows_core::PCWSTR = windows_core::w!("xinput1_4.dll");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XINPUT_FLAG(pub u32);
impl XINPUT_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for XINPUT_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for XINPUT_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for XINPUT_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for XINPUT_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for XINPUT_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const XINPUT_FLAG_ALL: XINPUT_FLAG = XINPUT_FLAG(0u32);
pub const XINPUT_FLAG_GAMEPAD: XINPUT_FLAG = XINPUT_FLAG(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_GAMEPAD {
    pub wButtons: XINPUT_GAMEPAD_BUTTON_FLAGS,
    pub bLeftTrigger: u8,
    pub bRightTrigger: u8,
    pub sThumbLX: i16,
    pub sThumbLY: i16,
    pub sThumbRX: i16,
    pub sThumbRY: i16,
}
pub const XINPUT_GAMEPAD_A: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(4096u16);
pub const XINPUT_GAMEPAD_B: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(8192u16);
pub const XINPUT_GAMEPAD_BACK: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(32u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XINPUT_GAMEPAD_BUTTON_FLAGS(pub u16);
impl XINPUT_GAMEPAD_BUTTON_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const XINPUT_GAMEPAD_DPAD_DOWN: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(2u16);
pub const XINPUT_GAMEPAD_DPAD_LEFT: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(4u16);
pub const XINPUT_GAMEPAD_DPAD_RIGHT: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(8u16);
pub const XINPUT_GAMEPAD_DPAD_UP: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(1u16);
pub const XINPUT_GAMEPAD_LEFT_SHOULDER: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(256u16);
pub const XINPUT_GAMEPAD_LEFT_THUMB: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(64u16);
pub const XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(7849u16);
pub const XINPUT_GAMEPAD_RIGHT_SHOULDER: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(512u16);
pub const XINPUT_GAMEPAD_RIGHT_THUMB: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(128u16);
pub const XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(8689u16);
pub const XINPUT_GAMEPAD_START: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(16u16);
pub const XINPUT_GAMEPAD_TRIGGER_THRESHOLD: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(30u16);
pub const XINPUT_GAMEPAD_X: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(16384u16);
pub const XINPUT_GAMEPAD_Y: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(32768u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_KEYSTROKE {
    pub VirtualKey: XINPUT_VIRTUAL_KEY,
    pub Unicode: u16,
    pub Flags: XINPUT_KEYSTROKE_FLAGS,
    pub UserIndex: u8,
    pub HidCode: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XINPUT_KEYSTROKE_FLAGS(pub u16);
impl XINPUT_KEYSTROKE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for XINPUT_KEYSTROKE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for XINPUT_KEYSTROKE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const XINPUT_KEYSTROKE_KEYDOWN: XINPUT_KEYSTROKE_FLAGS = XINPUT_KEYSTROKE_FLAGS(1u16);
pub const XINPUT_KEYSTROKE_KEYUP: XINPUT_KEYSTROKE_FLAGS = XINPUT_KEYSTROKE_FLAGS(2u16);
pub const XINPUT_KEYSTROKE_REPEAT: XINPUT_KEYSTROKE_FLAGS = XINPUT_KEYSTROKE_FLAGS(4u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_STATE {
    pub dwPacketNumber: u32,
    pub Gamepad: XINPUT_GAMEPAD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_VIBRATION {
    pub wLeftMotorSpeed: u16,
    pub wRightMotorSpeed: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XINPUT_VIRTUAL_KEY(pub u16);
pub const XUSER_INDEX_ANY: u32 = 255u32;
pub const XUSER_MAX_COUNT: u32 = 4u32;
