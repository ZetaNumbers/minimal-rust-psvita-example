use bytemuck::{Pod, Zeroable};
use core::convert::TryInto;
use sha1::{Digest, Sha1};

pub type Nid = u32;
pub type Address = u32;

pub fn generate_nid(name: &[u8]) -> Nid {
    let digest: [u8; 20] = Sha1::digest(name).into();
    let digest_tail: &[u8; 4] = digest[..4].try_into().unwrap();
    Nid::from_le_bytes(*digest_tail)
}

pub mod sce_module_info {
    use super::*;

    pub const MODULE_NAME_MAX_LEN: usize = 27;

    /// Common beginning of `SceModuleInfo` structs.
    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct Common {
        /// Attributes of the module
        pub attributes: u16,
        /// Major version of the module (usually set to 1) followed by Minor version of the module (usually set to 1)
        pub module_version: [u8; 2],
        /// Name of the module. Null-terminated string.
        pub name: [u8; MODULE_NAME_MAX_LEN],
        /// SceModuleInfo version
        pub info_version: u8,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct AddressRange {
        /// Offset to top of address range
        top: Address,
        /// Offset to bottom of address range
        bottom: Address,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct PublicApi {
        /// Exports array
        exports: AddressRange,
        /// Imports array
        imports: AddressRange,
    }

    /// Global pointer value for MIPS, TOC address (address of .toc) for PowerPC, always 0 for ARM
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct GPValue {
        pub inner: Address,
    }

    /// It was wrongly named module NID. It is a sort of hash to ensure integrity and versioning.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct DebugFingerprint {
        pub inner: u32,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct Entries {
        /// Offset to module_start function. To disable set it to:
        /// - `-1` according to [Henkaku wiki](https://wiki.henkaku.xyz/vita/Modules#SceModuleInfo)
        /// - `0` according to [vitasdk toolchain](https://github.com/vitasdk/vita-toolchain/blob/a075d3ab2963d6b12e1a51b6816022d4f0d2c41d/src/sce-elf-defs.h#L36)
        /// TODO: test both
        pub start_entry: Address,
        /// Offset to module_stop function. To disable set it to:
        /// - `-1` according to [Henkaku wiki](https://wiki.henkaku.xyz/vita/Modules#SceModuleInfo)
        /// - `0` according to [vitasdk toolchain](https://github.com/vitasdk/vita-toolchain/blob/a075d3ab2963d6b12e1a51b6816022d4f0d2c41d/src/sce-elf-defs.h#L37)
        /// TODO: test both
        pub stop_entry: Address,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct TlsInfo {
        /// Offset to start of TLS (Thread Local Storage)
        pub tls_start: Address,
        /// Certainly equals (tls_end - tls_start)
        pub tls_filesz: Address,
        /// Certainly equals (tls_initialized_data_end - tls_start)
        pub tls_memsz: Address,
    }

    /// Address range of ARM EXIDX (optional)
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct ArmExidx {
        pub inner: AddressRange,
    }

    /// Address range of ARM EXTAB (optional)
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct ArmExtab {
        pub inner: AddressRange,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct V0 {
        pub common: Common,
        pub gp_value: GPValue,
        pub public_api: PublicApi,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct V1 {
        pub common: Common,
        pub gp_value: GPValue,
        pub public_api: PublicApi,
        pub debug_fingerprint: DebugFingerprint,
        pub entries: Entries,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct V2 {
        pub common: Common,
        pub gp_value: GPValue,
        pub public_api: PublicApi,
        pub debug_fingerprint: DebugFingerprint,
        pub entries: Entries,
        pub arm_exidx: ArmExidx,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct V3 {
        pub common: Common,
        pub gp_value: GPValue,
        pub public_api: PublicApi,
        pub debug_fingerprint: DebugFingerprint,
        pub entries: Entries,
        pub arm_exidx: ArmExidx,
        pub tls: TlsInfo,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Zeroable, Pod)]
    pub struct V6 {
        pub common: Common,
        pub gp_value: GPValue,
        pub public_api: PublicApi,
        pub debug_fingerprint: DebugFingerprint,
        pub tls: TlsInfo,
        pub entries: Entries,
        pub arm_exidx: ArmExidx,
        pub arm_extab: ArmExtab,
    }

    #[cfg(test)]
    #[test]
    fn type_assertions() {
        use core::mem::size_of;
        assert_eq!(size_of::<Common>(), 0x20);
        assert_eq!(size_of::<V0>(), 0x34);
        assert_eq!(size_of::<V1>(), 0x40);
        assert_eq!(size_of::<V2>(), 0x48);
        assert_eq!(size_of::<V3>(), 0x54);
        assert_eq!(size_of::<V6>(), 0x5C);
    }
}