#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en0: CLK_EN0,
    _reserved1: [u8; 0x18],
    psram_d_pin0: PSRAM_D_PIN0,
    psram_q_pin0: PSRAM_Q_PIN0,
    psram_wp_pin0: PSRAM_WP_PIN0,
    psram_hold_pin0: PSRAM_HOLD_PIN0,
    psram_dq4_pin0: PSRAM_DQ4_PIN0,
    psram_dq5_pin0: PSRAM_DQ5_PIN0,
    psram_dq6_pin0: PSRAM_DQ6_PIN0,
    psram_dq7_pin0: PSRAM_DQ7_PIN0,
    psram_dqs_0_pin0: PSRAM_DQS_0_PIN0,
    psram_ck_pin0: PSRAM_CK_PIN0,
    psram_cs_pin0: PSRAM_CS_PIN0,
    psram_dq8_pin0: PSRAM_DQ8_PIN0,
    psram_dq9_pin0: PSRAM_DQ9_PIN0,
    psram_dq10_pin0: PSRAM_DQ10_PIN0,
    psram_dq11_pin0: PSRAM_DQ11_PIN0,
    psram_dq12_pin0: PSRAM_DQ12_PIN0,
    psram_dq13_pin0: PSRAM_DQ13_PIN0,
    psram_dq14_pin0: PSRAM_DQ14_PIN0,
    psram_dq15_pin0: PSRAM_DQ15_PIN0,
    psram_dqs_1_pin0: PSRAM_DQS_1_PIN0,
}
impl RegisterBlock {
    #[doc = "0x00 - apb registers auto clock gating reg"]
    #[inline(always)]
    pub const fn clk_en0(&self) -> &CLK_EN0 {
        &self.clk_en0
    }
    #[doc = "0x1c - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_d_pin0(&self) -> &PSRAM_D_PIN0 {
        &self.psram_d_pin0
    }
    #[doc = "0x20 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_q_pin0(&self) -> &PSRAM_Q_PIN0 {
        &self.psram_q_pin0
    }
    #[doc = "0x24 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_wp_pin0(&self) -> &PSRAM_WP_PIN0 {
        &self.psram_wp_pin0
    }
    #[doc = "0x28 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_hold_pin0(&self) -> &PSRAM_HOLD_PIN0 {
        &self.psram_hold_pin0
    }
    #[doc = "0x2c - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq4_pin0(&self) -> &PSRAM_DQ4_PIN0 {
        &self.psram_dq4_pin0
    }
    #[doc = "0x30 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq5_pin0(&self) -> &PSRAM_DQ5_PIN0 {
        &self.psram_dq5_pin0
    }
    #[doc = "0x34 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq6_pin0(&self) -> &PSRAM_DQ6_PIN0 {
        &self.psram_dq6_pin0
    }
    #[doc = "0x38 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq7_pin0(&self) -> &PSRAM_DQ7_PIN0 {
        &self.psram_dq7_pin0
    }
    #[doc = "0x3c - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dqs_0_pin0(&self) -> &PSRAM_DQS_0_PIN0 {
        &self.psram_dqs_0_pin0
    }
    #[doc = "0x40 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_ck_pin0(&self) -> &PSRAM_CK_PIN0 {
        &self.psram_ck_pin0
    }
    #[doc = "0x44 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_cs_pin0(&self) -> &PSRAM_CS_PIN0 {
        &self.psram_cs_pin0
    }
    #[doc = "0x48 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq8_pin0(&self) -> &PSRAM_DQ8_PIN0 {
        &self.psram_dq8_pin0
    }
    #[doc = "0x4c - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq9_pin0(&self) -> &PSRAM_DQ9_PIN0 {
        &self.psram_dq9_pin0
    }
    #[doc = "0x50 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq10_pin0(&self) -> &PSRAM_DQ10_PIN0 {
        &self.psram_dq10_pin0
    }
    #[doc = "0x54 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq11_pin0(&self) -> &PSRAM_DQ11_PIN0 {
        &self.psram_dq11_pin0
    }
    #[doc = "0x58 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq12_pin0(&self) -> &PSRAM_DQ12_PIN0 {
        &self.psram_dq12_pin0
    }
    #[doc = "0x5c - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq13_pin0(&self) -> &PSRAM_DQ13_PIN0 {
        &self.psram_dq13_pin0
    }
    #[doc = "0x60 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq14_pin0(&self) -> &PSRAM_DQ14_PIN0 {
        &self.psram_dq14_pin0
    }
    #[doc = "0x64 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dq15_pin0(&self) -> &PSRAM_DQ15_PIN0 {
        &self.psram_dq15_pin0
    }
    #[doc = "0x68 - IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
    #[inline(always)]
    pub const fn psram_dqs_1_pin0(&self) -> &PSRAM_DQS_1_PIN0 {
        &self.psram_dqs_1_pin0
    }
}
#[doc = "CLK_EN0 (rw) register accessor: apb registers auto clock gating reg\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en0`] module"]
pub type CLK_EN0 = crate::Reg<clk_en0::CLK_EN0_SPEC>;
#[doc = "apb registers auto clock gating reg"]
pub mod clk_en0;
#[doc = "PSRAM_D_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_d_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_d_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_d_pin0`] module"]
pub type PSRAM_D_PIN0 = crate::Reg<psram_d_pin0::PSRAM_D_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_d_pin0;
pub use psram_d_pin0 as psram_q_pin0;
pub use psram_d_pin0 as psram_wp_pin0;
pub use psram_d_pin0 as psram_hold_pin0;
pub use psram_d_pin0 as psram_dq4_pin0;
pub use psram_d_pin0 as psram_dq5_pin0;
pub use psram_d_pin0 as psram_dq6_pin0;
pub use psram_d_pin0 as psram_dq7_pin0;
pub use PSRAM_D_PIN0 as PSRAM_Q_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_WP_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_HOLD_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ4_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ5_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ6_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ7_PIN0;
#[doc = "PSRAM_DQS_0_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dqs_0_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dqs_0_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_dqs_0_pin0`] module"]
pub type PSRAM_DQS_0_PIN0 = crate::Reg<psram_dqs_0_pin0::PSRAM_DQS_0_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_dqs_0_pin0;
pub use psram_d_pin0 as psram_ck_pin0;
pub use psram_d_pin0 as psram_cs_pin0;
pub use psram_d_pin0 as psram_dq8_pin0;
pub use psram_d_pin0 as psram_dq9_pin0;
pub use psram_d_pin0 as psram_dq10_pin0;
pub use psram_d_pin0 as psram_dq11_pin0;
pub use psram_d_pin0 as psram_dq12_pin0;
pub use psram_d_pin0 as psram_dq13_pin0;
pub use psram_d_pin0 as psram_dq14_pin0;
pub use psram_d_pin0 as psram_dq15_pin0;
pub use psram_dqs_0_pin0 as psram_dqs_1_pin0;
pub use PSRAM_DQS_0_PIN0 as PSRAM_DQS_1_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_CK_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_CS_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ8_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ9_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ10_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ11_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ12_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ13_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ14_PIN0;
pub use PSRAM_D_PIN0 as PSRAM_DQ15_PIN0;
