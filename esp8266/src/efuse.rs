#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - EFUSE_DATA0"]
    pub efuse_data0: EFUSE_DATA0,
    #[doc = "0x04 - EFUSE_DATA1"]
    pub efuse_data1: EFUSE_DATA1,
    #[doc = "0x08 - EFUSE_DATA2"]
    pub efuse_data2: EFUSE_DATA2,
    #[doc = "0x0c - EFUSE_DATA3"]
    pub efuse_data3: EFUSE_DATA3,
}
#[doc = "EFUSE_DATA0 (rw) register accessor: an alias for `Reg<EFUSE_DATA0_SPEC>`"]
pub type EFUSE_DATA0 = crate::Reg<efuse_data0::EFUSE_DATA0_SPEC>;
#[doc = "EFUSE_DATA0"]
pub mod efuse_data0;
#[doc = "EFUSE_DATA1 (rw) register accessor: an alias for `Reg<EFUSE_DATA1_SPEC>`"]
pub type EFUSE_DATA1 = crate::Reg<efuse_data1::EFUSE_DATA1_SPEC>;
#[doc = "EFUSE_DATA1"]
pub mod efuse_data1;
#[doc = "EFUSE_DATA2 (rw) register accessor: an alias for `Reg<EFUSE_DATA2_SPEC>`"]
pub type EFUSE_DATA2 = crate::Reg<efuse_data2::EFUSE_DATA2_SPEC>;
#[doc = "EFUSE_DATA2"]
pub mod efuse_data2;
#[doc = "EFUSE_DATA3 (rw) register accessor: an alias for `Reg<EFUSE_DATA3_SPEC>`"]
pub type EFUSE_DATA3 = crate::Reg<efuse_data3::EFUSE_DATA3_SPEC>;
#[doc = "EFUSE_DATA3"]
pub mod efuse_data3;
