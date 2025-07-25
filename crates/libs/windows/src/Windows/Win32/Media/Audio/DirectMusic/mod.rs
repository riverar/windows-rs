pub const CLSID_DirectMusic: windows_core::GUID = windows_core::GUID::from_u128(0x636b9f10_0c7d_11d1_95b2_0020afdc7421);
pub const CLSID_DirectMusicCollection: windows_core::GUID = windows_core::GUID::from_u128(0x480ff4b0_28b2_11d1_bef7_00c04fbf8fef);
pub const CLSID_DirectMusicSynth: windows_core::GUID = windows_core::GUID::from_u128(0x58c2b4d0_46e7_11d1_89ac_00a0c9054129);
pub const CLSID_DirectMusicSynthSink: windows_core::GUID = windows_core::GUID::from_u128(0xaec17ce3_a514_11d1_afa6_00aa0024d8b6);
pub const CLSID_DirectSoundPrivate: windows_core::GUID = windows_core::GUID::from_u128(0x11ab3ec0_25ec_11d1_a4d8_00c04fc28aca);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONNECTION {
    pub usSource: u16,
    pub usControl: u16,
    pub usDestination: u16,
    pub usTransform: u16,
    pub lScale: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONNECTIONLIST {
    pub cbSize: u32,
    pub cConnections: u32,
}
pub const CONN_DST_ATTENUATION: u32 = 1u32;
pub const CONN_DST_CENTER: u32 = 18u32;
pub const CONN_DST_CHORUS: u32 = 128u32;
pub const CONN_DST_EG1_ATTACKTIME: u32 = 518u32;
pub const CONN_DST_EG1_DECAYTIME: u32 = 519u32;
pub const CONN_DST_EG1_DELAYTIME: u32 = 523u32;
pub const CONN_DST_EG1_HOLDTIME: u32 = 524u32;
pub const CONN_DST_EG1_RELEASETIME: u32 = 521u32;
pub const CONN_DST_EG1_SHUTDOWNTIME: u32 = 525u32;
pub const CONN_DST_EG1_SUSTAINLEVEL: u32 = 522u32;
pub const CONN_DST_EG2_ATTACKTIME: u32 = 778u32;
pub const CONN_DST_EG2_DECAYTIME: u32 = 779u32;
pub const CONN_DST_EG2_DELAYTIME: u32 = 783u32;
pub const CONN_DST_EG2_HOLDTIME: u32 = 784u32;
pub const CONN_DST_EG2_RELEASETIME: u32 = 781u32;
pub const CONN_DST_EG2_SUSTAINLEVEL: u32 = 782u32;
pub const CONN_DST_FILTER_CUTOFF: u32 = 1280u32;
pub const CONN_DST_FILTER_Q: u32 = 1281u32;
pub const CONN_DST_GAIN: u32 = 1u32;
pub const CONN_DST_KEYNUMBER: u32 = 5u32;
pub const CONN_DST_LEFT: u32 = 16u32;
pub const CONN_DST_LEFTREAR: u32 = 19u32;
pub const CONN_DST_LFE_CHANNEL: u32 = 21u32;
pub const CONN_DST_LFO_FREQUENCY: u32 = 260u32;
pub const CONN_DST_LFO_STARTDELAY: u32 = 261u32;
pub const CONN_DST_NONE: u32 = 0u32;
pub const CONN_DST_PAN: u32 = 4u32;
pub const CONN_DST_PITCH: u32 = 3u32;
pub const CONN_DST_REVERB: u32 = 129u32;
pub const CONN_DST_RIGHT: u32 = 17u32;
pub const CONN_DST_RIGHTREAR: u32 = 20u32;
pub const CONN_DST_VIB_FREQUENCY: u32 = 276u32;
pub const CONN_DST_VIB_STARTDELAY: u32 = 277u32;
pub const CONN_SRC_CC1: u32 = 129u32;
pub const CONN_SRC_CC10: u32 = 138u32;
pub const CONN_SRC_CC11: u32 = 139u32;
pub const CONN_SRC_CC7: u32 = 135u32;
pub const CONN_SRC_CC91: u32 = 219u32;
pub const CONN_SRC_CC93: u32 = 221u32;
pub const CONN_SRC_CHANNELPRESSURE: u32 = 8u32;
pub const CONN_SRC_EG1: u32 = 4u32;
pub const CONN_SRC_EG2: u32 = 5u32;
pub const CONN_SRC_KEYNUMBER: u32 = 3u32;
pub const CONN_SRC_KEYONVELOCITY: u32 = 2u32;
pub const CONN_SRC_LFO: u32 = 1u32;
pub const CONN_SRC_MONOPRESSURE: u32 = 10u32;
pub const CONN_SRC_NONE: u32 = 0u32;
pub const CONN_SRC_PITCHWHEEL: u32 = 6u32;
pub const CONN_SRC_POLYPRESSURE: u32 = 7u32;
pub const CONN_SRC_VIBRATO: u32 = 9u32;
pub const CONN_TRN_CONCAVE: u32 = 1u32;
pub const CONN_TRN_CONVEX: u32 = 2u32;
pub const CONN_TRN_NONE: u32 = 0u32;
pub const CONN_TRN_SWITCH: u32 = 3u32;
pub const DAUD_CHAN10_VOICE_PRIORITY_OFFSET: u32 = 15u32;
pub const DAUD_CHAN11_VOICE_PRIORITY_OFFSET: u32 = 5u32;
pub const DAUD_CHAN12_VOICE_PRIORITY_OFFSET: u32 = 4u32;
pub const DAUD_CHAN13_VOICE_PRIORITY_OFFSET: u32 = 3u32;
pub const DAUD_CHAN14_VOICE_PRIORITY_OFFSET: u32 = 2u32;
pub const DAUD_CHAN15_VOICE_PRIORITY_OFFSET: u32 = 1u32;
pub const DAUD_CHAN16_VOICE_PRIORITY_OFFSET: u32 = 0u32;
pub const DAUD_CHAN1_VOICE_PRIORITY_OFFSET: u32 = 14u32;
pub const DAUD_CHAN2_VOICE_PRIORITY_OFFSET: u32 = 13u32;
pub const DAUD_CHAN3_VOICE_PRIORITY_OFFSET: u32 = 12u32;
pub const DAUD_CHAN4_VOICE_PRIORITY_OFFSET: u32 = 11u32;
pub const DAUD_CHAN5_VOICE_PRIORITY_OFFSET: u32 = 10u32;
pub const DAUD_CHAN6_VOICE_PRIORITY_OFFSET: u32 = 9u32;
pub const DAUD_CHAN7_VOICE_PRIORITY_OFFSET: u32 = 8u32;
pub const DAUD_CHAN8_VOICE_PRIORITY_OFFSET: u32 = 7u32;
pub const DAUD_CHAN9_VOICE_PRIORITY_OFFSET: u32 = 6u32;
pub const DAUD_CRITICAL_VOICE_PRIORITY: u32 = 4026531840u32;
pub const DAUD_HIGH_VOICE_PRIORITY: u32 = 3221225472u32;
pub const DAUD_LOW_VOICE_PRIORITY: u32 = 1073741824u32;
pub const DAUD_PERSIST_VOICE_PRIORITY: u32 = 268435456u32;
pub const DAUD_STANDARD_VOICE_PRIORITY: u32 = 2147483648u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIRECTSOUNDDEVICE_DATAFLOW(pub i32);
pub const DIRECTSOUNDDEVICE_DATAFLOW_CAPTURE: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(1i32);
pub const DIRECTSOUNDDEVICE_DATAFLOW_RENDER: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIRECTSOUNDDEVICE_TYPE(pub i32);
pub const DIRECTSOUNDDEVICE_TYPE_EMULATED: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(0i32);
pub const DIRECTSOUNDDEVICE_TYPE_VXD: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(1i32);
pub const DIRECTSOUNDDEVICE_TYPE_WDM: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DLSHEADER {
    pub cInstruments: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DLSID {
    pub ulData1: u32,
    pub usData2: u16,
    pub usData3: u16,
    pub abData4: [u8; 8],
}
impl Default for DLSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DLSID_GMInHardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f24_c364_11d1_a760_0000f875ac12);
pub const DLSID_GSInHardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f25_c364_11d1_a760_0000f875ac12);
pub const DLSID_ManufacturersID: windows_core::GUID = windows_core::GUID::from_u128(0xb03e1181_8095_11d2_a1ef_00600833dbd8);
pub const DLSID_ProductID: windows_core::GUID = windows_core::GUID::from_u128(0xb03e1182_8095_11d2_a1ef_00600833dbd8);
pub const DLSID_SampleMemorySize: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const DLSID_SamplePlaybackRate: windows_core::GUID = windows_core::GUID::from_u128(0x2a91f713_a4bf_11d2_bbdf_00600833dbd8);
pub const DLSID_SupportsDLS1: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f27_c364_11d1_a760_0000f875ac12);
pub const DLSID_SupportsDLS2: windows_core::GUID = windows_core::GUID::from_u128(0xf14599e5_4689_11d2_afa6_00aa0024d8b6);
pub const DLSID_XGInHardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f26_c364_11d1_a760_0000f875ac12);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DLSVERSION {
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
}
pub const DLS_CDL_ADD: u32 = 4u32;
pub const DLS_CDL_AND: u32 = 1u32;
pub const DLS_CDL_CONST: u32 = 16u32;
pub const DLS_CDL_DIVIDE: u32 = 7u32;
pub const DLS_CDL_EQ: u32 = 14u32;
pub const DLS_CDL_GE: u32 = 13u32;
pub const DLS_CDL_GT: u32 = 12u32;
pub const DLS_CDL_LE: u32 = 11u32;
pub const DLS_CDL_LOGICAL_AND: u32 = 8u32;
pub const DLS_CDL_LOGICAL_OR: u32 = 9u32;
pub const DLS_CDL_LT: u32 = 10u32;
pub const DLS_CDL_MULTIPLY: u32 = 6u32;
pub const DLS_CDL_NOT: u32 = 15u32;
pub const DLS_CDL_OR: u32 = 2u32;
pub const DLS_CDL_QUERY: u32 = 17u32;
pub const DLS_CDL_QUERYSUPPORTED: u32 = 18u32;
pub const DLS_CDL_SUBTRACT: u32 = 5u32;
pub const DLS_CDL_XOR: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_ARTICPARAMS {
    pub LFO: DMUS_LFOPARAMS,
    pub VolEG: DMUS_VEGPARAMS,
    pub PitchEG: DMUS_PEGPARAMS,
    pub Misc: DMUS_MSCPARAMS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_ARTICULATION {
    pub ulArt1Idx: u32,
    pub ulFirstExtCkIdx: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_ARTICULATION2 {
    pub ulArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulNextArtIdx: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_BUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidBufferFormat: windows_core::GUID,
    pub cbBuffer: u32,
}
pub const DMUS_CLOCKF_GLOBAL: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMUS_CLOCKINFO7 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: windows_core::GUID,
    pub wszDescription: [u16; 128],
}
impl Default for DMUS_CLOCKINFO7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMUS_CLOCKINFO8 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: windows_core::GUID,
    pub wszDescription: [u16; 128],
    pub dwFlags: u32,
}
impl Default for DMUS_CLOCKINFO8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DMUS_CLOCKTYPE(pub i32);
pub const DMUS_CLOCK_SYSTEM: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(0i32);
pub const DMUS_CLOCK_WAVE: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMUS_COPYRIGHT {
    pub cbSize: u32,
    pub byCopyright: [u8; 4],
}
impl Default for DMUS_COPYRIGHT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMUS_DEFAULT_SIZE_OFFSETTABLE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_DOWNLOADINFO {
    pub dwDLType: u32,
    pub dwDLId: u32,
    pub dwNumOffsetTableEntries: u32,
    pub cbSize: u32,
}
pub const DMUS_DOWNLOADINFO_INSTRUMENT: u32 = 1u32;
pub const DMUS_DOWNLOADINFO_INSTRUMENT2: u32 = 3u32;
pub const DMUS_DOWNLOADINFO_ONESHOTWAVE: u32 = 6u32;
pub const DMUS_DOWNLOADINFO_STREAMINGWAVE: u32 = 5u32;
pub const DMUS_DOWNLOADINFO_WAVE: u32 = 2u32;
pub const DMUS_DOWNLOADINFO_WAVEARTICULATION: u32 = 4u32;
pub const DMUS_EFFECT_CHORUS: u32 = 2u32;
pub const DMUS_EFFECT_DELAY: u32 = 4u32;
pub const DMUS_EFFECT_NONE: u32 = 0u32;
pub const DMUS_EFFECT_REVERB: u32 = 1u32;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct DMUS_EVENTHEADER {
    pub cbEvent: u32,
    pub dwChannelGroup: u32,
    pub rtDelta: i64,
    pub dwFlags: u32,
}
pub const DMUS_EVENT_STRUCTURED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMUS_EXTENSIONCHUNK {
    pub cbSize: u32,
    pub ulNextExtCkIdx: u32,
    pub ExtCkID: u32,
    pub byExtCk: [u8; 4],
}
impl Default for DMUS_EXTENSIONCHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_INSTRUMENT {
    pub ulPatch: u32,
    pub ulFirstRegionIdx: u32,
    pub ulGlobalArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulFlags: u32,
}
pub const DMUS_INSTRUMENT_GM_INSTRUMENT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_LFOPARAMS {
    pub pcFrequency: i32,
    pub tcDelay: i32,
    pub gcVolumeScale: i32,
    pub pcPitchScale: i32,
    pub gcMWToVolume: i32,
    pub pcMWToPitch: i32,
}
pub const DMUS_MAX_DESCRIPTION: u32 = 128u32;
pub const DMUS_MAX_DRIVER: u32 = 128u32;
pub const DMUS_MIN_DATA_SIZE: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_MSCPARAMS {
    pub ptDefaultPan: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_NOTERANGE {
    pub dwLowNote: u32,
    pub dwHighNote: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMUS_OFFSETTABLE {
    pub ulOffsetTable: [u32; 1],
}
impl Default for DMUS_OFFSETTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMUS_PC_AUDIOPATH: u32 = 1024u32;
pub const DMUS_PC_DIRECTSOUND: u32 = 128u32;
pub const DMUS_PC_DLS: u32 = 1u32;
pub const DMUS_PC_DLS2: u32 = 512u32;
pub const DMUS_PC_EXTERNAL: u32 = 2u32;
pub const DMUS_PC_GMINHARDWARE: u32 = 16u32;
pub const DMUS_PC_GSINHARDWARE: u32 = 32u32;
pub const DMUS_PC_INPUTCLASS: u32 = 0u32;
pub const DMUS_PC_MEMORYSIZEFIXED: u32 = 8u32;
pub const DMUS_PC_OUTPUTCLASS: u32 = 1u32;
pub const DMUS_PC_SHAREABLE: u32 = 256u32;
pub const DMUS_PC_SOFTWARESYNTH: u32 = 4u32;
pub const DMUS_PC_SYSTEMMEMORY: u32 = 2147483647u32;
pub const DMUS_PC_WAVE: u32 = 2048u32;
pub const DMUS_PC_XGINHARDWARE: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_PEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
    pub pcRange: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMUS_PORTCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidPort: windows_core::GUID,
    pub dwClass: u32,
    pub dwType: u32,
    pub dwMemorySize: u32,
    pub dwMaxChannelGroups: u32,
    pub dwMaxVoices: u32,
    pub dwMaxAudioChannels: u32,
    pub dwEffectFlags: u32,
    pub wszDescription: [u16; 128],
}
impl Default for DMUS_PORTCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_PORTPARAMS7 {
    pub dwSize: u32,
    pub dwValidParams: u32,
    pub dwVoices: u32,
    pub dwChannelGroups: u32,
    pub dwAudioChannels: u32,
    pub dwSampleRate: u32,
    pub dwEffectFlags: u32,
    pub fShare: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_PORTPARAMS8 {
    pub dwSize: u32,
    pub dwValidParams: u32,
    pub dwVoices: u32,
    pub dwChannelGroups: u32,
    pub dwAudioChannels: u32,
    pub dwSampleRate: u32,
    pub dwEffectFlags: u32,
    pub fShare: windows_core::BOOL,
    pub dwFeatures: u32,
}
pub const DMUS_PORTPARAMS_AUDIOCHANNELS: u32 = 4u32;
pub const DMUS_PORTPARAMS_CHANNELGROUPS: u32 = 2u32;
pub const DMUS_PORTPARAMS_EFFECTS: u32 = 32u32;
pub const DMUS_PORTPARAMS_FEATURES: u32 = 128u32;
pub const DMUS_PORTPARAMS_SAMPLERATE: u32 = 8u32;
pub const DMUS_PORTPARAMS_SHARE: u32 = 64u32;
pub const DMUS_PORTPARAMS_VOICES: u32 = 1u32;
pub const DMUS_PORT_FEATURE_AUDIOPATH: u32 = 1u32;
pub const DMUS_PORT_FEATURE_STREAMING: u32 = 2u32;
pub const DMUS_PORT_KERNEL_MODE: u32 = 2u32;
pub const DMUS_PORT_USER_MODE_SYNTH: u32 = 1u32;
pub const DMUS_PORT_WINMM_DRIVER: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMUS_REGION {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
    pub ulRegionArtIdx: u32,
    pub ulNextRegionIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub WaveLink: WAVELINK,
    pub WSMP: WSMPL,
    pub WLOOP: [WLOOP; 1],
}
impl Default for DMUS_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_SYNTHSTATS {
    pub dwSize: u32,
    pub dwValidStats: u32,
    pub dwVoices: u32,
    pub dwTotalCPU: u32,
    pub dwCPUPerVoice: u32,
    pub dwLostNotes: u32,
    pub dwFreeMemory: u32,
    pub lPeakVolume: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_SYNTHSTATS8 {
    pub dwSize: u32,
    pub dwValidStats: u32,
    pub dwVoices: u32,
    pub dwTotalCPU: u32,
    pub dwCPUPerVoice: u32,
    pub dwLostNotes: u32,
    pub dwFreeMemory: u32,
    pub lPeakVolume: i32,
    pub dwSynthMemUse: u32,
}
pub const DMUS_SYNTHSTATS_CPU_PER_VOICE: u32 = 4u32;
pub const DMUS_SYNTHSTATS_FREE_MEMORY: u32 = 32u32;
pub const DMUS_SYNTHSTATS_LOST_NOTES: u32 = 8u32;
pub const DMUS_SYNTHSTATS_PEAK_VOLUME: u32 = 16u32;
pub const DMUS_SYNTHSTATS_SYSTEMMEMORY: u32 = 2147483647u32;
pub const DMUS_SYNTHSTATS_TOTAL_CPU: u32 = 2u32;
pub const DMUS_SYNTHSTATS_VOICES: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_VEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_VOICE_STATE {
    pub bExists: windows_core::BOOL,
    pub spPosition: u64,
}
pub const DMUS_VOLUME_MAX: u32 = 2000u32;
pub const DMUS_VOLUME_MIN: i32 = -20000i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DMUS_WAVE {
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulWaveDataIdx: u32,
    pub WaveformatEx: super::WAVEFORMATEX,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_WAVEARTDL {
    pub ulDownloadIdIdx: u32,
    pub ulBus: u32,
    pub ulBuffers: u32,
    pub ulMasterDLId: u32,
    pub usOptions: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMUS_WAVEDATA {
    pub cbSize: u32,
    pub byData: [u8; 4],
}
impl Default for DMUS_WAVEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_WAVEDL {
    pub cbWaveData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMUS_WAVES_REVERB_PARAMS {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
pub const DSBUSID_BACK_CENTER: u32 = 8u32;
pub const DSBUSID_BACK_LEFT: u32 = 4u32;
pub const DSBUSID_BACK_RIGHT: u32 = 5u32;
pub const DSBUSID_CHORUS_SEND: u32 = 65u32;
pub const DSBUSID_DYNAMIC_0: u32 = 512u32;
pub const DSBUSID_FIRST_SPKR_LOC: u32 = 0u32;
pub const DSBUSID_FRONT_CENTER: u32 = 2u32;
pub const DSBUSID_FRONT_LEFT: u32 = 0u32;
pub const DSBUSID_FRONT_LEFT_OF_CENTER: u32 = 6u32;
pub const DSBUSID_FRONT_RIGHT: u32 = 1u32;
pub const DSBUSID_FRONT_RIGHT_OF_CENTER: u32 = 7u32;
pub const DSBUSID_LAST_SPKR_LOC: u32 = 17u32;
pub const DSBUSID_LEFT: u32 = 0u32;
pub const DSBUSID_LOW_FREQUENCY: u32 = 3u32;
pub const DSBUSID_NULL: u32 = 4294967295u32;
pub const DSBUSID_REVERB_SEND: u32 = 64u32;
pub const DSBUSID_RIGHT: u32 = 1u32;
pub const DSBUSID_SIDE_LEFT: u32 = 9u32;
pub const DSBUSID_SIDE_RIGHT: u32 = 10u32;
pub const DSBUSID_TOP_BACK_CENTER: u32 = 16u32;
pub const DSBUSID_TOP_BACK_LEFT: u32 = 15u32;
pub const DSBUSID_TOP_BACK_RIGHT: u32 = 17u32;
pub const DSBUSID_TOP_CENTER: u32 = 11u32;
pub const DSBUSID_TOP_FRONT_CENTER: u32 = 13u32;
pub const DSBUSID_TOP_FRONT_LEFT: u32 = 12u32;
pub const DSBUSID_TOP_FRONT_RIGHT: u32 = 14u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE(pub i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    pub DeviceId: windows_core::GUID,
    pub DescriptionA: [i8; 256],
    pub DescriptionW: [u16; 256],
    pub ModuleA: [i8; 260],
    pub ModuleW: [u16; 260],
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub WaveDeviceId: u32,
    pub Devnode: u32,
}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: windows_core::GUID,
    pub Description: windows_core::PSTR,
    pub Module: windows_core::PSTR,
    pub Interface: windows_core::PSTR,
    pub WaveDeviceId: u32,
}
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(6i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: windows_core::GUID,
    pub Description: windows_core::PWSTR,
    pub Module: windows_core::PWSTR,
    pub Interface: windows_core::PWSTR,
    pub WaveDeviceId: u32,
}
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1,
    pub Context: *mut core::ffi::c_void,
}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA,
    pub Context: *mut core::ffi::c_void,
}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(8i32);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW,
    pub Context: *mut core::ffi::c_void,
}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    pub DeviceName: windows_core::PSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: windows_core::GUID,
}
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    pub DeviceName: windows_core::PWSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: windows_core::GUID,
}
pub const DSPROPSETID_DirectSoundDevice: windows_core::GUID = windows_core::GUID::from_u128(0x84624f82_25ec_11d1_a4d8_00c04fc28aca);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVAudInfo {
    pub bAudStyle: [u8; 2],
    pub bAudQu: [u8; 2],
    pub bNumAudPin: u8,
    pub wAvgSamplesPerPinPerFrm: [u16; 2],
    pub wBlkMode: u16,
    pub wDIFMode: u16,
    pub wBlkDiv: u16,
}
impl Default for DVAudInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DV_AUDIOMODE: u32 = 3840u32;
pub const DV_AUDIOQU: u32 = 117440512u32;
pub const DV_AUDIOSMP: u32 = 939524096u32;
pub const DV_CAP_AUD12Bits: u32 = 1u32;
pub const DV_CAP_AUD16Bits: u32 = 0u32;
pub const DV_DVSD_NTSC_FRAMESIZE: i32 = 120000i32;
pub const DV_DVSD_PAL_FRAMESIZE: i32 = 144000i32;
pub const DV_HD: u32 = 1u32;
pub const DV_NTSC: u32 = 0u32;
pub const DV_NTSCPAL: u32 = 2097152u32;
pub const DV_PAL: u32 = 1u32;
pub const DV_SD: u32 = 0u32;
pub const DV_SL: u32 = 2u32;
pub const DV_SMCHN: u32 = 57344u32;
pub const DV_STYPE: u32 = 2031616u32;
pub const F_INSTRUMENT_DRUMS: u32 = 2147483648u32;
pub const F_RGN_OPTION_SELFNONEXCLUSIVE: u32 = 1u32;
pub const F_WAVELINK_MULTICHANNEL: u32 = 2u32;
pub const F_WAVELINK_PHASE_MASTER: u32 = 1u32;
pub const F_WSMP_NO_COMPRESSION: i32 = 2i32;
pub const F_WSMP_NO_TRUNCATION: i32 = 1i32;
pub const GUID_DMUS_PROP_DLS1: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f27_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_DLS2: windows_core::GUID = windows_core::GUID::from_u128(0xf14599e5_4689_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_Effects: windows_core::GUID = windows_core::GUID::from_u128(0xcda8d611_684a_11d2_871e_00600893b1bd);
pub const GUID_DMUS_PROP_GM_Hardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f24_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_GS_Capable: windows_core::GUID = windows_core::GUID::from_u128(0x6496aba2_61b0_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_GS_Hardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f25_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_INSTRUMENT2: windows_core::GUID = windows_core::GUID::from_u128(0x865fd372_9f67_11d2_872a_00600893b1bd);
pub const GUID_DMUS_PROP_LegacyCaps: windows_core::GUID = windows_core::GUID::from_u128(0xcfa7cdc2_00a1_11d2_aad5_0000f875ac12);
pub const GUID_DMUS_PROP_MemorySize: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_SampleMemorySize: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_SamplePlaybackRate: windows_core::GUID = windows_core::GUID::from_u128(0x2a91f713_a4bf_11d2_bbdf_00600833dbd8);
pub const GUID_DMUS_PROP_SetSynthSink: windows_core::GUID = windows_core::GUID::from_u128(0x0a3a5ba5_37b6_11d2_b9f9_0000f875ac12);
pub const GUID_DMUS_PROP_SinkUsesDSound: windows_core::GUID = windows_core::GUID::from_u128(0xbe208857_8952_11d2_ba1c_0000f875ac12);
pub const GUID_DMUS_PROP_SynthSink_DSOUND: windows_core::GUID = windows_core::GUID::from_u128(0x0aa97844_c877_11d1_870c_00600893b1bd);
pub const GUID_DMUS_PROP_SynthSink_WAVE: windows_core::GUID = windows_core::GUID::from_u128(0x0aa97845_c877_11d1_870c_00600893b1bd);
pub const GUID_DMUS_PROP_Volume: windows_core::GUID = windows_core::GUID::from_u128(0xfedfae25_e46e_11d1_aace_0000f875ac12);
pub const GUID_DMUS_PROP_WavesReverb: windows_core::GUID = windows_core::GUID::from_u128(0x04cb5622_32e5_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_WriteLatency: windows_core::GUID = windows_core::GUID::from_u128(0x268a0fa0_60f2_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_WritePeriod: windows_core::GUID = windows_core::GUID::from_u128(0x268a0fa1_60f2_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_XG_Capable: windows_core::GUID = windows_core::GUID::from_u128(0x6496aba1_61b0_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_XG_Hardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f26_c364_11d1_a760_0000f875ac12);
windows_core::imp::define_interface!(IDirectMusic, IDirectMusic_Vtbl, 0x6536115a_7b2d_11d2_ba18_0000f875ac12);
windows_core::imp::interface_hierarchy!(IDirectMusic, windows_core::IUnknown);
impl IDirectMusic {
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumPort)(windows_core::Interface::as_raw(self), dwindex, pportcaps as _).ok() }
    }
    pub unsafe fn CreateMusicBuffer<P2>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut Option<IDirectMusicBuffer>, punkouter: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateMusicBuffer)(windows_core::Interface::as_raw(self), pbufferdesc as _, core::mem::transmute(ppbuffer), punkouter.param().abi()).ok() }
    }
    pub unsafe fn CreatePort<P3>(&self, rclsidport: *const windows_core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut Option<IDirectMusicPort>, punkouter: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePort)(windows_core::Interface::as_raw(self), rclsidport, pportparams as _, core::mem::transmute(ppport), punkouter.param().abi()).ok() }
    }
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumMasterClock)(windows_core::Interface::as_raw(self), dwindex, lpclockinfo as _).ok() }
    }
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut windows_core::GUID, ppreferenceclock: *mut Option<super::super::IReferenceClock>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMasterClock)(windows_core::Interface::as_raw(self), pguidclock as _, core::mem::transmute(ppreferenceclock)).ok() }
    }
    pub unsafe fn SetMasterClock(&self, rguidclock: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMasterClock)(windows_core::Interface::as_raw(self), rguidclock).ok() }
    }
    pub unsafe fn Activate(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDefaultPort)(windows_core::Interface::as_raw(self), pguidport as _).ok() }
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<P0>(&self, pdirectsound: P0, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectSound::IDirectSound>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDirectSound)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), hwnd).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumPort: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DMUS_PORTCAPS) -> windows_core::HRESULT,
    pub CreateMusicBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_BUFFERDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePort: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DMUS_PORTPARAMS8, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DMUS_CLOCKINFO8) -> windows_core::HRESULT,
    pub GetMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetDefaultPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub SetDirectSound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))]
    SetDirectSound: usize,
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub trait IDirectMusic_Impl: windows_core::IUnknownImpl {
    fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()>;
    fn CreateMusicBuffer(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: windows_core::OutRef<'_, IDirectMusicBuffer>, punkouter: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePort(&self, rclsidport: *const windows_core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: windows_core::OutRef<'_, IDirectMusicPort>, punkouter: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> windows_core::Result<()>;
    fn GetMasterClock(&self, pguidclock: *mut windows_core::GUID, ppreferenceclock: windows_core::OutRef<'_, super::super::IReferenceClock>) -> windows_core::Result<()>;
    fn SetMasterClock(&self, rguidclock: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Activate(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDefaultPort(&self, pguidport: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: windows_core::Ref<'_, super::DirectSound::IDirectSound>, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl IDirectMusic_Vtbl {
    pub const fn new<Identity: IDirectMusic_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumPort<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::EnumPort(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pportcaps)).into()
            }
        }
        unsafe extern "system" fn CreateMusicBuffer<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::CreateMusicBuffer(this, core::mem::transmute_copy(&pbufferdesc), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn CreatePort<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsidport: *const windows_core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::CreatePort(this, core::mem::transmute_copy(&rclsidport), core::mem::transmute_copy(&pportparams), core::mem::transmute_copy(&ppport), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn EnumMasterClock<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::EnumMasterClock(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&lpclockinfo)).into()
            }
        }
        unsafe extern "system" fn GetMasterClock<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidclock: *mut windows_core::GUID, ppreferenceclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::GetMasterClock(this, core::mem::transmute_copy(&pguidclock), core::mem::transmute_copy(&ppreferenceclock)).into()
            }
        }
        unsafe extern "system" fn SetMasterClock<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidclock: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::SetMasterClock(this, core::mem::transmute_copy(&rguidclock)).into()
            }
        }
        unsafe extern "system" fn Activate<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::Activate(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn GetDefaultPort<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidport: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::GetDefaultPort(this, core::mem::transmute_copy(&pguidport)).into()
            }
        }
        unsafe extern "system" fn SetDirectSound<Identity: IDirectMusic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic_Impl::SetDirectSound(this, core::mem::transmute_copy(&pdirectsound), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumPort: EnumPort::<Identity, OFFSET>,
            CreateMusicBuffer: CreateMusicBuffer::<Identity, OFFSET>,
            CreatePort: CreatePort::<Identity, OFFSET>,
            EnumMasterClock: EnumMasterClock::<Identity, OFFSET>,
            GetMasterClock: GetMasterClock::<Identity, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            GetDefaultPort: GetDefaultPort::<Identity, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusic as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl windows_core::RuntimeName for IDirectMusic {}
windows_core::imp::define_interface!(IDirectMusic8, IDirectMusic8_Vtbl, 0x2d3629f7_813d_4939_8508_f05c6b75fd97);
impl core::ops::Deref for IDirectMusic8 {
    type Target = IDirectMusic;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusic8, windows_core::IUnknown, IDirectMusic);
impl IDirectMusic8 {
    pub unsafe fn SetExternalMasterClock<P0>(&self, pclock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::IReferenceClock>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetExternalMasterClock)(windows_core::Interface::as_raw(self), pclock.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic8_Vtbl {
    pub base__: IDirectMusic_Vtbl,
    pub SetExternalMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub trait IDirectMusic8_Impl: IDirectMusic_Impl {
    fn SetExternalMasterClock(&self, pclock: windows_core::Ref<'_, super::super::IReferenceClock>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl IDirectMusic8_Vtbl {
    pub const fn new<Identity: IDirectMusic8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetExternalMasterClock<Identity: IDirectMusic8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclock: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusic8_Impl::SetExternalMasterClock(this, core::mem::transmute_copy(&pclock)).into()
            }
        }
        Self { base__: IDirectMusic_Vtbl::new::<Identity, OFFSET>(), SetExternalMasterClock: SetExternalMasterClock::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusic8 as windows_core::Interface>::IID || iid == &<IDirectMusic as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl windows_core::RuntimeName for IDirectMusic8 {}
windows_core::imp::define_interface!(IDirectMusicBuffer, IDirectMusicBuffer_Vtbl, 0xd2ac2878_b39b_11d1_8704_00600893b1bd);
windows_core::imp::interface_hierarchy!(IDirectMusicBuffer, windows_core::IUnknown);
impl IDirectMusicBuffer {
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn TotalTime(&self, prttime: *mut i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TotalTime)(windows_core::Interface::as_raw(self), prttime as _).ok() }
    }
    pub unsafe fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PackStructured)(windows_core::Interface::as_raw(self), rt, dwchannelgroup, dwchannelmessage).ok() }
    }
    pub unsafe fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PackUnstructured)(windows_core::Interface::as_raw(self), rt, dwchannelgroup, cb, lpb as _).ok() }
    }
    pub unsafe fn ResetReadPtr(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetReadPtr)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNextEvent)(windows_core::Interface::as_raw(self), prt as _, pdwchannelgroup as _, pdwlength as _, ppdata as _).ok() }
    }
    pub unsafe fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRawBufferPtr)(windows_core::Interface::as_raw(self), ppdata as _).ok() }
    }
    pub unsafe fn GetStartTime(&self, prt: *mut i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStartTime)(windows_core::Interface::as_raw(self), prt as _).ok() }
    }
    pub unsafe fn GetUsedBytes(&self, pcb: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetUsedBytes)(windows_core::Interface::as_raw(self), pcb as _).ok() }
    }
    pub unsafe fn GetMaxBytes(&self, pcb: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMaxBytes)(windows_core::Interface::as_raw(self), pcb as _).ok() }
    }
    pub unsafe fn GetBufferFormat(&self, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBufferFormat)(windows_core::Interface::as_raw(self), pguidformat as _).ok() }
    }
    pub unsafe fn SetStartTime(&self, rt: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), rt).ok() }
    }
    pub unsafe fn SetUsedBytes(&self, cb: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUsedBytes)(windows_core::Interface::as_raw(self), cb).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub PackStructured: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, u32) -> windows_core::HRESULT,
    pub PackUnstructured: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, u32, *mut u8) -> windows_core::HRESULT,
    pub ResetReadPtr: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64, *mut u32, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetRawBufferPtr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
    pub GetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetUsedBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaxBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBufferFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetUsedBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDirectMusicBuffer_Impl: windows_core::IUnknownImpl {
    fn Flush(&self) -> windows_core::Result<()>;
    fn TotalTime(&self, prttime: *mut i64) -> windows_core::Result<()>;
    fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> windows_core::Result<()>;
    fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> windows_core::Result<()>;
    fn ResetReadPtr(&self) -> windows_core::Result<()>;
    fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> windows_core::Result<()>;
    fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> windows_core::Result<()>;
    fn GetStartTime(&self, prt: *mut i64) -> windows_core::Result<()>;
    fn GetUsedBytes(&self, pcb: *mut u32) -> windows_core::Result<()>;
    fn GetMaxBytes(&self, pcb: *mut u32) -> windows_core::Result<()>;
    fn GetBufferFormat(&self, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetStartTime(&self, rt: i64) -> windows_core::Result<()>;
    fn SetUsedBytes(&self, cb: u32) -> windows_core::Result<()>;
}
impl IDirectMusicBuffer_Vtbl {
    pub const fn new<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Flush<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::Flush(this).into()
            }
        }
        unsafe extern "system" fn TotalTime<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prttime: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::TotalTime(this, core::mem::transmute_copy(&prttime)).into()
            }
        }
        unsafe extern "system" fn PackStructured<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::PackStructured(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannelmessage)).into()
            }
        }
        unsafe extern "system" fn PackUnstructured<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::PackUnstructured(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&lpb)).into()
            }
        }
        unsafe extern "system" fn ResetReadPtr<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::ResetReadPtr(this).into()
            }
        }
        unsafe extern "system" fn GetNextEvent<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::GetNextEvent(this, core::mem::transmute_copy(&prt), core::mem::transmute_copy(&pdwchannelgroup), core::mem::transmute_copy(&pdwlength), core::mem::transmute_copy(&ppdata)).into()
            }
        }
        unsafe extern "system" fn GetRawBufferPtr<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::GetRawBufferPtr(this, core::mem::transmute_copy(&ppdata)).into()
            }
        }
        unsafe extern "system" fn GetStartTime<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prt: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::GetStartTime(this, core::mem::transmute_copy(&prt)).into()
            }
        }
        unsafe extern "system" fn GetUsedBytes<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::GetUsedBytes(this, core::mem::transmute_copy(&pcb)).into()
            }
        }
        unsafe extern "system" fn GetMaxBytes<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::GetMaxBytes(this, core::mem::transmute_copy(&pcb)).into()
            }
        }
        unsafe extern "system" fn GetBufferFormat<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::GetBufferFormat(this, core::mem::transmute_copy(&pguidformat)).into()
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::SetStartTime(this, core::mem::transmute_copy(&rt)).into()
            }
        }
        unsafe extern "system" fn SetUsedBytes<Identity: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicBuffer_Impl::SetUsedBytes(this, core::mem::transmute_copy(&cb)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Flush: Flush::<Identity, OFFSET>,
            TotalTime: TotalTime::<Identity, OFFSET>,
            PackStructured: PackStructured::<Identity, OFFSET>,
            PackUnstructured: PackUnstructured::<Identity, OFFSET>,
            ResetReadPtr: ResetReadPtr::<Identity, OFFSET>,
            GetNextEvent: GetNextEvent::<Identity, OFFSET>,
            GetRawBufferPtr: GetRawBufferPtr::<Identity, OFFSET>,
            GetStartTime: GetStartTime::<Identity, OFFSET>,
            GetUsedBytes: GetUsedBytes::<Identity, OFFSET>,
            GetMaxBytes: GetMaxBytes::<Identity, OFFSET>,
            GetBufferFormat: GetBufferFormat::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            SetUsedBytes: SetUsedBytes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicBuffer {}
windows_core::imp::define_interface!(IDirectMusicCollection, IDirectMusicCollection_Vtbl, 0xd2ac287c_b39b_11d1_8704_00600893b1bd);
windows_core::imp::interface_hierarchy!(IDirectMusicCollection, windows_core::IUnknown);
impl IDirectMusicCollection {
    pub unsafe fn GetInstrument(&self, dwpatch: u32) -> windows_core::Result<IDirectMusicInstrument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstrument)(windows_core::Interface::as_raw(self), dwpatch, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumInstrument<P2>(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: P2, dwnamelen: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumInstrument)(windows_core::Interface::as_raw(self), dwindex, pdwpatch as _, pwszname.param().abi(), dwnamelen).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInstrument: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumInstrument: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait IDirectMusicCollection_Impl: windows_core::IUnknownImpl {
    fn GetInstrument(&self, dwpatch: u32) -> windows_core::Result<IDirectMusicInstrument>;
    fn EnumInstrument(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: &windows_core::PCWSTR, dwnamelen: u32) -> windows_core::Result<()>;
}
impl IDirectMusicCollection_Vtbl {
    pub const fn new<Identity: IDirectMusicCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInstrument<Identity: IDirectMusicCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpatch: u32, ppinstrument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectMusicCollection_Impl::GetInstrument(this, core::mem::transmute_copy(&dwpatch)) {
                    Ok(ok__) => {
                        ppinstrument.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumInstrument<Identity: IDirectMusicCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pdwpatch: *mut u32, pwszname: windows_core::PCWSTR, dwnamelen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicCollection_Impl::EnumInstrument(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pdwpatch), core::mem::transmute(&pwszname), core::mem::transmute_copy(&dwnamelen)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInstrument: GetInstrument::<Identity, OFFSET>,
            EnumInstrument: EnumInstrument::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicCollection {}
windows_core::imp::define_interface!(IDirectMusicDownload, IDirectMusicDownload_Vtbl, 0xd2ac287b_b39b_11d1_8704_00600893b1bd);
windows_core::imp::interface_hierarchy!(IDirectMusicDownload, windows_core::IUnknown);
impl IDirectMusicDownload {
    pub unsafe fn GetBuffer(&self, ppvbuffer: *mut *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), ppvbuffer as _, pdwsize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownload_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDirectMusicDownload_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, ppvbuffer: *mut *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::Result<()>;
}
impl IDirectMusicDownload_Vtbl {
    pub const fn new<Identity: IDirectMusicDownload_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IDirectMusicDownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvbuffer: *mut *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicDownload_Impl::GetBuffer(this, core::mem::transmute_copy(&ppvbuffer), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBuffer: GetBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicDownload as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicDownload {}
windows_core::imp::define_interface!(IDirectMusicDownloadedInstrument, IDirectMusicDownloadedInstrument_Vtbl, 0xd2ac287e_b39b_11d1_8704_00600893b1bd);
windows_core::imp::interface_hierarchy!(IDirectMusicDownloadedInstrument, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownloadedInstrument_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDirectMusicDownloadedInstrument_Impl: windows_core::IUnknownImpl {}
impl IDirectMusicDownloadedInstrument_Vtbl {
    pub const fn new<Identity: IDirectMusicDownloadedInstrument_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicDownloadedInstrument as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicDownloadedInstrument {}
windows_core::imp::define_interface!(IDirectMusicInstrument, IDirectMusicInstrument_Vtbl, 0xd2ac287d_b39b_11d1_8704_00600893b1bd);
windows_core::imp::interface_hierarchy!(IDirectMusicInstrument, windows_core::IUnknown);
impl IDirectMusicInstrument {
    pub unsafe fn GetPatch(&self, pdwpatch: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPatch)(windows_core::Interface::as_raw(self), pdwpatch as _).ok() }
    }
    pub unsafe fn SetPatch(&self, dwpatch: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPatch)(windows_core::Interface::as_raw(self), dwpatch).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicInstrument_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetPatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDirectMusicInstrument_Impl: windows_core::IUnknownImpl {
    fn GetPatch(&self, pdwpatch: *mut u32) -> windows_core::Result<()>;
    fn SetPatch(&self, dwpatch: u32) -> windows_core::Result<()>;
}
impl IDirectMusicInstrument_Vtbl {
    pub const fn new<Identity: IDirectMusicInstrument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPatch<Identity: IDirectMusicInstrument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpatch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicInstrument_Impl::GetPatch(this, core::mem::transmute_copy(&pdwpatch)).into()
            }
        }
        unsafe extern "system" fn SetPatch<Identity: IDirectMusicInstrument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpatch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicInstrument_Impl::SetPatch(this, core::mem::transmute_copy(&dwpatch)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPatch: GetPatch::<Identity, OFFSET>, SetPatch: SetPatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicInstrument as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicInstrument {}
windows_core::imp::define_interface!(IDirectMusicPort, IDirectMusicPort_Vtbl, 0x08f2d8c9_37c2_11d2_b9f9_0000f875ac12);
windows_core::imp::interface_hierarchy!(IDirectMusicPort, windows_core::IUnknown);
impl IDirectMusicPort {
    pub unsafe fn PlayBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).PlayBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok() }
    }
    pub unsafe fn SetReadNotificationHandle(&self, hevent: super::super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReadNotificationHandle)(windows_core::Interface::as_raw(self), hevent).ok() }
    }
    pub unsafe fn Read<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok() }
    }
    pub unsafe fn DownloadInstrument<P0>(&self, pinstrument: P0, ppdownloadedinstrument: *mut Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicInstrument>,
    {
        unsafe { (windows_core::Interface::vtable(self).DownloadInstrument)(windows_core::Interface::as_raw(self), pinstrument.param().abi(), core::mem::transmute(ppdownloadedinstrument), pnoteranges as _, dwnumnoteranges).ok() }
    }
    pub unsafe fn UnloadInstrument<P0>(&self, pdownloadedinstrument: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicDownloadedInstrument>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnloadInstrument)(windows_core::Interface::as_raw(self), pdownloadedinstrument.param().abi()).ok() }
    }
    pub unsafe fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLatencyClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRunningStats)(windows_core::Interface::as_raw(self), pstats as _).ok() }
    }
    pub unsafe fn Compact(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pportcaps as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceIoControl)(windows_core::Interface::as_raw(self), dwiocontrolcode, lpinbuffer as _, ninbuffersize, lpoutbuffer as _, noutbuffersize, lpbytesreturned as _, lpoverlapped as _).ok() }
    }
    pub unsafe fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNumChannelGroups)(windows_core::Interface::as_raw(self), dwchannelgroups).ok() }
    }
    pub unsafe fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNumChannelGroups)(windows_core::Interface::as_raw(self), pdwchannelgroups as _).ok() }
    }
    pub unsafe fn Activate(&self, factive: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), factive.into()).ok() }
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelPriority)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, dwpriority).ok() }
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetChannelPriority)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, pdwpriority as _).ok() }
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<P0, P1>(&self, pdirectsound: P0, pdirectsoundbuffer: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectSound::IDirectSound>,
        P1: windows_core::Param<super::DirectSound::IDirectSoundBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDirectSound)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), pdirectsoundbuffer.param().abi()).ok() }
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pwaveformatex as _, pdwwaveformatexsize as _, pdwbuffersize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPort_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PlayBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReadNotificationHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DownloadInstrument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut DMUS_NOTERANGE, u32) -> windows_core::HRESULT,
    pub UnloadInstrument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRunningStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_SYNTHSTATS) -> windows_core::HRESULT,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_PORTCAPS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_IO")]
    pub DeviceIoControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32, *mut super::super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    DeviceIoControl: usize,
    pub SetNumChannelGroups: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNumChannelGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetChannelPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetChannelPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub SetDirectSound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))]
    SetDirectSound: usize,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WAVEFORMATEX, *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
pub trait IDirectMusicPort_Impl: windows_core::IUnknownImpl {
    fn PlayBuffer(&self, pbuffer: windows_core::Ref<'_, IDirectMusicBuffer>) -> windows_core::Result<()>;
    fn SetReadNotificationHandle(&self, hevent: super::super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn Read(&self, pbuffer: windows_core::Ref<'_, IDirectMusicBuffer>) -> windows_core::Result<()>;
    fn DownloadInstrument(&self, pinstrument: windows_core::Ref<'_, IDirectMusicInstrument>, ppdownloadedinstrument: windows_core::OutRef<'_, IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> windows_core::Result<()>;
    fn UnloadInstrument(&self, pdownloadedinstrument: windows_core::Ref<'_, IDirectMusicDownloadedInstrument>) -> windows_core::Result<()>;
    fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock>;
    fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::Result<()>;
    fn Compact(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> windows_core::Result<()>;
    fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> windows_core::Result<()>;
    fn Activate(&self, factive: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::Result<()>;
    fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: windows_core::Ref<'_, super::DirectSound::IDirectSound>, pdirectsoundbuffer: windows_core::Ref<'_, super::DirectSound::IDirectSoundBuffer>) -> windows_core::Result<()>;
    fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
impl IDirectMusicPort_Vtbl {
    pub const fn new<Identity: IDirectMusicPort_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PlayBuffer<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::PlayBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn SetReadNotificationHandle<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::SetReadNotificationHandle(this, core::mem::transmute_copy(&hevent)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::Read(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn DownloadInstrument<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstrument: *mut core::ffi::c_void, ppdownloadedinstrument: *mut *mut core::ffi::c_void, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::DownloadInstrument(this, core::mem::transmute_copy(&pinstrument), core::mem::transmute_copy(&ppdownloadedinstrument), core::mem::transmute_copy(&pnoteranges), core::mem::transmute_copy(&dwnumnoteranges)).into()
            }
        }
        unsafe extern "system" fn UnloadInstrument<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdownloadedinstrument: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::UnloadInstrument(this, core::mem::transmute_copy(&pdownloadedinstrument)).into()
            }
        }
        unsafe extern "system" fn GetLatencyClock<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectMusicPort_Impl::GetLatencyClock(this) {
                    Ok(ok__) => {
                        ppclock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRunningStats<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::GetRunningStats(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn Compact<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::Compact(this).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::GetCaps(this, core::mem::transmute_copy(&pportcaps)).into()
            }
        }
        unsafe extern "system" fn DeviceIoControl<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *mut core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::DeviceIoControl(this, core::mem::transmute_copy(&dwiocontrolcode), core::mem::transmute_copy(&lpinbuffer), core::mem::transmute_copy(&ninbuffersize), core::mem::transmute_copy(&lpoutbuffer), core::mem::transmute_copy(&noutbuffersize), core::mem::transmute_copy(&lpbytesreturned), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn SetNumChannelGroups<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroups: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::SetNumChannelGroups(this, core::mem::transmute_copy(&dwchannelgroups)).into()
            }
        }
        unsafe extern "system" fn GetNumChannelGroups<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwchannelgroups: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::GetNumChannelGroups(this, core::mem::transmute_copy(&pdwchannelgroups)).into()
            }
        }
        unsafe extern "system" fn Activate<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::Activate(this, core::mem::transmute_copy(&factive)).into()
            }
        }
        unsafe extern "system" fn SetChannelPriority<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::SetChannelPriority(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&dwpriority)).into()
            }
        }
        unsafe extern "system" fn GetChannelPriority<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::GetChannelPriority(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&pdwpriority)).into()
            }
        }
        unsafe extern "system" fn SetDirectSound<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, pdirectsoundbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::SetDirectSound(this, core::mem::transmute_copy(&pdirectsound), core::mem::transmute_copy(&pdirectsoundbuffer)).into()
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPort_Impl::GetFormat(this, core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&pdwwaveformatexsize), core::mem::transmute_copy(&pdwbuffersize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PlayBuffer: PlayBuffer::<Identity, OFFSET>,
            SetReadNotificationHandle: SetReadNotificationHandle::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            DownloadInstrument: DownloadInstrument::<Identity, OFFSET>,
            UnloadInstrument: UnloadInstrument::<Identity, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, OFFSET>,
            GetRunningStats: GetRunningStats::<Identity, OFFSET>,
            Compact: Compact::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Identity, OFFSET>,
            GetNumChannelGroups: GetNumChannelGroups::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            SetChannelPriority: SetChannelPriority::<Identity, OFFSET>,
            GetChannelPriority: GetChannelPriority::<Identity, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicPort as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
impl windows_core::RuntimeName for IDirectMusicPort {}
windows_core::imp::define_interface!(IDirectMusicPortDownload, IDirectMusicPortDownload_Vtbl, 0xd2ac287a_b39b_11d1_8704_00600893b1bd);
windows_core::imp::interface_hierarchy!(IDirectMusicPortDownload, windows_core::IUnknown);
impl IDirectMusicPortDownload {
    pub unsafe fn GetBuffer(&self, dwdlid: u32) -> windows_core::Result<IDirectMusicDownload> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), dwdlid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AllocateBuffer(&self, dwsize: u32) -> windows_core::Result<IDirectMusicDownload> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocateBuffer)(windows_core::Interface::as_raw(self), dwsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDLId)(windows_core::Interface::as_raw(self), pdwstartdlid as _, dwcount).ok() }
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAppend)(windows_core::Interface::as_raw(self), pdwappend as _).ok() }
    }
    pub unsafe fn Download<P0>(&self, pidmdownload: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicDownload>,
    {
        unsafe { (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), pidmdownload.param().abi()).ok() }
    }
    pub unsafe fn Unload<P0>(&self, pidmdownload: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicDownload>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unload)(windows_core::Interface::as_raw(self), pidmdownload.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPortDownload_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDLId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub GetAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectMusicPortDownload_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, dwdlid: u32) -> windows_core::Result<IDirectMusicDownload>;
    fn AllocateBuffer(&self, dwsize: u32) -> windows_core::Result<IDirectMusicDownload>;
    fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> windows_core::Result<()>;
    fn GetAppend(&self, pdwappend: *mut u32) -> windows_core::Result<()>;
    fn Download(&self, pidmdownload: windows_core::Ref<'_, IDirectMusicDownload>) -> windows_core::Result<()>;
    fn Unload(&self, pidmdownload: windows_core::Ref<'_, IDirectMusicDownload>) -> windows_core::Result<()>;
}
impl IDirectMusicPortDownload_Vtbl {
    pub const fn new<Identity: IDirectMusicPortDownload_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdlid: u32, ppidmdownload: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectMusicPortDownload_Impl::GetBuffer(this, core::mem::transmute_copy(&dwdlid)) {
                    Ok(ok__) => {
                        ppidmdownload.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AllocateBuffer<Identity: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsize: u32, ppidmdownload: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectMusicPortDownload_Impl::AllocateBuffer(this, core::mem::transmute_copy(&dwsize)) {
                    Ok(ok__) => {
                        ppidmdownload.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDLId<Identity: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstartdlid: *mut u32, dwcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPortDownload_Impl::GetDLId(this, core::mem::transmute_copy(&pdwstartdlid), core::mem::transmute_copy(&dwcount)).into()
            }
        }
        unsafe extern "system" fn GetAppend<Identity: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwappend: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPortDownload_Impl::GetAppend(this, core::mem::transmute_copy(&pdwappend)).into()
            }
        }
        unsafe extern "system" fn Download<Identity: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidmdownload: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPortDownload_Impl::Download(this, core::mem::transmute_copy(&pidmdownload)).into()
            }
        }
        unsafe extern "system" fn Unload<Identity: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidmdownload: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicPortDownload_Impl::Unload(this, core::mem::transmute_copy(&pidmdownload)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, OFFSET>,
            GetDLId: GetDLId::<Identity, OFFSET>,
            GetAppend: GetAppend::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            Unload: Unload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicPortDownload as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicPortDownload {}
windows_core::imp::define_interface!(IDirectMusicSynth, IDirectMusicSynth_Vtbl, 0x09823661_5c85_11d2_afa6_00aa0024d8b6);
windows_core::imp::interface_hierarchy!(IDirectMusicSynth, windows_core::IUnknown);
impl IDirectMusicSynth {
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pportparams as _).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNumChannelGroups)(windows_core::Interface::as_raw(self), dwgroups).ok() }
    }
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut core::ffi::c_void, pbfree: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), phdownload as _, pvdata as _, pbfree as _).ok() }
    }
    pub unsafe fn Unload(&self, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unload)(windows_core::Interface::as_raw(self), hdownload, lpfreehandle, huserdata).ok() }
    }
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PlayBuffer)(windows_core::Interface::as_raw(self), rt, pbbuffer as _, cbbuffer).ok() }
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRunningStats)(windows_core::Interface::as_raw(self), pstats as _).ok() }
    }
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPortCaps)(windows_core::Interface::as_raw(self), pcaps as _).ok() }
    }
    pub unsafe fn SetMasterClock<P0>(&self, pclock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::IReferenceClock>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMasterClock)(windows_core::Interface::as_raw(self), pclock.param().abi()).ok() }
    }
    pub unsafe fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLatencyClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Activate(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
    pub unsafe fn SetSynthSink<P0>(&self, psynthsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicSynthSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSynthSink)(windows_core::Interface::as_raw(self), psynthsink.param().abi()).ok() }
    }
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), pbuffer as _, dwlength, llposition).ok() }
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelPriority)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, dwpriority).ok() }
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetChannelPriority)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, pdwpriority as _).ok() }
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pwaveformatex as _, pdwwaveformatexsize as _).ok() }
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAppend)(windows_core::Interface::as_raw(self), pdwappend as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_PORTPARAMS8) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumChannelGroups: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::HANDLE, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Unload: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HANDLE, isize, super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub PlayBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut u8, u32) -> windows_core::HRESULT,
    pub GetRunningStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_SYNTHSTATS) -> windows_core::HRESULT,
    pub GetPortCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_PORTCAPS) -> windows_core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetSynthSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16, u32, i64) -> windows_core::HRESULT,
    pub SetChannelPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetChannelPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    pub GetAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDirectMusicSynth_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn SetNumChannelGroups(&self, dwgroups: u32) -> windows_core::Result<()>;
    fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut core::ffi::c_void, pbfree: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn Unload(&self, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> windows_core::Result<()>;
    fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::Result<()>;
    fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()>;
    fn SetMasterClock(&self, pclock: windows_core::Ref<'_, super::super::IReferenceClock>) -> windows_core::Result<()>;
    fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock>;
    fn Activate(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetSynthSink(&self, psynthsink: windows_core::Ref<'_, IDirectMusicSynthSink>) -> windows_core::Result<()>;
    fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> windows_core::Result<()>;
    fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::Result<()>;
    fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::Result<()>;
    fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> windows_core::Result<()>;
    fn GetAppend(&self, pdwappend: *mut u32) -> windows_core::Result<()>;
}
impl IDirectMusicSynth_Vtbl {
    pub const fn new<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::Open(this, core::mem::transmute_copy(&pportparams)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn SetNumChannelGroups<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwgroups: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::SetNumChannelGroups(this, core::mem::transmute_copy(&dwgroups)).into()
            }
        }
        unsafe extern "system" fn Download<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut core::ffi::c_void, pbfree: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::Download(this, core::mem::transmute_copy(&phdownload), core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&pbfree)).into()
            }
        }
        unsafe extern "system" fn Unload<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::Unload(this, core::mem::transmute_copy(&hdownload), core::mem::transmute_copy(&lpfreehandle), core::mem::transmute_copy(&huserdata)).into()
            }
        }
        unsafe extern "system" fn PlayBuffer<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::PlayBuffer(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&cbbuffer)).into()
            }
        }
        unsafe extern "system" fn GetRunningStats<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::GetRunningStats(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn GetPortCaps<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::GetPortCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn SetMasterClock<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclock: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::SetMasterClock(this, core::mem::transmute_copy(&pclock)).into()
            }
        }
        unsafe extern "system" fn GetLatencyClock<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectMusicSynth_Impl::GetLatencyClock(this) {
                    Ok(ok__) => {
                        ppclock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Activate<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::Activate(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn SetSynthSink<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psynthsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::SetSynthSink(this, core::mem::transmute_copy(&psynthsink)).into()
            }
        }
        unsafe extern "system" fn Render<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::Render(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwlength), core::mem::transmute_copy(&llposition)).into()
            }
        }
        unsafe extern "system" fn SetChannelPriority<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::SetChannelPriority(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&dwpriority)).into()
            }
        }
        unsafe extern "system" fn GetChannelPriority<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::GetChannelPriority(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&pdwpriority)).into()
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::GetFormat(this, core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&pdwwaveformatexsize)).into()
            }
        }
        unsafe extern "system" fn GetAppend<Identity: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwappend: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth_Impl::GetAppend(this, core::mem::transmute_copy(&pdwappend)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            Unload: Unload::<Identity, OFFSET>,
            PlayBuffer: PlayBuffer::<Identity, OFFSET>,
            GetRunningStats: GetRunningStats::<Identity, OFFSET>,
            GetPortCaps: GetPortCaps::<Identity, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            SetSynthSink: SetSynthSink::<Identity, OFFSET>,
            Render: Render::<Identity, OFFSET>,
            SetChannelPriority: SetChannelPriority::<Identity, OFFSET>,
            GetChannelPriority: GetChannelPriority::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            GetAppend: GetAppend::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicSynth as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicSynth {}
windows_core::imp::define_interface!(IDirectMusicSynth8, IDirectMusicSynth8_Vtbl, 0x53cab625_2711_4c9f_9de7_1b7f925f6fc8);
impl core::ops::Deref for IDirectMusicSynth8 {
    type Target = IDirectMusicSynth;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicSynth8, windows_core::IUnknown, IDirectMusicSynth);
impl IDirectMusicSynth8 {
    pub unsafe fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PlayVoice)(windows_core::Interface::as_raw(self), rt, dwvoiceid, dwchannelgroup, dwchannel, dwdlid, prpitch, vrvolume, stvoicestart, stloopstart, stloopend).ok() }
    }
    pub unsafe fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopVoice)(windows_core::Interface::as_raw(self), rt, dwvoiceid).ok() }
    }
    pub unsafe fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVoiceState)(windows_core::Interface::as_raw(self), dwvoice as _, cbvoice, dwvoicestate as _).ok() }
    }
    pub unsafe fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), dwdownloadid, dwflags).ok() }
    }
    pub unsafe fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssignChannelToBuses)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, pdwbuses as _, cbuses).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth8_Vtbl {
    pub base__: IDirectMusicSynth_Vtbl,
    pub PlayVoice: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, u32, u32, u32, i32, i32, u64, u64, u64) -> windows_core::HRESULT,
    pub StopVoice: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32) -> windows_core::HRESULT,
    pub GetVoiceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut DMUS_VOICE_STATE) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub AssignChannelToBuses: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, u32) -> windows_core::HRESULT,
}
pub trait IDirectMusicSynth8_Impl: IDirectMusicSynth_Impl {
    fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> windows_core::Result<()>;
    fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> windows_core::Result<()>;
    fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> windows_core::Result<()>;
    fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> windows_core::Result<()>;
    fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> windows_core::Result<()>;
}
impl IDirectMusicSynth8_Vtbl {
    pub const fn new<Identity: IDirectMusicSynth8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PlayVoice<Identity: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth8_Impl::PlayVoice(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&dwvoiceid), core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&dwdlid), core::mem::transmute_copy(&prpitch), core::mem::transmute_copy(&vrvolume), core::mem::transmute_copy(&stvoicestart), core::mem::transmute_copy(&stloopstart), core::mem::transmute_copy(&stloopend)).into()
            }
        }
        unsafe extern "system" fn StopVoice<Identity: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, dwvoiceid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth8_Impl::StopVoice(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&dwvoiceid)).into()
            }
        }
        unsafe extern "system" fn GetVoiceState<Identity: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth8_Impl::GetVoiceState(this, core::mem::transmute_copy(&dwvoice), core::mem::transmute_copy(&cbvoice), core::mem::transmute_copy(&dwvoicestate)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdownloadid: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth8_Impl::Refresh(this, core::mem::transmute_copy(&dwdownloadid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn AssignChannelToBuses<Identity: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynth8_Impl::AssignChannelToBuses(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&pdwbuses), core::mem::transmute_copy(&cbuses)).into()
            }
        }
        Self {
            base__: IDirectMusicSynth_Vtbl::new::<Identity, OFFSET>(),
            PlayVoice: PlayVoice::<Identity, OFFSET>,
            StopVoice: StopVoice::<Identity, OFFSET>,
            GetVoiceState: GetVoiceState::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            AssignChannelToBuses: AssignChannelToBuses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicSynth8 as windows_core::Interface>::IID || iid == &<IDirectMusicSynth as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicSynth8 {}
windows_core::imp::define_interface!(IDirectMusicSynthSink, IDirectMusicSynthSink_Vtbl, 0x09823663_5c85_11d2_afa6_00aa0024d8b6);
windows_core::imp::interface_hierarchy!(IDirectMusicSynthSink, windows_core::IUnknown);
impl IDirectMusicSynthSink {
    pub unsafe fn Init<P0>(&self, psynth: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicSynth>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), psynth.param().abi()).ok() }
    }
    pub unsafe fn SetMasterClock<P0>(&self, pclock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::IReferenceClock>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMasterClock)(windows_core::Interface::as_raw(self), pclock.param().abi()).ok() }
    }
    pub unsafe fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLatencyClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Activate(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
    pub unsafe fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SampleToRefTime)(windows_core::Interface::as_raw(self), llsampletime, prftime as _).ok() }
    }
    pub unsafe fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RefTimeToSample)(windows_core::Interface::as_raw(self), rftime, pllsampletime as _).ok() }
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<P0, P1>(&self, pdirectsound: P0, pdirectsoundbuffer: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectSound::IDirectSound>,
        P1: windows_core::Param<super::DirectSound::IDirectSoundBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDirectSound)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), pdirectsoundbuffer.param().abi()).ok() }
    }
    pub unsafe fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesiredBufferSize)(windows_core::Interface::as_raw(self), pdwbuffersizeinsamples as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynthSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SampleToRefTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut i64) -> windows_core::HRESULT,
    pub RefTimeToSample: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut i64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub SetDirectSound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))]
    SetDirectSound: usize,
    pub GetDesiredBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub trait IDirectMusicSynthSink_Impl: windows_core::IUnknownImpl {
    fn Init(&self, psynth: windows_core::Ref<'_, IDirectMusicSynth>) -> windows_core::Result<()>;
    fn SetMasterClock(&self, pclock: windows_core::Ref<'_, super::super::IReferenceClock>) -> windows_core::Result<()>;
    fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock>;
    fn Activate(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> windows_core::Result<()>;
    fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> windows_core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: windows_core::Ref<'_, super::DirectSound::IDirectSound>, pdirectsoundbuffer: windows_core::Ref<'_, super::DirectSound::IDirectSoundBuffer>) -> windows_core::Result<()>;
    fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl IDirectMusicSynthSink_Vtbl {
    pub const fn new<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psynth: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynthSink_Impl::Init(this, core::mem::transmute_copy(&psynth)).into()
            }
        }
        unsafe extern "system" fn SetMasterClock<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclock: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynthSink_Impl::SetMasterClock(this, core::mem::transmute_copy(&pclock)).into()
            }
        }
        unsafe extern "system" fn GetLatencyClock<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectMusicSynthSink_Impl::GetLatencyClock(this) {
                    Ok(ok__) => {
                        ppclock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Activate<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynthSink_Impl::Activate(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn SampleToRefTime<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llsampletime: i64, prftime: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynthSink_Impl::SampleToRefTime(this, core::mem::transmute_copy(&llsampletime), core::mem::transmute_copy(&prftime)).into()
            }
        }
        unsafe extern "system" fn RefTimeToSample<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rftime: i64, pllsampletime: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynthSink_Impl::RefTimeToSample(this, core::mem::transmute_copy(&rftime), core::mem::transmute_copy(&pllsampletime)).into()
            }
        }
        unsafe extern "system" fn SetDirectSound<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, pdirectsoundbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynthSink_Impl::SetDirectSound(this, core::mem::transmute_copy(&pdirectsound), core::mem::transmute_copy(&pdirectsoundbuffer)).into()
            }
        }
        unsafe extern "system" fn GetDesiredBufferSize<Identity: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbuffersizeinsamples: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicSynthSink_Impl::GetDesiredBufferSize(this, core::mem::transmute_copy(&pdwbuffersizeinsamples)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            SampleToRefTime: SampleToRefTime::<Identity, OFFSET>,
            RefTimeToSample: RefTimeToSample::<Identity, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, OFFSET>,
            GetDesiredBufferSize: GetDesiredBufferSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicSynthSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl windows_core::RuntimeName for IDirectMusicSynthSink {}
windows_core::imp::define_interface!(IDirectMusicThru, IDirectMusicThru_Vtbl, 0xced153e7_3606_11d2_b9f9_0000f875ac12);
windows_core::imp::interface_hierarchy!(IDirectMusicThru, windows_core::IUnknown);
impl IDirectMusicThru {
    pub unsafe fn ThruChannel<P4>(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: P4) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IDirectMusicPort>,
    {
        unsafe { (windows_core::Interface::vtable(self).ThruChannel)(windows_core::Interface::as_raw(self), dwsourcechannelgroup, dwsourcechannel, dwdestinationchannelgroup, dwdestinationchannel, pdestinationport.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicThru_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThruChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectMusicThru_Impl: windows_core::IUnknownImpl {
    fn ThruChannel(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: windows_core::Ref<'_, IDirectMusicPort>) -> windows_core::Result<()>;
}
impl IDirectMusicThru_Vtbl {
    pub const fn new<Identity: IDirectMusicThru_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ThruChannel<Identity: IDirectMusicThru_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectMusicThru_Impl::ThruChannel(this, core::mem::transmute_copy(&dwsourcechannelgroup), core::mem::transmute_copy(&dwsourcechannel), core::mem::transmute_copy(&dwdestinationchannelgroup), core::mem::transmute_copy(&dwdestinationchannel), core::mem::transmute_copy(&pdestinationport)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ThruChannel: ThruChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicThru as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectMusicThru {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INSTHEADER {
    pub cRegions: u32,
    pub Locale: MIDILOCALE,
}
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1 = Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA = Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW = Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MDEVICECAPSEX {
    pub cbSize: u32,
    pub pCaps: *mut core::ffi::c_void,
}
impl Default for MDEVICECAPSEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIDILOCALE {
    pub ulBank: u32,
    pub ulInstrument: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Multimedia")]
#[derive(Clone, Copy)]
pub struct MIDIOPENDESC {
    pub hMidi: super::HMIDI,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub dnDevNode: usize,
    pub cIds: u32,
    pub rgIds: [super::super::Multimedia::MIDIOPENSTRMID; 1],
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl Default for MIDIOPENDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POOLCUE {
    pub ulOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POOLTABLE {
    pub cbSize: u32,
    pub cCues: u32,
}
pub const POOL_CUE_NULL: i32 = -1i32;
pub const REFRESH_F_LASTBUFFER: u32 = 1u32;
pub const REGSTR_PATH_SOFTWARESYNTHS: windows_core::PCSTR = windows_core::s!("Software\\Microsoft\\DirectMusic\\SoftwareSynths");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RGNHEADER {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RGNRANGE {
    pub usLow: u16,
    pub usHigh: u16,
}
pub const SIZE_DVINFO: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WAVELINK {
    pub fusOptions: u16,
    pub usPhaseGroup: u16,
    pub ulChannel: u32,
    pub ulTableIndex: u32,
}
pub const WAVELINK_CHANNEL_LEFT: i32 = 1i32;
pub const WAVELINK_CHANNEL_RIGHT: i32 = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WLOOP {
    pub cbSize: u32,
    pub ulType: u32,
    pub ulStart: u32,
    pub ulLength: u32,
}
pub const WLOOP_TYPE_FORWARD: u32 = 0u32;
pub const WLOOP_TYPE_RELEASE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMPL {
    pub cbSize: u32,
    pub usUnityNote: u16,
    pub sFineTune: i16,
    pub lAttenuation: i32,
    pub fulOptions: u32,
    pub cSampleLoops: u32,
}
