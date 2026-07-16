#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lpcpu_cfg: LPCPU_CFG,
    lpcpu_sts: LPCPU_STS,
    lpcpu_ctrl: LPCPU_CTRL,
    lpcpu_wakeup_src: LPCPU_WAKEUP_SRC,
    lpcpu_reject_src: LPCPU_REJECT_SRC,
    lpcpu_wakeup_cause: LPCPU_WAKEUP_CAUSE,
    lpcpu_reject_cause: LPCPU_REJECT_CAUSE,
    lpcpu_cause_clr: LPCPU_CAUSE_CLR,
    appwr_pwr_cfg: APPWR_PWR_CFG,
    appwr_clk_cfg: APPWR_CLK_CFG,
    appwr_cfg: APPWR_CFG,
    appwr_sts: APPWR_STS,
    appwr_ctrl: APPWR_CTRL,
    appwr_wakeup_src: APPWR_WAKEUP_SRC,
    appwr_reject_src: APPWR_REJECT_SRC,
    appwr_wakeup_cause: APPWR_WAKEUP_CAUSE,
    appwr_reject_cause: APPWR_REJECT_CAUSE,
    appwr_cause_clr: APPWR_CAUSE_CLR,
    wifi_ctrl: WIFI_CTRL,
    wifi_wakeup_src: WIFI_WAKEUP_SRC,
    ble_ctrl: BLE_CTRL,
    ble_wakeup_src: BLE_WAKEUP_SRC,
    zb_ctrl: ZB_CTRL,
    zb_wakeup_src: ZB_WAKEUP_SRC,
    modempwr_ctrl: MODEMPWR_CTRL,
    modempwr_wakeup_src: MODEMPWR_WAKEUP_SRC,
    peri_pad_wake_slp_en: PERI_PAD_WAKE_SLP_EN,
    touch_ctrl: TOUCH_CTRL,
    touch_cfg: TOUCH_CFG,
    touch_sts: TOUCH_STS,
    cali_ctrl: CALI_CTRL,
    cali_sts: CALI_STS,
    lpperi_pwr_clk_cfg: LPPERI_PWR_CLK_CFG,
    lpperi_ctrl: LPPERI_CTRL,
    lp_pad_cfg: LP_PAD_CFG,
    cnnt_pad_cfg: CNNT_PAD_CFG,
    hp_pad_cfg: HP_PAD_CFG,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    ver_date: VER_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - config register for lpcpu power control"]
    #[inline(always)]
    pub const fn lpcpu_cfg(&self) -> &LPCPU_CFG {
        &self.lpcpu_cfg
    }
    #[doc = "0x04 - status register for LPCPU PWR"]
    #[inline(always)]
    pub const fn lpcpu_sts(&self) -> &LPCPU_STS {
        &self.lpcpu_sts
    }
    #[doc = "0x08 - ctrl register for lpcpu power control"]
    #[inline(always)]
    pub const fn lpcpu_ctrl(&self) -> &LPCPU_CTRL {
        &self.lpcpu_ctrl
    }
    #[doc = "0x0c - wakeup source register for lpcpu"]
    #[inline(always)]
    pub const fn lpcpu_wakeup_src(&self) -> &LPCPU_WAKEUP_SRC {
        &self.lpcpu_wakeup_src
    }
    #[doc = "0x10 - reject source register for lpcpu"]
    #[inline(always)]
    pub const fn lpcpu_reject_src(&self) -> &LPCPU_REJECT_SRC {
        &self.lpcpu_reject_src
    }
    #[doc = "0x14 - wakeup cause register for lpcpu"]
    #[inline(always)]
    pub const fn lpcpu_wakeup_cause(&self) -> &LPCPU_WAKEUP_CAUSE {
        &self.lpcpu_wakeup_cause
    }
    #[doc = "0x18 - reject cause register for lpcpu"]
    #[inline(always)]
    pub const fn lpcpu_reject_cause(&self) -> &LPCPU_REJECT_CAUSE {
        &self.lpcpu_reject_cause
    }
    #[doc = "0x1c - cause clear register for lpcpu"]
    #[inline(always)]
    pub const fn lpcpu_cause_clr(&self) -> &LPCPU_CAUSE_CLR {
        &self.lpcpu_cause_clr
    }
    #[doc = "0x20 - config register for apsys pwr and clk mode"]
    #[inline(always)]
    pub const fn appwr_pwr_cfg(&self) -> &APPWR_PWR_CFG {
        &self.appwr_pwr_cfg
    }
    #[doc = "0x24 - config register for apsys pwr and clk mode"]
    #[inline(always)]
    pub const fn appwr_clk_cfg(&self) -> &APPWR_CLK_CFG {
        &self.appwr_clk_cfg
    }
    #[doc = "0x28 - config register for apsys pwr"]
    #[inline(always)]
    pub const fn appwr_cfg(&self) -> &APPWR_CFG {
        &self.appwr_cfg
    }
    #[doc = "0x2c - status register for apsys pwr"]
    #[inline(always)]
    pub const fn appwr_sts(&self) -> &APPWR_STS {
        &self.appwr_sts
    }
    #[doc = "0x30 - ctrl register for appwr power control"]
    #[inline(always)]
    pub const fn appwr_ctrl(&self) -> &APPWR_CTRL {
        &self.appwr_ctrl
    }
    #[doc = "0x34 - wakeup source register for appwr"]
    #[inline(always)]
    pub const fn appwr_wakeup_src(&self) -> &APPWR_WAKEUP_SRC {
        &self.appwr_wakeup_src
    }
    #[doc = "0x38 - reject source register for appwr"]
    #[inline(always)]
    pub const fn appwr_reject_src(&self) -> &APPWR_REJECT_SRC {
        &self.appwr_reject_src
    }
    #[doc = "0x3c - wakeup cause register for appwr"]
    #[inline(always)]
    pub const fn appwr_wakeup_cause(&self) -> &APPWR_WAKEUP_CAUSE {
        &self.appwr_wakeup_cause
    }
    #[doc = "0x40 - reject cause register for appwr"]
    #[inline(always)]
    pub const fn appwr_reject_cause(&self) -> &APPWR_REJECT_CAUSE {
        &self.appwr_reject_cause
    }
    #[doc = "0x44 - cause clear register for appwr"]
    #[inline(always)]
    pub const fn appwr_cause_clr(&self) -> &APPWR_CAUSE_CLR {
        &self.appwr_cause_clr
    }
    #[doc = "0x48 - ctrl register for wifi power control"]
    #[inline(always)]
    pub const fn wifi_ctrl(&self) -> &WIFI_CTRL {
        &self.wifi_ctrl
    }
    #[doc = "0x4c - wakeup source register for wifi"]
    #[inline(always)]
    pub const fn wifi_wakeup_src(&self) -> &WIFI_WAKEUP_SRC {
        &self.wifi_wakeup_src
    }
    #[doc = "0x50 - ctrl register for ble power control"]
    #[inline(always)]
    pub const fn ble_ctrl(&self) -> &BLE_CTRL {
        &self.ble_ctrl
    }
    #[doc = "0x54 - wakeup source register for ble"]
    #[inline(always)]
    pub const fn ble_wakeup_src(&self) -> &BLE_WAKEUP_SRC {
        &self.ble_wakeup_src
    }
    #[doc = "0x58 - ctrl register for zb power control"]
    #[inline(always)]
    pub const fn zb_ctrl(&self) -> &ZB_CTRL {
        &self.zb_ctrl
    }
    #[doc = "0x5c - wakeup source register for zb"]
    #[inline(always)]
    pub const fn zb_wakeup_src(&self) -> &ZB_WAKEUP_SRC {
        &self.zb_wakeup_src
    }
    #[doc = "0x60 - ctrl register for modempwr power control"]
    #[inline(always)]
    pub const fn modempwr_ctrl(&self) -> &MODEMPWR_CTRL {
        &self.modempwr_ctrl
    }
    #[doc = "0x64 - wakeup source register for modempwr"]
    #[inline(always)]
    pub const fn modempwr_wakeup_src(&self) -> &MODEMPWR_WAKEUP_SRC {
        &self.modempwr_wakeup_src
    }
    #[doc = "0x68 - used for future potential eco, others don't care"]
    #[inline(always)]
    pub const fn peri_pad_wake_slp_en(&self) -> &PERI_PAD_WAKE_SLP_EN {
        &self.peri_pad_wake_slp_en
    }
    #[doc = "0x6c - ctrl register for touch power control"]
    #[inline(always)]
    pub const fn touch_ctrl(&self) -> &TOUCH_CTRL {
        &self.touch_ctrl
    }
    #[doc = "0x70 - ctrl register for touch power control"]
    #[inline(always)]
    pub const fn touch_cfg(&self) -> &TOUCH_CFG {
        &self.touch_cfg
    }
    #[doc = "0x74 - status register for TOUCH PWR"]
    #[inline(always)]
    pub const fn touch_sts(&self) -> &TOUCH_STS {
        &self.touch_sts
    }
    #[doc = "0x78 - ctrl register for cali power control"]
    #[inline(always)]
    pub const fn cali_ctrl(&self) -> &CALI_CTRL {
        &self.cali_ctrl
    }
    #[doc = "0x7c - status register for CALI PWR"]
    #[inline(always)]
    pub const fn cali_sts(&self) -> &CALI_STS {
        &self.cali_sts
    }
    #[doc = "0x80 - ctrl register for lpcpu power clock mode control"]
    #[inline(always)]
    pub const fn lpperi_pwr_clk_cfg(&self) -> &LPPERI_PWR_CLK_CFG {
        &self.lpperi_pwr_clk_cfg
    }
    #[doc = "0x84 - ctrl register for lpperi power control"]
    #[inline(always)]
    pub const fn lpperi_ctrl(&self) -> &LPPERI_CTRL {
        &self.lpperi_ctrl
    }
    #[doc = "0x88 - config register for lppad hold signal"]
    #[inline(always)]
    pub const fn lp_pad_cfg(&self) -> &LP_PAD_CFG {
        &self.lp_pad_cfg
    }
    #[doc = "0x8c - config register for cnntpad hold signal"]
    #[inline(always)]
    pub const fn cnnt_pad_cfg(&self) -> &CNNT_PAD_CFG {
        &self.cnnt_pad_cfg
    }
    #[doc = "0x90 - config register for hppad hold signal"]
    #[inline(always)]
    pub const fn hp_pad_cfg(&self) -> &HP_PAD_CFG {
        &self.hp_pad_cfg
    }
    #[doc = "0x94 - need_des"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x98 - need_des"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x9c - need_des"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xa0 - need_des"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xa4 - need_des"]
    #[inline(always)]
    pub const fn ver_date(&self) -> &VER_DATE {
        &self.ver_date
    }
}
#[doc = "LPCPU_CFG (rw) register accessor: config register for lpcpu power control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcpu_cfg`] module"]
pub type LPCPU_CFG = crate::Reg<lpcpu_cfg::LPCPU_CFG_SPEC>;
#[doc = "config register for lpcpu power control"]
pub mod lpcpu_cfg;
#[doc = "LPCPU_STS (r) register accessor: status register for LPCPU PWR\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcpu_sts`] module"]
pub type LPCPU_STS = crate::Reg<lpcpu_sts::LPCPU_STS_SPEC>;
#[doc = "status register for LPCPU PWR"]
pub mod lpcpu_sts;
#[doc = "LPCPU_CTRL (rw) register accessor: ctrl register for lpcpu power control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcpu_ctrl`] module"]
pub type LPCPU_CTRL = crate::Reg<lpcpu_ctrl::LPCPU_CTRL_SPEC>;
#[doc = "ctrl register for lpcpu power control"]
pub mod lpcpu_ctrl;
#[doc = "LPCPU_WAKEUP_SRC (rw) register accessor: wakeup source register for lpcpu\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_wakeup_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_wakeup_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcpu_wakeup_src`] module"]
pub type LPCPU_WAKEUP_SRC = crate::Reg<lpcpu_wakeup_src::LPCPU_WAKEUP_SRC_SPEC>;
#[doc = "wakeup source register for lpcpu"]
pub mod lpcpu_wakeup_src;
#[doc = "LPCPU_REJECT_SRC (rw) register accessor: reject source register for lpcpu\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_reject_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_reject_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcpu_reject_src`] module"]
pub type LPCPU_REJECT_SRC = crate::Reg<lpcpu_reject_src::LPCPU_REJECT_SRC_SPEC>;
#[doc = "reject source register for lpcpu"]
pub mod lpcpu_reject_src;
#[doc = "LPCPU_WAKEUP_CAUSE (r) register accessor: wakeup cause register for lpcpu\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_wakeup_cause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcpu_wakeup_cause`] module"]
pub type LPCPU_WAKEUP_CAUSE = crate::Reg<lpcpu_wakeup_cause::LPCPU_WAKEUP_CAUSE_SPEC>;
#[doc = "wakeup cause register for lpcpu"]
pub mod lpcpu_wakeup_cause;
#[doc = "LPCPU_REJECT_CAUSE (r) register accessor: reject cause register for lpcpu\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_reject_cause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcpu_reject_cause`] module"]
pub type LPCPU_REJECT_CAUSE = crate::Reg<lpcpu_reject_cause::LPCPU_REJECT_CAUSE_SPEC>;
#[doc = "reject cause register for lpcpu"]
pub mod lpcpu_reject_cause;
#[doc = "LPCPU_CAUSE_CLR (w) register accessor: cause clear register for lpcpu\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_cause_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcpu_cause_clr`] module"]
pub type LPCPU_CAUSE_CLR = crate::Reg<lpcpu_cause_clr::LPCPU_CAUSE_CLR_SPEC>;
#[doc = "cause clear register for lpcpu"]
pub mod lpcpu_cause_clr;
#[doc = "APPWR_PWR_CFG (rw) register accessor: config register for apsys pwr and clk mode\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_pwr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_pwr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_pwr_cfg`] module"]
pub type APPWR_PWR_CFG = crate::Reg<appwr_pwr_cfg::APPWR_PWR_CFG_SPEC>;
#[doc = "config register for apsys pwr and clk mode"]
pub mod appwr_pwr_cfg;
#[doc = "APPWR_CLK_CFG (rw) register accessor: config register for apsys pwr and clk mode\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_clk_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_clk_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_clk_cfg`] module"]
pub type APPWR_CLK_CFG = crate::Reg<appwr_clk_cfg::APPWR_CLK_CFG_SPEC>;
#[doc = "config register for apsys pwr and clk mode"]
pub mod appwr_clk_cfg;
#[doc = "APPWR_CFG (rw) register accessor: config register for apsys pwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_cfg`] module"]
pub type APPWR_CFG = crate::Reg<appwr_cfg::APPWR_CFG_SPEC>;
#[doc = "config register for apsys pwr"]
pub mod appwr_cfg;
#[doc = "APPWR_STS (r) register accessor: status register for apsys pwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_sts`] module"]
pub type APPWR_STS = crate::Reg<appwr_sts::APPWR_STS_SPEC>;
#[doc = "status register for apsys pwr"]
pub mod appwr_sts;
#[doc = "APPWR_CTRL (rw) register accessor: ctrl register for appwr power control\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_ctrl`] module"]
pub type APPWR_CTRL = crate::Reg<appwr_ctrl::APPWR_CTRL_SPEC>;
#[doc = "ctrl register for appwr power control"]
pub mod appwr_ctrl;
#[doc = "APPWR_WAKEUP_SRC (rw) register accessor: wakeup source register for appwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_wakeup_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_wakeup_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_wakeup_src`] module"]
pub type APPWR_WAKEUP_SRC = crate::Reg<appwr_wakeup_src::APPWR_WAKEUP_SRC_SPEC>;
#[doc = "wakeup source register for appwr"]
pub mod appwr_wakeup_src;
#[doc = "APPWR_REJECT_SRC (rw) register accessor: reject source register for appwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_reject_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_reject_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_reject_src`] module"]
pub type APPWR_REJECT_SRC = crate::Reg<appwr_reject_src::APPWR_REJECT_SRC_SPEC>;
#[doc = "reject source register for appwr"]
pub mod appwr_reject_src;
#[doc = "APPWR_WAKEUP_CAUSE (r) register accessor: wakeup cause register for appwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_wakeup_cause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_wakeup_cause`] module"]
pub type APPWR_WAKEUP_CAUSE = crate::Reg<appwr_wakeup_cause::APPWR_WAKEUP_CAUSE_SPEC>;
#[doc = "wakeup cause register for appwr"]
pub mod appwr_wakeup_cause;
#[doc = "APPWR_REJECT_CAUSE (r) register accessor: reject cause register for appwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_reject_cause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_reject_cause`] module"]
pub type APPWR_REJECT_CAUSE = crate::Reg<appwr_reject_cause::APPWR_REJECT_CAUSE_SPEC>;
#[doc = "reject cause register for appwr"]
pub mod appwr_reject_cause;
#[doc = "APPWR_CAUSE_CLR (w) register accessor: cause clear register for appwr\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_cause_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appwr_cause_clr`] module"]
pub type APPWR_CAUSE_CLR = crate::Reg<appwr_cause_clr::APPWR_CAUSE_CLR_SPEC>;
#[doc = "cause clear register for appwr"]
pub mod appwr_cause_clr;
#[doc = "WIFI_CTRL (w) register accessor: ctrl register for wifi power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_ctrl`] module"]
pub type WIFI_CTRL = crate::Reg<wifi_ctrl::WIFI_CTRL_SPEC>;
#[doc = "ctrl register for wifi power control"]
pub mod wifi_ctrl;
#[doc = "WIFI_WAKEUP_SRC (rw) register accessor: wakeup source register for wifi\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_wakeup_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_wakeup_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_wakeup_src`] module"]
pub type WIFI_WAKEUP_SRC = crate::Reg<wifi_wakeup_src::WIFI_WAKEUP_SRC_SPEC>;
#[doc = "wakeup source register for wifi"]
pub mod wifi_wakeup_src;
#[doc = "BLE_CTRL (w) register accessor: ctrl register for ble power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ble_ctrl`] module"]
pub type BLE_CTRL = crate::Reg<ble_ctrl::BLE_CTRL_SPEC>;
#[doc = "ctrl register for ble power control"]
pub mod ble_ctrl;
#[doc = "BLE_WAKEUP_SRC (rw) register accessor: wakeup source register for ble\n\nYou can [`read`](crate::Reg::read) this register and get [`ble_wakeup_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_wakeup_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ble_wakeup_src`] module"]
pub type BLE_WAKEUP_SRC = crate::Reg<ble_wakeup_src::BLE_WAKEUP_SRC_SPEC>;
#[doc = "wakeup source register for ble"]
pub mod ble_wakeup_src;
#[doc = "ZB_CTRL (w) register accessor: ctrl register for zb power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zb_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zb_ctrl`] module"]
pub type ZB_CTRL = crate::Reg<zb_ctrl::ZB_CTRL_SPEC>;
#[doc = "ctrl register for zb power control"]
pub mod zb_ctrl;
#[doc = "ZB_WAKEUP_SRC (rw) register accessor: wakeup source register for zb\n\nYou can [`read`](crate::Reg::read) this register and get [`zb_wakeup_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zb_wakeup_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zb_wakeup_src`] module"]
pub type ZB_WAKEUP_SRC = crate::Reg<zb_wakeup_src::ZB_WAKEUP_SRC_SPEC>;
#[doc = "wakeup source register for zb"]
pub mod zb_wakeup_src;
#[doc = "MODEMPWR_CTRL (w) register accessor: ctrl register for modempwr power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modempwr_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modempwr_ctrl`] module"]
pub type MODEMPWR_CTRL = crate::Reg<modempwr_ctrl::MODEMPWR_CTRL_SPEC>;
#[doc = "ctrl register for modempwr power control"]
pub mod modempwr_ctrl;
#[doc = "MODEMPWR_WAKEUP_SRC (rw) register accessor: wakeup source register for modempwr\n\nYou can [`read`](crate::Reg::read) this register and get [`modempwr_wakeup_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modempwr_wakeup_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modempwr_wakeup_src`] module"]
pub type MODEMPWR_WAKEUP_SRC = crate::Reg<modempwr_wakeup_src::MODEMPWR_WAKEUP_SRC_SPEC>;
#[doc = "wakeup source register for modempwr"]
pub mod modempwr_wakeup_src;
#[doc = "PERI_PAD_WAKE_SLP_EN (rw) register accessor: used for future potential eco, others don't care\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_pad_wake_slp_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_pad_wake_slp_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_pad_wake_slp_en`] module"]
pub type PERI_PAD_WAKE_SLP_EN = crate::Reg<peri_pad_wake_slp_en::PERI_PAD_WAKE_SLP_EN_SPEC>;
#[doc = "used for future potential eco, others don't care"]
pub mod peri_pad_wake_slp_en;
#[doc = "TOUCH_CTRL (w) register accessor: ctrl register for touch power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_ctrl`] module"]
pub type TOUCH_CTRL = crate::Reg<touch_ctrl::TOUCH_CTRL_SPEC>;
#[doc = "ctrl register for touch power control"]
pub mod touch_ctrl;
#[doc = "TOUCH_CFG (rw) register accessor: ctrl register for touch power control\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_cfg`] module"]
pub type TOUCH_CFG = crate::Reg<touch_cfg::TOUCH_CFG_SPEC>;
#[doc = "ctrl register for touch power control"]
pub mod touch_cfg;
#[doc = "TOUCH_STS (r) register accessor: status register for TOUCH PWR\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_sts`] module"]
pub type TOUCH_STS = crate::Reg<touch_sts::TOUCH_STS_SPEC>;
#[doc = "status register for TOUCH PWR"]
pub mod touch_sts;
#[doc = "CALI_CTRL (w) register accessor: ctrl register for cali power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cali_ctrl`] module"]
pub type CALI_CTRL = crate::Reg<cali_ctrl::CALI_CTRL_SPEC>;
#[doc = "ctrl register for cali power control"]
pub mod cali_ctrl;
#[doc = "CALI_STS (r) register accessor: status register for CALI PWR\n\nYou can [`read`](crate::Reg::read) this register and get [`cali_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cali_sts`] module"]
pub type CALI_STS = crate::Reg<cali_sts::CALI_STS_SPEC>;
#[doc = "status register for CALI PWR"]
pub mod cali_sts;
#[doc = "LPPERI_PWR_CLK_CFG (rw) register accessor: ctrl register for lpcpu power clock mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpperi_pwr_clk_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpperi_pwr_clk_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpperi_pwr_clk_cfg`] module"]
pub type LPPERI_PWR_CLK_CFG = crate::Reg<lpperi_pwr_clk_cfg::LPPERI_PWR_CLK_CFG_SPEC>;
#[doc = "ctrl register for lpcpu power clock mode control"]
pub mod lpperi_pwr_clk_cfg;
#[doc = "LPPERI_CTRL (rw) register accessor: ctrl register for lpperi power control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpperi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpperi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpperi_ctrl`] module"]
pub type LPPERI_CTRL = crate::Reg<lpperi_ctrl::LPPERI_CTRL_SPEC>;
#[doc = "ctrl register for lpperi power control"]
pub mod lpperi_ctrl;
#[doc = "LP_PAD_CFG (rw) register accessor: config register for lppad hold signal\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pad_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pad_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pad_cfg`] module"]
pub type LP_PAD_CFG = crate::Reg<lp_pad_cfg::LP_PAD_CFG_SPEC>;
#[doc = "config register for lppad hold signal"]
pub mod lp_pad_cfg;
#[doc = "CNNT_PAD_CFG (rw) register accessor: config register for cnntpad hold signal\n\nYou can [`read`](crate::Reg::read) this register and get [`cnnt_pad_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnnt_pad_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnnt_pad_cfg`] module"]
pub type CNNT_PAD_CFG = crate::Reg<cnnt_pad_cfg::CNNT_PAD_CFG_SPEC>;
#[doc = "config register for cnntpad hold signal"]
pub mod cnnt_pad_cfg;
#[doc = "HP_PAD_CFG (rw) register accessor: config register for hppad hold signal\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pad_cfg`] module"]
pub type HP_PAD_CFG = crate::Reg<hp_pad_cfg::HP_PAD_CFG_SPEC>;
#[doc = "config register for hppad hold signal"]
pub mod hp_pad_cfg;
#[doc = "INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "need_des"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod int_clr;
#[doc = "VER_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
#[doc = "need_des"]
pub mod ver_date;
