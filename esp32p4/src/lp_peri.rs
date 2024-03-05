#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en: CLK_EN,
    core_clk_sel: CORE_CLK_SEL,
    reset_en: RESET_EN,
    cpu: CPU,
    _reserved4: [u8; 0x18],
    mem_ctrl: MEM_CTRL,
    adc_ctrl: ADC_CTRL,
    lp_i2s_rxclk_div_num: LP_I2S_RXCLK_DIV_NUM,
    lp_i2s_rxclk_div_xyz: LP_I2S_RXCLK_DIV_XYZ,
    lp_i2s_txclk_div_num: LP_I2S_TXCLK_DIV_NUM,
    lp_i2s_txclk_div_xyz: LP_I2S_TXCLK_DIV_XYZ,
    _reserved10: [u8; 0x03bc],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn core_clk_sel(&self) -> &CORE_CLK_SEL {
        &self.core_clk_sel
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn reset_en(&self) -> &RESET_EN {
        &self.reset_en
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn cpu(&self) -> &CPU {
        &self.cpu
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn mem_ctrl(&self) -> &MEM_CTRL {
        &self.mem_ctrl
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn adc_ctrl(&self) -> &ADC_CTRL {
        &self.adc_ctrl
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_i2s_rxclk_div_num(&self) -> &LP_I2S_RXCLK_DIV_NUM {
        &self.lp_i2s_rxclk_div_num
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_i2s_rxclk_div_xyz(&self) -> &LP_I2S_RXCLK_DIV_XYZ {
        &self.lp_i2s_rxclk_div_xyz
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_i2s_txclk_div_num(&self) -> &LP_I2S_TXCLK_DIV_NUM {
        &self.lp_i2s_txclk_div_num
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_i2s_txclk_div_xyz(&self) -> &LP_I2S_TXCLK_DIV_XYZ {
        &self.lp_i2s_txclk_div_xyz
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CLK_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "need_des"]
pub mod clk_en;
#[doc = "CORE_CLK_SEL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_clk_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_clk_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_clk_sel`] module"]
pub type CORE_CLK_SEL = crate::Reg<core_clk_sel::CORE_CLK_SEL_SPEC>;
#[doc = "need_des"]
pub mod core_clk_sel;
#[doc = "RESET_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_en`] module"]
pub type RESET_EN = crate::Reg<reset_en::RESET_EN_SPEC>;
#[doc = "need_des"]
pub mod reset_en;
#[doc = "CPU (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu`] module"]
pub type CPU = crate::Reg<cpu::CPU_SPEC>;
#[doc = "need_des"]
pub mod cpu;
#[doc = "MEM_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ctrl`] module"]
pub type MEM_CTRL = crate::Reg<mem_ctrl::MEM_CTRL_SPEC>;
#[doc = "need_des"]
pub mod mem_ctrl;
#[doc = "ADC_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ctrl`] module"]
pub type ADC_CTRL = crate::Reg<adc_ctrl::ADC_CTRL_SPEC>;
#[doc = "need_des"]
pub mod adc_ctrl;
#[doc = "LP_I2S_RXCLK_DIV_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_rxclk_div_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_rxclk_div_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_rxclk_div_num`] module"]
pub type LP_I2S_RXCLK_DIV_NUM = crate::Reg<lp_i2s_rxclk_div_num::LP_I2S_RXCLK_DIV_NUM_SPEC>;
#[doc = "need_des"]
pub mod lp_i2s_rxclk_div_num;
#[doc = "LP_I2S_RXCLK_DIV_XYZ (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_rxclk_div_xyz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_rxclk_div_xyz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_rxclk_div_xyz`] module"]
pub type LP_I2S_RXCLK_DIV_XYZ = crate::Reg<lp_i2s_rxclk_div_xyz::LP_I2S_RXCLK_DIV_XYZ_SPEC>;
#[doc = "need_des"]
pub mod lp_i2s_rxclk_div_xyz;
#[doc = "LP_I2S_TXCLK_DIV_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_txclk_div_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_txclk_div_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_txclk_div_num`] module"]
pub type LP_I2S_TXCLK_DIV_NUM = crate::Reg<lp_i2s_txclk_div_num::LP_I2S_TXCLK_DIV_NUM_SPEC>;
#[doc = "need_des"]
pub mod lp_i2s_txclk_div_num;
#[doc = "LP_I2S_TXCLK_DIV_XYZ (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_txclk_div_xyz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_txclk_div_xyz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_txclk_div_xyz`] module"]
pub type LP_I2S_TXCLK_DIV_XYZ = crate::Reg<lp_i2s_txclk_div_xyz::LP_I2S_TXCLK_DIV_XYZ_SPEC>;
#[doc = "need_des"]
pub mod lp_i2s_txclk_div_xyz;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
