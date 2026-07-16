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
}
#[doc = "CLK_EN0 (rw) register accessor: apb registers auto clock gating reg\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en0`] module"]
pub type CLK_EN0 = crate::Reg<clk_en0::CLK_EN0_SPEC>;
#[doc = "apb registers auto clock gating reg"]
pub mod clk_en0;
#[doc = "PSRAM_D_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_d_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_d_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_d_pin0`] module"]
pub type PSRAM_D_PIN0 = crate::Reg<psram_d_pin0::PSRAM_D_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_d_pin0;
#[doc = "PSRAM_Q_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_q_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_q_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_q_pin0`] module"]
pub type PSRAM_Q_PIN0 = crate::Reg<psram_q_pin0::PSRAM_Q_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_q_pin0;
#[doc = "PSRAM_WP_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_wp_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_wp_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_wp_pin0`] module"]
pub type PSRAM_WP_PIN0 = crate::Reg<psram_wp_pin0::PSRAM_WP_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_wp_pin0;
#[doc = "PSRAM_HOLD_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_hold_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_hold_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_hold_pin0`] module"]
pub type PSRAM_HOLD_PIN0 = crate::Reg<psram_hold_pin0::PSRAM_HOLD_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_hold_pin0;
#[doc = "PSRAM_DQ4_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dq4_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dq4_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_dq4_pin0`] module"]
pub type PSRAM_DQ4_PIN0 = crate::Reg<psram_dq4_pin0::PSRAM_DQ4_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_dq4_pin0;
#[doc = "PSRAM_DQ5_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dq5_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dq5_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_dq5_pin0`] module"]
pub type PSRAM_DQ5_PIN0 = crate::Reg<psram_dq5_pin0::PSRAM_DQ5_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_dq5_pin0;
#[doc = "PSRAM_DQ6_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dq6_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dq6_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_dq6_pin0`] module"]
pub type PSRAM_DQ6_PIN0 = crate::Reg<psram_dq6_pin0::PSRAM_DQ6_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_dq6_pin0;
#[doc = "PSRAM_DQ7_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dq7_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dq7_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_dq7_pin0`] module"]
pub type PSRAM_DQ7_PIN0 = crate::Reg<psram_dq7_pin0::PSRAM_DQ7_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_dq7_pin0;
#[doc = "PSRAM_DQS_0_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dqs_0_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dqs_0_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_dqs_0_pin0`] module"]
pub type PSRAM_DQS_0_PIN0 = crate::Reg<psram_dqs_0_pin0::PSRAM_DQS_0_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_dqs_0_pin0;
#[doc = "PSRAM_CK_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_ck_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_ck_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_ck_pin0`] module"]
pub type PSRAM_CK_PIN0 = crate::Reg<psram_ck_pin0::PSRAM_CK_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_ck_pin0;
#[doc = "PSRAM_CS_PIN0 (rw) register accessor: IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_cs_pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_cs_pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_cs_pin0`] module"]
pub type PSRAM_CS_PIN0 = crate::Reg<psram_cs_pin0::PSRAM_CS_PIN0_SPEC>;
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG"]
pub mod psram_cs_pin0;
