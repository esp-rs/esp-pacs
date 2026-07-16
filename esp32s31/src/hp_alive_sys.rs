#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    hp_clk_ctrl: HP_CLK_CTRL,
    hp_pad_parlio_ctrl: HP_PAD_PARLIO_CTRL,
    hp_pad_i2s0_ctrl: HP_PAD_I2S0_CTRL,
    hp_pad_i2s1_ctrl: HP_PAD_I2S1_CTRL,
    hp_pad_uart0_ctrl: HP_PAD_UART0_CTRL,
    hp_pad_uart1_ctrl: HP_PAD_UART1_CTRL,
    hp_pad_uart2_ctrl: HP_PAD_UART2_CTRL,
    hp_pad_uart3_ctrl: HP_PAD_UART3_CTRL,
    modem_clk_aon: MODEM_CLK_AON,
    usb_ctrl: USB_CTRL,
    hp_tcm_mem_lp_ctrl: HP_TCM_MEM_LP_CTRL,
    flash_mmu_mem_lp_ctrl: FLASH_MMU_MEM_LP_CTRL,
    psram_mmu_mem_lp_ctrl: PSRAM_MMU_MEM_LP_CTRL,
    spram_mem_aux_ctrl: SPRAM_MEM_AUX_CTRL,
    sprf_mem_aux_ctrl: SPRF_MEM_AUX_CTRL,
    _reserved15: [u8; 0x04],
    sdprf_mem_aux_ctrl: SDPRF_MEM_AUX_CTRL,
    cpu_sprom_mem_aux_ctrl: CPU_SPROM_MEM_AUX_CTRL,
    cpu_sprf_mem_aux_ctrl: CPU_SPRF_MEM_AUX_CTRL,
    _reserved18: [u8; 0x30],
    intr_hp2lp_conf_0: INTR_HP2LP_CONF_0,
    intr_hp2lp_conf_1: INTR_HP2LP_CONF_1,
    intr_hp2lp_conf_2: INTR_HP2LP_CONF_2,
    intr_hp2lp_conf_3: INTR_HP2LP_CONF_3,
    intr_hp2lp_conf_4: INTR_HP2LP_CONF_4,
    intr_hp2lp_status_0: INTR_HP2LP_STATUS_0,
    intr_hp2lp_status_1: INTR_HP2LP_STATUS_1,
    intr_hp2lp_status_2: INTR_HP2LP_STATUS_2,
    intr_hp2lp_status_3: INTR_HP2LP_STATUS_3,
    intr_hp2lp_status_4: INTR_HP2LP_STATUS_4,
    cpu_spram_mem_aux_ctrl: CPU_SPRAM_MEM_AUX_CTRL,
    bus_clock_gate_bypass: BUS_CLOCK_GATE_BYPASS,
    rst_event_bypass: RST_EVENT_BYPASS,
    usb_otghs_ctrl: USB_OTGHS_CTRL,
    _reserved32: [u8; 0x0348],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_clk_ctrl(&self) -> &HP_CLK_CTRL {
        &self.hp_clk_ctrl
    }
    #[doc = "0x04 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_pad_parlio_ctrl(&self) -> &HP_PAD_PARLIO_CTRL {
        &self.hp_pad_parlio_ctrl
    }
    #[doc = "0x08 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_pad_i2s0_ctrl(&self) -> &HP_PAD_I2S0_CTRL {
        &self.hp_pad_i2s0_ctrl
    }
    #[doc = "0x0c - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_pad_i2s1_ctrl(&self) -> &HP_PAD_I2S1_CTRL {
        &self.hp_pad_i2s1_ctrl
    }
    #[doc = "0x10 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_pad_uart0_ctrl(&self) -> &HP_PAD_UART0_CTRL {
        &self.hp_pad_uart0_ctrl
    }
    #[doc = "0x14 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_pad_uart1_ctrl(&self) -> &HP_PAD_UART1_CTRL {
        &self.hp_pad_uart1_ctrl
    }
    #[doc = "0x18 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_pad_uart2_ctrl(&self) -> &HP_PAD_UART2_CTRL {
        &self.hp_pad_uart2_ctrl
    }
    #[doc = "0x1c - HP Clock Control Register."]
    #[inline(always)]
    pub const fn hp_pad_uart3_ctrl(&self) -> &HP_PAD_UART3_CTRL {
        &self.hp_pad_uart3_ctrl
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn modem_clk_aon(&self) -> &MODEM_CLK_AON {
        &self.modem_clk_aon
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn usb_ctrl(&self) -> &USB_CTRL {
        &self.usb_ctrl
    }
    #[doc = "0x28 - configure rmemory power in lp system register"]
    #[inline(always)]
    pub const fn hp_tcm_mem_lp_ctrl(&self) -> &HP_TCM_MEM_LP_CTRL {
        &self.hp_tcm_mem_lp_ctrl
    }
    #[doc = "0x2c - configure rmemory power in lp system register"]
    #[inline(always)]
    pub const fn flash_mmu_mem_lp_ctrl(&self) -> &FLASH_MMU_MEM_LP_CTRL {
        &self.flash_mmu_mem_lp_ctrl
    }
    #[doc = "0x30 - configure rmemory power in lp system register"]
    #[inline(always)]
    pub const fn psram_mmu_mem_lp_ctrl(&self) -> &PSRAM_MMU_MEM_LP_CTRL {
        &self.psram_mmu_mem_lp_ctrl
    }
    #[doc = "0x34 - spram mem aux ctrl , control TOP/CNNT/MODEM power domain"]
    #[inline(always)]
    pub const fn spram_mem_aux_ctrl(&self) -> &SPRAM_MEM_AUX_CTRL {
        &self.spram_mem_aux_ctrl
    }
    #[doc = "0x38 - sprf mem aux ctrl , control TOP/CNNT power domain"]
    #[inline(always)]
    pub const fn sprf_mem_aux_ctrl(&self) -> &SPRF_MEM_AUX_CTRL {
        &self.sprf_mem_aux_ctrl
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn sdprf_mem_aux_ctrl(&self) -> &SDPRF_MEM_AUX_CTRL {
        &self.sdprf_mem_aux_ctrl
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn cpu_sprom_mem_aux_ctrl(&self) -> &CPU_SPROM_MEM_AUX_CTRL {
        &self.cpu_sprom_mem_aux_ctrl
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn cpu_sprf_mem_aux_ctrl(&self) -> &CPU_SPRF_MEM_AUX_CTRL {
        &self.cpu_sprf_mem_aux_ctrl
    }
    #[doc = "0x7c - intr hp2lp configuration register 0"]
    #[inline(always)]
    pub const fn intr_hp2lp_conf_0(&self) -> &INTR_HP2LP_CONF_0 {
        &self.intr_hp2lp_conf_0
    }
    #[doc = "0x80 - intr hp2lp configuration register 1"]
    #[inline(always)]
    pub const fn intr_hp2lp_conf_1(&self) -> &INTR_HP2LP_CONF_1 {
        &self.intr_hp2lp_conf_1
    }
    #[doc = "0x84 - intr hp2lp configuration register 2"]
    #[inline(always)]
    pub const fn intr_hp2lp_conf_2(&self) -> &INTR_HP2LP_CONF_2 {
        &self.intr_hp2lp_conf_2
    }
    #[doc = "0x88 - intr hp2lp configuration register 3"]
    #[inline(always)]
    pub const fn intr_hp2lp_conf_3(&self) -> &INTR_HP2LP_CONF_3 {
        &self.intr_hp2lp_conf_3
    }
    #[doc = "0x8c - intr hp2lp configuration register 4"]
    #[inline(always)]
    pub const fn intr_hp2lp_conf_4(&self) -> &INTR_HP2LP_CONF_4 {
        &self.intr_hp2lp_conf_4
    }
    #[doc = "0x90 - intr hp2lp status register 0"]
    #[inline(always)]
    pub const fn intr_hp2lp_status_0(&self) -> &INTR_HP2LP_STATUS_0 {
        &self.intr_hp2lp_status_0
    }
    #[doc = "0x94 - intr hp2lp status register 1"]
    #[inline(always)]
    pub const fn intr_hp2lp_status_1(&self) -> &INTR_HP2LP_STATUS_1 {
        &self.intr_hp2lp_status_1
    }
    #[doc = "0x98 - intr hp2lp status register 2"]
    #[inline(always)]
    pub const fn intr_hp2lp_status_2(&self) -> &INTR_HP2LP_STATUS_2 {
        &self.intr_hp2lp_status_2
    }
    #[doc = "0x9c - intr hp2lp status register 3"]
    #[inline(always)]
    pub const fn intr_hp2lp_status_3(&self) -> &INTR_HP2LP_STATUS_3 {
        &self.intr_hp2lp_status_3
    }
    #[doc = "0xa0 - intr hp2lp status register 4"]
    #[inline(always)]
    pub const fn intr_hp2lp_status_4(&self) -> &INTR_HP2LP_STATUS_4 {
        &self.intr_hp2lp_status_4
    }
    #[doc = "0xa4 - spram mem aux ctrl , control CPU power domain"]
    #[inline(always)]
    pub const fn cpu_spram_mem_aux_ctrl(&self) -> &CPU_SPRAM_MEM_AUX_CTRL {
        &self.cpu_spram_mem_aux_ctrl
    }
    #[doc = "0xa8 - bus clock gating bypass configuration register"]
    #[inline(always)]
    pub const fn bus_clock_gate_bypass(&self) -> &BUS_CLOCK_GATE_BYPASS {
        &self.bus_clock_gate_bypass
    }
    #[doc = "0xac - bus clock gating bypass configuration register"]
    #[inline(always)]
    pub const fn rst_event_bypass(&self) -> &RST_EVENT_BYPASS {
        &self.rst_event_bypass
    }
    #[doc = "0xb0 - usb_otghs Control configuration register"]
    #[inline(always)]
    pub const fn usb_otghs_ctrl(&self) -> &USB_OTGHS_CTRL {
        &self.usb_otghs_ctrl
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "HP_CLK_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_clk_ctrl`] module"]
pub type HP_CLK_CTRL = crate::Reg<hp_clk_ctrl::HP_CLK_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_clk_ctrl;
#[doc = "HP_PAD_PARLIO_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_parlio_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_parlio_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pad_parlio_ctrl`] module"]
pub type HP_PAD_PARLIO_CTRL = crate::Reg<hp_pad_parlio_ctrl::HP_PAD_PARLIO_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_pad_parlio_ctrl;
#[doc = "HP_PAD_I2S0_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_i2s0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_i2s0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pad_i2s0_ctrl`] module"]
pub type HP_PAD_I2S0_CTRL = crate::Reg<hp_pad_i2s0_ctrl::HP_PAD_I2S0_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_pad_i2s0_ctrl;
#[doc = "HP_PAD_I2S1_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_i2s1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_i2s1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pad_i2s1_ctrl`] module"]
pub type HP_PAD_I2S1_CTRL = crate::Reg<hp_pad_i2s1_ctrl::HP_PAD_I2S1_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_pad_i2s1_ctrl;
#[doc = "HP_PAD_UART0_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_uart0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_uart0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pad_uart0_ctrl`] module"]
pub type HP_PAD_UART0_CTRL = crate::Reg<hp_pad_uart0_ctrl::HP_PAD_UART0_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_pad_uart0_ctrl;
#[doc = "HP_PAD_UART1_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_uart1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_uart1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pad_uart1_ctrl`] module"]
pub type HP_PAD_UART1_CTRL = crate::Reg<hp_pad_uart1_ctrl::HP_PAD_UART1_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_pad_uart1_ctrl;
#[doc = "HP_PAD_UART2_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_uart2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_uart2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pad_uart2_ctrl`] module"]
pub type HP_PAD_UART2_CTRL = crate::Reg<hp_pad_uart2_ctrl::HP_PAD_UART2_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_pad_uart2_ctrl;
#[doc = "HP_PAD_UART3_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_uart3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_uart3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pad_uart3_ctrl`] module"]
pub type HP_PAD_UART3_CTRL = crate::Reg<hp_pad_uart3_ctrl::HP_PAD_UART3_CTRL_SPEC>;
#[doc = "HP Clock Control Register."]
pub mod hp_pad_uart3_ctrl;
#[doc = "MODEM_CLK_AON (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_clk_aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_clk_aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_clk_aon`] module"]
pub type MODEM_CLK_AON = crate::Reg<modem_clk_aon::MODEM_CLK_AON_SPEC>;
#[doc = "need_des"]
pub mod modem_clk_aon;
#[doc = "USB_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ctrl`] module"]
pub type USB_CTRL = crate::Reg<usb_ctrl::USB_CTRL_SPEC>;
#[doc = "need_des"]
pub mod usb_ctrl;
#[doc = "HP_TCM_MEM_LP_CTRL (rw) register accessor: configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_mem_lp_ctrl`] module"]
pub type HP_TCM_MEM_LP_CTRL = crate::Reg<hp_tcm_mem_lp_ctrl::HP_TCM_MEM_LP_CTRL_SPEC>;
#[doc = "configure rmemory power in lp system register"]
pub mod hp_tcm_mem_lp_ctrl;
#[doc = "FLASH_MMU_MEM_LP_CTRL (rw) register accessor: configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_mmu_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_mmu_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_mmu_mem_lp_ctrl`] module"]
pub type FLASH_MMU_MEM_LP_CTRL = crate::Reg<flash_mmu_mem_lp_ctrl::FLASH_MMU_MEM_LP_CTRL_SPEC>;
#[doc = "configure rmemory power in lp system register"]
pub mod flash_mmu_mem_lp_ctrl;
#[doc = "PSRAM_MMU_MEM_LP_CTRL (rw) register accessor: configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_mmu_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_mmu_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_mmu_mem_lp_ctrl`] module"]
pub type PSRAM_MMU_MEM_LP_CTRL = crate::Reg<psram_mmu_mem_lp_ctrl::PSRAM_MMU_MEM_LP_CTRL_SPEC>;
#[doc = "configure rmemory power in lp system register"]
pub mod psram_mmu_mem_lp_ctrl;
#[doc = "SPRAM_MEM_AUX_CTRL (rw) register accessor: spram mem aux ctrl , control TOP/CNNT/MODEM power domain\n\nYou can [`read`](crate::Reg::read) this register and get [`spram_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spram_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spram_mem_aux_ctrl`] module"]
pub type SPRAM_MEM_AUX_CTRL = crate::Reg<spram_mem_aux_ctrl::SPRAM_MEM_AUX_CTRL_SPEC>;
#[doc = "spram mem aux ctrl , control TOP/CNNT/MODEM power domain"]
pub mod spram_mem_aux_ctrl;
#[doc = "SPRF_MEM_AUX_CTRL (rw) register accessor: sprf mem aux ctrl , control TOP/CNNT power domain\n\nYou can [`read`](crate::Reg::read) this register and get [`sprf_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprf_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprf_mem_aux_ctrl`] module"]
pub type SPRF_MEM_AUX_CTRL = crate::Reg<sprf_mem_aux_ctrl::SPRF_MEM_AUX_CTRL_SPEC>;
#[doc = "sprf mem aux ctrl , control TOP/CNNT power domain"]
pub mod sprf_mem_aux_ctrl;
#[doc = "SDPRF_MEM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sdprf_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdprf_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdprf_mem_aux_ctrl`] module"]
pub type SDPRF_MEM_AUX_CTRL = crate::Reg<sdprf_mem_aux_ctrl::SDPRF_MEM_AUX_CTRL_SPEC>;
#[doc = "need_des"]
pub mod sdprf_mem_aux_ctrl;
#[doc = "CPU_SPROM_MEM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sprom_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_sprom_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_sprom_mem_aux_ctrl`] module"]
pub type CPU_SPROM_MEM_AUX_CTRL = crate::Reg<cpu_sprom_mem_aux_ctrl::CPU_SPROM_MEM_AUX_CTRL_SPEC>;
#[doc = "need_des"]
pub mod cpu_sprom_mem_aux_ctrl;
#[doc = "CPU_SPRF_MEM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sprf_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_sprf_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_sprf_mem_aux_ctrl`] module"]
pub type CPU_SPRF_MEM_AUX_CTRL = crate::Reg<cpu_sprf_mem_aux_ctrl::CPU_SPRF_MEM_AUX_CTRL_SPEC>;
#[doc = "need_des"]
pub mod cpu_sprf_mem_aux_ctrl;
#[doc = "INTR_HP2LP_CONF_0 (rw) register accessor: intr hp2lp configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_conf_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_hp2lp_conf_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_conf_0`] module"]
pub type INTR_HP2LP_CONF_0 = crate::Reg<intr_hp2lp_conf_0::INTR_HP2LP_CONF_0_SPEC>;
#[doc = "intr hp2lp configuration register 0"]
pub mod intr_hp2lp_conf_0;
#[doc = "INTR_HP2LP_CONF_1 (rw) register accessor: intr hp2lp configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_conf_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_hp2lp_conf_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_conf_1`] module"]
pub type INTR_HP2LP_CONF_1 = crate::Reg<intr_hp2lp_conf_1::INTR_HP2LP_CONF_1_SPEC>;
#[doc = "intr hp2lp configuration register 1"]
pub mod intr_hp2lp_conf_1;
#[doc = "INTR_HP2LP_CONF_2 (rw) register accessor: intr hp2lp configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_conf_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_hp2lp_conf_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_conf_2`] module"]
pub type INTR_HP2LP_CONF_2 = crate::Reg<intr_hp2lp_conf_2::INTR_HP2LP_CONF_2_SPEC>;
#[doc = "intr hp2lp configuration register 2"]
pub mod intr_hp2lp_conf_2;
#[doc = "INTR_HP2LP_CONF_3 (rw) register accessor: intr hp2lp configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_conf_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_hp2lp_conf_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_conf_3`] module"]
pub type INTR_HP2LP_CONF_3 = crate::Reg<intr_hp2lp_conf_3::INTR_HP2LP_CONF_3_SPEC>;
#[doc = "intr hp2lp configuration register 3"]
pub mod intr_hp2lp_conf_3;
#[doc = "INTR_HP2LP_CONF_4 (rw) register accessor: intr hp2lp configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_conf_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_hp2lp_conf_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_conf_4`] module"]
pub type INTR_HP2LP_CONF_4 = crate::Reg<intr_hp2lp_conf_4::INTR_HP2LP_CONF_4_SPEC>;
#[doc = "intr hp2lp configuration register 4"]
pub mod intr_hp2lp_conf_4;
#[doc = "INTR_HP2LP_STATUS_0 (r) register accessor: intr hp2lp status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_status_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_status_0`] module"]
pub type INTR_HP2LP_STATUS_0 = crate::Reg<intr_hp2lp_status_0::INTR_HP2LP_STATUS_0_SPEC>;
#[doc = "intr hp2lp status register 0"]
pub mod intr_hp2lp_status_0;
#[doc = "INTR_HP2LP_STATUS_1 (r) register accessor: intr hp2lp status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_status_1`] module"]
pub type INTR_HP2LP_STATUS_1 = crate::Reg<intr_hp2lp_status_1::INTR_HP2LP_STATUS_1_SPEC>;
#[doc = "intr hp2lp status register 1"]
pub mod intr_hp2lp_status_1;
#[doc = "INTR_HP2LP_STATUS_2 (r) register accessor: intr hp2lp status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_status_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_status_2`] module"]
pub type INTR_HP2LP_STATUS_2 = crate::Reg<intr_hp2lp_status_2::INTR_HP2LP_STATUS_2_SPEC>;
#[doc = "intr hp2lp status register 2"]
pub mod intr_hp2lp_status_2;
#[doc = "INTR_HP2LP_STATUS_3 (r) register accessor: intr hp2lp status register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_status_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_status_3`] module"]
pub type INTR_HP2LP_STATUS_3 = crate::Reg<intr_hp2lp_status_3::INTR_HP2LP_STATUS_3_SPEC>;
#[doc = "intr hp2lp status register 3"]
pub mod intr_hp2lp_status_3;
#[doc = "INTR_HP2LP_STATUS_4 (r) register accessor: intr hp2lp status register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_status_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_hp2lp_status_4`] module"]
pub type INTR_HP2LP_STATUS_4 = crate::Reg<intr_hp2lp_status_4::INTR_HP2LP_STATUS_4_SPEC>;
#[doc = "intr hp2lp status register 4"]
pub mod intr_hp2lp_status_4;
#[doc = "CPU_SPRAM_MEM_AUX_CTRL (rw) register accessor: spram mem aux ctrl , control CPU power domain\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_spram_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_spram_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_spram_mem_aux_ctrl`] module"]
pub type CPU_SPRAM_MEM_AUX_CTRL = crate::Reg<cpu_spram_mem_aux_ctrl::CPU_SPRAM_MEM_AUX_CTRL_SPEC>;
#[doc = "spram mem aux ctrl , control CPU power domain"]
pub mod cpu_spram_mem_aux_ctrl;
#[doc = "BUS_CLOCK_GATE_BYPASS (rw) register accessor: bus clock gating bypass configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clock_gate_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clock_gate_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_clock_gate_bypass`] module"]
pub type BUS_CLOCK_GATE_BYPASS = crate::Reg<bus_clock_gate_bypass::BUS_CLOCK_GATE_BYPASS_SPEC>;
#[doc = "bus clock gating bypass configuration register"]
pub mod bus_clock_gate_bypass;
#[doc = "RST_EVENT_BYPASS (rw) register accessor: bus clock gating bypass configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_event_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_event_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_event_bypass`] module"]
pub type RST_EVENT_BYPASS = crate::Reg<rst_event_bypass::RST_EVENT_BYPASS_SPEC>;
#[doc = "bus clock gating bypass configuration register"]
pub mod rst_event_bypass;
#[doc = "USB_OTGHS_CTRL (rw) register accessor: usb_otghs Control configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otghs_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otghs_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otghs_ctrl`] module"]
pub type USB_OTGHS_CTRL = crate::Reg<usb_otghs_ctrl::USB_OTGHS_CTRL_SPEC>;
#[doc = "usb_otghs Control configuration register"]
pub mod usb_otghs_ctrl;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
