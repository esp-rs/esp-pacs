#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    cpu_peri_clk_en: CPU_PERI_CLK_EN,
    cpu_peri_rst_en: CPU_PERI_RST_EN,
    cpu_per_conf: CPU_PER_CONF,
    mem_pd_mask: MEM_PD_MASK,
    perip_clk_en0: PERIP_CLK_EN0,
    perip_clk_en1: PERIP_CLK_EN1,
    perip_rst_en0: PERIP_RST_EN0,
    perip_rst_en1: PERIP_RST_EN1,
    bt_lpck_div_int: BT_LPCK_DIV_INT,
    bt_lpck_div_frac: BT_LPCK_DIV_FRAC,
    cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    rsa_pd_ctrl: RSA_PD_CTRL,
    edma_ctrl: EDMA_CTRL,
    cache_control: CACHE_CONTROL,
    external_device_encrypt_decrypt_control: EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL,
    rtc_fastmem_config: RTC_FASTMEM_CONFIG,
    rtc_fastmem_crc: RTC_FASTMEM_CRC,
    redundant_eco_ctrl: REDUNDANT_ECO_CTRL,
    clock_gate: CLOCK_GATE,
    sysclk_conf: SYSCLK_CONF,
    mem_pvt: MEM_PVT,
    comb_pvt_lvt_conf: COMB_PVT_LVT_CONF,
    comb_pvt_nvt_conf: COMB_PVT_NVT_CONF,
    comb_pvt_hvt_conf: COMB_PVT_HVT_CONF,
    comb_pvt_err_lvt_site0: COMB_PVT_ERR_LVT_SITE0,
    comb_pvt_err_nvt_site0: COMB_PVT_ERR_NVT_SITE0,
    comb_pvt_err_hvt_site0: COMB_PVT_ERR_HVT_SITE0,
    comb_pvt_err_lvt_site1: COMB_PVT_ERR_LVT_SITE1,
    comb_pvt_err_nvt_site1: COMB_PVT_ERR_NVT_SITE1,
    comb_pvt_err_hvt_site1: COMB_PVT_ERR_HVT_SITE1,
    comb_pvt_err_lvt_site2: COMB_PVT_ERR_LVT_SITE2,
    comb_pvt_err_nvt_site2: COMB_PVT_ERR_NVT_SITE2,
    comb_pvt_err_hvt_site2: COMB_PVT_ERR_HVT_SITE2,
    comb_pvt_err_lvt_site3: COMB_PVT_ERR_LVT_SITE3,
    comb_pvt_err_nvt_site3: COMB_PVT_ERR_NVT_SITE3,
    comb_pvt_err_hvt_site3: COMB_PVT_ERR_HVT_SITE3,
    _reserved39: [u8; 0x0f60],
    reg_date: REG_DATE,
}
impl RegisterBlock {
    ///0x00 - cpu_peripheral clock gating register
    #[inline(always)]
    pub const fn cpu_peri_clk_en(&self) -> &CPU_PERI_CLK_EN {
        &self.cpu_peri_clk_en
    }
    ///0x04 - cpu_peripheral reset register
    #[inline(always)]
    pub const fn cpu_peri_rst_en(&self) -> &CPU_PERI_RST_EN {
        &self.cpu_peri_rst_en
    }
    ///0x08 - cpu clock config register
    #[inline(always)]
    pub const fn cpu_per_conf(&self) -> &CPU_PER_CONF {
        &self.cpu_per_conf
    }
    ///0x0c - memory power down mask register
    #[inline(always)]
    pub const fn mem_pd_mask(&self) -> &MEM_PD_MASK {
        &self.mem_pd_mask
    }
    ///0x10 - peripheral clock gating register
    #[inline(always)]
    pub const fn perip_clk_en0(&self) -> &PERIP_CLK_EN0 {
        &self.perip_clk_en0
    }
    ///0x14 - peripheral clock gating register
    #[inline(always)]
    pub const fn perip_clk_en1(&self) -> &PERIP_CLK_EN1 {
        &self.perip_clk_en1
    }
    ///0x18 - reserved
    #[inline(always)]
    pub const fn perip_rst_en0(&self) -> &PERIP_RST_EN0 {
        &self.perip_rst_en0
    }
    ///0x1c - peripheral reset register
    #[inline(always)]
    pub const fn perip_rst_en1(&self) -> &PERIP_RST_EN1 {
        &self.perip_rst_en1
    }
    ///0x20 - clock config register
    #[inline(always)]
    pub const fn bt_lpck_div_int(&self) -> &BT_LPCK_DIV_INT {
        &self.bt_lpck_div_int
    }
    ///0x24 - low power clock configuration register
    #[inline(always)]
    pub const fn bt_lpck_div_frac(&self) -> &BT_LPCK_DIV_FRAC {
        &self.bt_lpck_div_frac
    }
    ///0x28 - interrupt generate register
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0(&self) -> &CPU_INTR_FROM_CPU_0 {
        &self.cpu_intr_from_cpu_0
    }
    ///0x2c - interrupt generate register
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1(&self) -> &CPU_INTR_FROM_CPU_1 {
        &self.cpu_intr_from_cpu_1
    }
    ///0x30 - interrupt generate register
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2(&self) -> &CPU_INTR_FROM_CPU_2 {
        &self.cpu_intr_from_cpu_2
    }
    ///0x34 - interrupt generate register
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3(&self) -> &CPU_INTR_FROM_CPU_3 {
        &self.cpu_intr_from_cpu_3
    }
    ///0x38 - rsa memory power control register
    #[inline(always)]
    pub const fn rsa_pd_ctrl(&self) -> &RSA_PD_CTRL {
        &self.rsa_pd_ctrl
    }
    ///0x3c - edma clcok and reset register
    #[inline(always)]
    pub const fn edma_ctrl(&self) -> &EDMA_CTRL {
        &self.edma_ctrl
    }
    ///0x40 - cache control register
    #[inline(always)]
    pub const fn cache_control(&self) -> &CACHE_CONTROL {
        &self.cache_control
    }
    ///0x44 - SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_REG
    #[inline(always)]
    pub const fn external_device_encrypt_decrypt_control(
        &self,
    ) -> &EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL {
        &self.external_device_encrypt_decrypt_control
    }
    ///0x48 - fast memory config register
    #[inline(always)]
    pub const fn rtc_fastmem_config(&self) -> &RTC_FASTMEM_CONFIG {
        &self.rtc_fastmem_config
    }
    ///0x4c - reserved
    #[inline(always)]
    pub const fn rtc_fastmem_crc(&self) -> &RTC_FASTMEM_CRC {
        &self.rtc_fastmem_crc
    }
    ///0x50 - eco register
    #[inline(always)]
    pub const fn redundant_eco_ctrl(&self) -> &REDUNDANT_ECO_CTRL {
        &self.redundant_eco_ctrl
    }
    ///0x54 - clock gating register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x58 - system clock config register
    #[inline(always)]
    pub const fn sysclk_conf(&self) -> &SYSCLK_CONF {
        &self.sysclk_conf
    }
    ///0x5c - mem pvt register
    #[inline(always)]
    pub const fn mem_pvt(&self) -> &MEM_PVT {
        &self.mem_pvt
    }
    ///0x60 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_lvt_conf(&self) -> &COMB_PVT_LVT_CONF {
        &self.comb_pvt_lvt_conf
    }
    ///0x64 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_nvt_conf(&self) -> &COMB_PVT_NVT_CONF {
        &self.comb_pvt_nvt_conf
    }
    ///0x68 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_hvt_conf(&self) -> &COMB_PVT_HVT_CONF {
        &self.comb_pvt_hvt_conf
    }
    ///0x6c - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_lvt_site0(&self) -> &COMB_PVT_ERR_LVT_SITE0 {
        &self.comb_pvt_err_lvt_site0
    }
    ///0x70 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_nvt_site0(&self) -> &COMB_PVT_ERR_NVT_SITE0 {
        &self.comb_pvt_err_nvt_site0
    }
    ///0x74 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_hvt_site0(&self) -> &COMB_PVT_ERR_HVT_SITE0 {
        &self.comb_pvt_err_hvt_site0
    }
    ///0x78 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_lvt_site1(&self) -> &COMB_PVT_ERR_LVT_SITE1 {
        &self.comb_pvt_err_lvt_site1
    }
    ///0x7c - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_nvt_site1(&self) -> &COMB_PVT_ERR_NVT_SITE1 {
        &self.comb_pvt_err_nvt_site1
    }
    ///0x80 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_hvt_site1(&self) -> &COMB_PVT_ERR_HVT_SITE1 {
        &self.comb_pvt_err_hvt_site1
    }
    ///0x84 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_lvt_site2(&self) -> &COMB_PVT_ERR_LVT_SITE2 {
        &self.comb_pvt_err_lvt_site2
    }
    ///0x88 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_nvt_site2(&self) -> &COMB_PVT_ERR_NVT_SITE2 {
        &self.comb_pvt_err_nvt_site2
    }
    ///0x8c - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_hvt_site2(&self) -> &COMB_PVT_ERR_HVT_SITE2 {
        &self.comb_pvt_err_hvt_site2
    }
    ///0x90 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_lvt_site3(&self) -> &COMB_PVT_ERR_LVT_SITE3 {
        &self.comb_pvt_err_lvt_site3
    }
    ///0x94 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_nvt_site3(&self) -> &COMB_PVT_ERR_NVT_SITE3 {
        &self.comb_pvt_err_nvt_site3
    }
    ///0x98 - mem pvt register
    #[inline(always)]
    pub const fn comb_pvt_err_hvt_site3(&self) -> &COMB_PVT_ERR_HVT_SITE3 {
        &self.comb_pvt_err_hvt_site3
    }
    ///0xffc - Version register
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
/**CPU_PERI_CLK_EN (rw) register accessor: cpu_peripheral clock gating register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_peri_clk_en`] module*/
pub type CPU_PERI_CLK_EN = crate::Reg<cpu_peri_clk_en::CPU_PERI_CLK_EN_SPEC>;
///cpu_peripheral clock gating register
pub mod cpu_peri_clk_en;
/**CPU_PERI_RST_EN (rw) register accessor: cpu_peripheral reset register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_peri_rst_en`] module*/
pub type CPU_PERI_RST_EN = crate::Reg<cpu_peri_rst_en::CPU_PERI_RST_EN_SPEC>;
///cpu_peripheral reset register
pub mod cpu_peri_rst_en;
/**CPU_PER_CONF (rw) register accessor: cpu clock config register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_per_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_per_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_per_conf`] module*/
pub type CPU_PER_CONF = crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>;
///cpu clock config register
pub mod cpu_per_conf;
/**MEM_PD_MASK (rw) register accessor: memory power down mask register

You can [`read`](crate::generic::Reg::read) this register and get [`mem_pd_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pd_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_pd_mask`] module*/
pub type MEM_PD_MASK = crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>;
///memory power down mask register
pub mod mem_pd_mask;
/**PERIP_CLK_EN0 (rw) register accessor: peripheral clock gating register

You can [`read`](crate::generic::Reg::read) this register and get [`perip_clk_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_clk_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perip_clk_en0`] module*/
pub type PERIP_CLK_EN0 = crate::Reg<perip_clk_en0::PERIP_CLK_EN0_SPEC>;
///peripheral clock gating register
pub mod perip_clk_en0;
/**PERIP_CLK_EN1 (rw) register accessor: peripheral clock gating register

You can [`read`](crate::generic::Reg::read) this register and get [`perip_clk_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_clk_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perip_clk_en1`] module*/
pub type PERIP_CLK_EN1 = crate::Reg<perip_clk_en1::PERIP_CLK_EN1_SPEC>;
///peripheral clock gating register
pub mod perip_clk_en1;
/**PERIP_RST_EN0 (rw) register accessor: reserved

You can [`read`](crate::generic::Reg::read) this register and get [`perip_rst_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_rst_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perip_rst_en0`] module*/
pub type PERIP_RST_EN0 = crate::Reg<perip_rst_en0::PERIP_RST_EN0_SPEC>;
///reserved
pub mod perip_rst_en0;
/**PERIP_RST_EN1 (rw) register accessor: peripheral reset register

You can [`read`](crate::generic::Reg::read) this register and get [`perip_rst_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_rst_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perip_rst_en1`] module*/
pub type PERIP_RST_EN1 = crate::Reg<perip_rst_en1::PERIP_RST_EN1_SPEC>;
///peripheral reset register
pub mod perip_rst_en1;
/**BT_LPCK_DIV_INT (rw) register accessor: clock config register

You can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_lpck_div_int`] module*/
pub type BT_LPCK_DIV_INT = crate::Reg<bt_lpck_div_int::BT_LPCK_DIV_INT_SPEC>;
///clock config register
pub mod bt_lpck_div_int;
/**BT_LPCK_DIV_FRAC (rw) register accessor: low power clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_frac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_frac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_lpck_div_frac`] module*/
pub type BT_LPCK_DIV_FRAC = crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>;
///low power clock configuration register
pub mod bt_lpck_div_frac;
/**CPU_INTR_FROM_CPU_0 (rw) register accessor: interrupt generate register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_0`] module*/
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
///interrupt generate register
pub mod cpu_intr_from_cpu_0;
/**CPU_INTR_FROM_CPU_1 (rw) register accessor: interrupt generate register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_1`] module*/
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
///interrupt generate register
pub mod cpu_intr_from_cpu_1;
/**CPU_INTR_FROM_CPU_2 (rw) register accessor: interrupt generate register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_2`] module*/
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
///interrupt generate register
pub mod cpu_intr_from_cpu_2;
/**CPU_INTR_FROM_CPU_3 (rw) register accessor: interrupt generate register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_3`] module*/
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
///interrupt generate register
pub mod cpu_intr_from_cpu_3;
/**RSA_PD_CTRL (rw) register accessor: rsa memory power control register

You can [`read`](crate::generic::Reg::read) this register and get [`rsa_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rsa_pd_ctrl`] module*/
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
///rsa memory power control register
pub mod rsa_pd_ctrl;
/**EDMA_CTRL (rw) register accessor: edma clcok and reset register

You can [`read`](crate::generic::Reg::read) this register and get [`edma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edma_ctrl`] module*/
pub type EDMA_CTRL = crate::Reg<edma_ctrl::EDMA_CTRL_SPEC>;
///edma clcok and reset register
pub mod edma_ctrl;
/**CACHE_CONTROL (rw) register accessor: cache control register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_control`] module*/
pub type CACHE_CONTROL = crate::Reg<cache_control::CACHE_CONTROL_SPEC>;
///cache control register
pub mod cache_control;
/**EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL (rw) register accessor: SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_REG

You can [`read`](crate::generic::Reg::read) this register and get [`external_device_encrypt_decrypt_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`external_device_encrypt_decrypt_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@external_device_encrypt_decrypt_control`] module*/
pub type EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL = crate::Reg<
    external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
>;
///SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_REG
pub mod external_device_encrypt_decrypt_control;
/**RTC_FASTMEM_CONFIG (rw) register accessor: fast memory config register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_fastmem_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_fastmem_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_fastmem_config`] module*/
pub type RTC_FASTMEM_CONFIG = crate::Reg<rtc_fastmem_config::RTC_FASTMEM_CONFIG_SPEC>;
///fast memory config register
pub mod rtc_fastmem_config;
/**RTC_FASTMEM_CRC (r) register accessor: reserved

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_fastmem_crc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_fastmem_crc`] module*/
pub type RTC_FASTMEM_CRC = crate::Reg<rtc_fastmem_crc::RTC_FASTMEM_CRC_SPEC>;
///reserved
pub mod rtc_fastmem_crc;
/**REDUNDANT_ECO_CTRL (rw) register accessor: eco register

You can [`read`](crate::generic::Reg::read) this register and get [`redundant_eco_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundant_eco_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@redundant_eco_ctrl`] module*/
pub type REDUNDANT_ECO_CTRL = crate::Reg<redundant_eco_ctrl::REDUNDANT_ECO_CTRL_SPEC>;
///eco register
pub mod redundant_eco_ctrl;
/**CLOCK_GATE (rw) register accessor: clock gating register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///clock gating register
pub mod clock_gate;
/**SYSCLK_CONF (rw) register accessor: system clock config register

You can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sysclk_conf`] module*/
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
///system clock config register
pub mod sysclk_conf;
/**MEM_PVT (rw) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`mem_pvt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pvt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_pvt`] module*/
pub type MEM_PVT = crate::Reg<mem_pvt::MEM_PVT_SPEC>;
///mem pvt register
pub mod mem_pvt;
/**COMB_PVT_LVT_CONF (rw) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_lvt_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pvt_lvt_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_lvt_conf`] module*/
pub type COMB_PVT_LVT_CONF = crate::Reg<comb_pvt_lvt_conf::COMB_PVT_LVT_CONF_SPEC>;
///mem pvt register
pub mod comb_pvt_lvt_conf;
/**COMB_PVT_NVT_CONF (rw) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_nvt_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pvt_nvt_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_nvt_conf`] module*/
pub type COMB_PVT_NVT_CONF = crate::Reg<comb_pvt_nvt_conf::COMB_PVT_NVT_CONF_SPEC>;
///mem pvt register
pub mod comb_pvt_nvt_conf;
/**COMB_PVT_HVT_CONF (rw) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_hvt_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pvt_hvt_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_hvt_conf`] module*/
pub type COMB_PVT_HVT_CONF = crate::Reg<comb_pvt_hvt_conf::COMB_PVT_HVT_CONF_SPEC>;
///mem pvt register
pub mod comb_pvt_hvt_conf;
/**COMB_PVT_ERR_LVT_SITE0 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_lvt_site0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_lvt_site0`] module*/
pub type COMB_PVT_ERR_LVT_SITE0 = crate::Reg<comb_pvt_err_lvt_site0::COMB_PVT_ERR_LVT_SITE0_SPEC>;
///mem pvt register
pub mod comb_pvt_err_lvt_site0;
/**COMB_PVT_ERR_NVT_SITE0 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_nvt_site0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_nvt_site0`] module*/
pub type COMB_PVT_ERR_NVT_SITE0 = crate::Reg<comb_pvt_err_nvt_site0::COMB_PVT_ERR_NVT_SITE0_SPEC>;
///mem pvt register
pub mod comb_pvt_err_nvt_site0;
/**COMB_PVT_ERR_HVT_SITE0 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_hvt_site0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_hvt_site0`] module*/
pub type COMB_PVT_ERR_HVT_SITE0 = crate::Reg<comb_pvt_err_hvt_site0::COMB_PVT_ERR_HVT_SITE0_SPEC>;
///mem pvt register
pub mod comb_pvt_err_hvt_site0;
/**COMB_PVT_ERR_LVT_SITE1 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_lvt_site1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_lvt_site1`] module*/
pub type COMB_PVT_ERR_LVT_SITE1 = crate::Reg<comb_pvt_err_lvt_site1::COMB_PVT_ERR_LVT_SITE1_SPEC>;
///mem pvt register
pub mod comb_pvt_err_lvt_site1;
/**COMB_PVT_ERR_NVT_SITE1 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_nvt_site1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_nvt_site1`] module*/
pub type COMB_PVT_ERR_NVT_SITE1 = crate::Reg<comb_pvt_err_nvt_site1::COMB_PVT_ERR_NVT_SITE1_SPEC>;
///mem pvt register
pub mod comb_pvt_err_nvt_site1;
/**COMB_PVT_ERR_HVT_SITE1 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_hvt_site1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_hvt_site1`] module*/
pub type COMB_PVT_ERR_HVT_SITE1 = crate::Reg<comb_pvt_err_hvt_site1::COMB_PVT_ERR_HVT_SITE1_SPEC>;
///mem pvt register
pub mod comb_pvt_err_hvt_site1;
/**COMB_PVT_ERR_LVT_SITE2 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_lvt_site2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_lvt_site2`] module*/
pub type COMB_PVT_ERR_LVT_SITE2 = crate::Reg<comb_pvt_err_lvt_site2::COMB_PVT_ERR_LVT_SITE2_SPEC>;
///mem pvt register
pub mod comb_pvt_err_lvt_site2;
/**COMB_PVT_ERR_NVT_SITE2 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_nvt_site2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_nvt_site2`] module*/
pub type COMB_PVT_ERR_NVT_SITE2 = crate::Reg<comb_pvt_err_nvt_site2::COMB_PVT_ERR_NVT_SITE2_SPEC>;
///mem pvt register
pub mod comb_pvt_err_nvt_site2;
/**COMB_PVT_ERR_HVT_SITE2 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_hvt_site2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_hvt_site2`] module*/
pub type COMB_PVT_ERR_HVT_SITE2 = crate::Reg<comb_pvt_err_hvt_site2::COMB_PVT_ERR_HVT_SITE2_SPEC>;
///mem pvt register
pub mod comb_pvt_err_hvt_site2;
/**COMB_PVT_ERR_LVT_SITE3 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_lvt_site3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_lvt_site3`] module*/
pub type COMB_PVT_ERR_LVT_SITE3 = crate::Reg<comb_pvt_err_lvt_site3::COMB_PVT_ERR_LVT_SITE3_SPEC>;
///mem pvt register
pub mod comb_pvt_err_lvt_site3;
/**COMB_PVT_ERR_NVT_SITE3 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_nvt_site3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_nvt_site3`] module*/
pub type COMB_PVT_ERR_NVT_SITE3 = crate::Reg<comb_pvt_err_nvt_site3::COMB_PVT_ERR_NVT_SITE3_SPEC>;
///mem pvt register
pub mod comb_pvt_err_nvt_site3;
/**COMB_PVT_ERR_HVT_SITE3 (r) register accessor: mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_hvt_site3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comb_pvt_err_hvt_site3`] module*/
pub type COMB_PVT_ERR_HVT_SITE3 = crate::Reg<comb_pvt_err_hvt_site3::COMB_PVT_ERR_HVT_SITE3_SPEC>;
///mem pvt register
pub mod comb_pvt_err_hvt_site3;
/**REG_DATE (rw) register accessor: Version register

You can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg_date`] module*/
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
///Version register
pub mod reg_date;
