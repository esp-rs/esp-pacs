#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_ver_date: SYS_VER_DATE,
    clk_en: CLK_EN,
    _reserved2: [u8; 0x08],
    sys_sdmmc_mem_lp_ctrl: SYS_SDMMC_MEM_LP_CTRL,
    sys_usb_otghs_mem_lp_ctrl: SYS_USB_OTGHS_MEM_LP_CTRL,
    sys_usb_device_mem_lp_ctrl: SYS_USB_DEVICE_MEM_LP_CTRL,
    sys_gmac_mem_lp_ctrl: SYS_GMAC_MEM_LP_CTRL,
    sys_sprf_mem_aux_ctrl: SYS_SPRF_MEM_AUX_CTRL,
    sys_sdprf_mem_aux_ctrl: SYS_SDPRF_MEM_AUX_CTRL,
    sys_hp_pad_ulpi_ctrl: SYS_HP_PAD_ULPI_CTRL,
    sys_usb_clk_ctrl: SYS_USB_CLK_CTRL,
    sys_usb_otg20_ctrl: SYS_USB_OTG20_CTRL,
    sys_hp_usb_device_ctrl: SYS_HP_USB_DEVICE_CTRL,
    sys_hp_sdmmc_ctrl: SYS_HP_SDMMC_CTRL,
    sys_hp_emac_ctrl: SYS_HP_EMAC_CTRL,
    sys_hp_emac_ref_ctrl: SYS_HP_EMAC_REF_CTRL,
    sys_hp_emac_rmii_pad_ctrl: SYS_HP_EMAC_RMII_PAD_CTRL,
    sys_hp_emac_rmii_ctrl: SYS_HP_EMAC_RMII_CTRL,
    sys_hp_emac_rx_ctrl: SYS_HP_EMAC_RX_CTRL,
    sys_hp_emac_tx_ctrl: SYS_HP_EMAC_TX_CTRL,
    sys_hp_emac_ptp_ctrl: SYS_HP_EMAC_PTP_CTRL,
    sys_cnnt_iomux_ctrl0: SYS_CNNT_IOMUX_CTRL0,
    sys_spram_mem_aux_ctrl: SYS_SPRAM_MEM_AUX_CTRL,
    sys_gmac_ctrl0: SYS_GMAC_CTRL0,
    sys_gmac_ctrl1: SYS_GMAC_CTRL1,
    sys_gmac_ctrl2: SYS_GMAC_CTRL2,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn sys_ver_date(&self) -> &SYS_VER_DATE {
        &self.sys_ver_date
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn sys_sdmmc_mem_lp_ctrl(&self) -> &SYS_SDMMC_MEM_LP_CTRL {
        &self.sys_sdmmc_mem_lp_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sys_usb_otghs_mem_lp_ctrl(&self) -> &SYS_USB_OTGHS_MEM_LP_CTRL {
        &self.sys_usb_otghs_mem_lp_ctrl
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn sys_usb_device_mem_lp_ctrl(&self) -> &SYS_USB_DEVICE_MEM_LP_CTRL {
        &self.sys_usb_device_mem_lp_ctrl
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn sys_gmac_mem_lp_ctrl(&self) -> &SYS_GMAC_MEM_LP_CTRL {
        &self.sys_gmac_mem_lp_ctrl
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn sys_sprf_mem_aux_ctrl(&self) -> &SYS_SPRF_MEM_AUX_CTRL {
        &self.sys_sprf_mem_aux_ctrl
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn sys_sdprf_mem_aux_ctrl(&self) -> &SYS_SDPRF_MEM_AUX_CTRL {
        &self.sys_sdprf_mem_aux_ctrl
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn sys_hp_pad_ulpi_ctrl(&self) -> &SYS_HP_PAD_ULPI_CTRL {
        &self.sys_hp_pad_ulpi_ctrl
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn sys_usb_clk_ctrl(&self) -> &SYS_USB_CLK_CTRL {
        &self.sys_usb_clk_ctrl
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn sys_usb_otg20_ctrl(&self) -> &SYS_USB_OTG20_CTRL {
        &self.sys_usb_otg20_ctrl
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn sys_hp_usb_device_ctrl(&self) -> &SYS_HP_USB_DEVICE_CTRL {
        &self.sys_hp_usb_device_ctrl
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn sys_hp_sdmmc_ctrl(&self) -> &SYS_HP_SDMMC_CTRL {
        &self.sys_hp_sdmmc_ctrl
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn sys_hp_emac_ctrl(&self) -> &SYS_HP_EMAC_CTRL {
        &self.sys_hp_emac_ctrl
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn sys_hp_emac_ref_ctrl(&self) -> &SYS_HP_EMAC_REF_CTRL {
        &self.sys_hp_emac_ref_ctrl
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn sys_hp_emac_rmii_pad_ctrl(&self) -> &SYS_HP_EMAC_RMII_PAD_CTRL {
        &self.sys_hp_emac_rmii_pad_ctrl
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn sys_hp_emac_rmii_ctrl(&self) -> &SYS_HP_EMAC_RMII_CTRL {
        &self.sys_hp_emac_rmii_ctrl
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn sys_hp_emac_rx_ctrl(&self) -> &SYS_HP_EMAC_RX_CTRL {
        &self.sys_hp_emac_rx_ctrl
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn sys_hp_emac_tx_ctrl(&self) -> &SYS_HP_EMAC_TX_CTRL {
        &self.sys_hp_emac_tx_ctrl
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn sys_hp_emac_ptp_ctrl(&self) -> &SYS_HP_EMAC_PTP_CTRL {
        &self.sys_hp_emac_ptp_ctrl
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn sys_cnnt_iomux_ctrl0(&self) -> &SYS_CNNT_IOMUX_CTRL0 {
        &self.sys_cnnt_iomux_ctrl0
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn sys_spram_mem_aux_ctrl(&self) -> &SYS_SPRAM_MEM_AUX_CTRL {
        &self.sys_spram_mem_aux_ctrl
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn sys_gmac_ctrl0(&self) -> &SYS_GMAC_CTRL0 {
        &self.sys_gmac_ctrl0
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn sys_gmac_ctrl1(&self) -> &SYS_GMAC_CTRL1 {
        &self.sys_gmac_ctrl1
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn sys_gmac_ctrl2(&self) -> &SYS_GMAC_CTRL2 {
        &self.sys_gmac_ctrl2
    }
}
#[doc = "SYS_VER_DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ver_date`] module"]
pub type SYS_VER_DATE = crate::Reg<sys_ver_date::SYS_VER_DATE_SPEC>;
#[doc = ""]
pub mod sys_ver_date;
#[doc = "CLK_EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = ""]
pub mod clk_en;
#[doc = "SYS_SDMMC_MEM_LP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sdmmc_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sdmmc_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sdmmc_mem_lp_ctrl`] module"]
pub type SYS_SDMMC_MEM_LP_CTRL = crate::Reg<sys_sdmmc_mem_lp_ctrl::SYS_SDMMC_MEM_LP_CTRL_SPEC>;
#[doc = ""]
pub mod sys_sdmmc_mem_lp_ctrl;
#[doc = "SYS_USB_OTGHS_MEM_LP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_usb_otghs_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_usb_otghs_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_usb_otghs_mem_lp_ctrl`] module"]
pub type SYS_USB_OTGHS_MEM_LP_CTRL =
    crate::Reg<sys_usb_otghs_mem_lp_ctrl::SYS_USB_OTGHS_MEM_LP_CTRL_SPEC>;
#[doc = ""]
pub mod sys_usb_otghs_mem_lp_ctrl;
#[doc = "SYS_USB_DEVICE_MEM_LP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_usb_device_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_usb_device_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_usb_device_mem_lp_ctrl`] module"]
pub type SYS_USB_DEVICE_MEM_LP_CTRL =
    crate::Reg<sys_usb_device_mem_lp_ctrl::SYS_USB_DEVICE_MEM_LP_CTRL_SPEC>;
#[doc = ""]
pub mod sys_usb_device_mem_lp_ctrl;
#[doc = "SYS_GMAC_MEM_LP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_gmac_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_gmac_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_gmac_mem_lp_ctrl`] module"]
pub type SYS_GMAC_MEM_LP_CTRL = crate::Reg<sys_gmac_mem_lp_ctrl::SYS_GMAC_MEM_LP_CTRL_SPEC>;
#[doc = ""]
pub mod sys_gmac_mem_lp_ctrl;
#[doc = "SYS_SPRF_MEM_AUX_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sprf_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sprf_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sprf_mem_aux_ctrl`] module"]
pub type SYS_SPRF_MEM_AUX_CTRL = crate::Reg<sys_sprf_mem_aux_ctrl::SYS_SPRF_MEM_AUX_CTRL_SPEC>;
#[doc = ""]
pub mod sys_sprf_mem_aux_ctrl;
#[doc = "SYS_SDPRF_MEM_AUX_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sdprf_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sdprf_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sdprf_mem_aux_ctrl`] module"]
pub type SYS_SDPRF_MEM_AUX_CTRL = crate::Reg<sys_sdprf_mem_aux_ctrl::SYS_SDPRF_MEM_AUX_CTRL_SPEC>;
#[doc = ""]
pub mod sys_sdprf_mem_aux_ctrl;
#[doc = "SYS_HP_PAD_ULPI_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_pad_ulpi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_pad_ulpi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_pad_ulpi_ctrl`] module"]
pub type SYS_HP_PAD_ULPI_CTRL = crate::Reg<sys_hp_pad_ulpi_ctrl::SYS_HP_PAD_ULPI_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_pad_ulpi_ctrl;
#[doc = "SYS_USB_CLK_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_usb_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_usb_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_usb_clk_ctrl`] module"]
pub type SYS_USB_CLK_CTRL = crate::Reg<sys_usb_clk_ctrl::SYS_USB_CLK_CTRL_SPEC>;
#[doc = ""]
pub mod sys_usb_clk_ctrl;
#[doc = "SYS_USB_OTG20_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_usb_otg20_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_usb_otg20_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_usb_otg20_ctrl`] module"]
pub type SYS_USB_OTG20_CTRL = crate::Reg<sys_usb_otg20_ctrl::SYS_USB_OTG20_CTRL_SPEC>;
#[doc = ""]
pub mod sys_usb_otg20_ctrl;
#[doc = "SYS_HP_USB_DEVICE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_usb_device_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_usb_device_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_usb_device_ctrl`] module"]
pub type SYS_HP_USB_DEVICE_CTRL = crate::Reg<sys_hp_usb_device_ctrl::SYS_HP_USB_DEVICE_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_usb_device_ctrl;
#[doc = "SYS_HP_SDMMC_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_sdmmc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_sdmmc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_sdmmc_ctrl`] module"]
pub type SYS_HP_SDMMC_CTRL = crate::Reg<sys_hp_sdmmc_ctrl::SYS_HP_SDMMC_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_sdmmc_ctrl;
#[doc = "SYS_HP_EMAC_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_emac_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_emac_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_emac_ctrl`] module"]
pub type SYS_HP_EMAC_CTRL = crate::Reg<sys_hp_emac_ctrl::SYS_HP_EMAC_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_emac_ctrl;
#[doc = "SYS_HP_EMAC_REF_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_emac_ref_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_emac_ref_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_emac_ref_ctrl`] module"]
pub type SYS_HP_EMAC_REF_CTRL = crate::Reg<sys_hp_emac_ref_ctrl::SYS_HP_EMAC_REF_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_emac_ref_ctrl;
#[doc = "SYS_HP_EMAC_RMII_PAD_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_emac_rmii_pad_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_emac_rmii_pad_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_emac_rmii_pad_ctrl`] module"]
pub type SYS_HP_EMAC_RMII_PAD_CTRL =
    crate::Reg<sys_hp_emac_rmii_pad_ctrl::SYS_HP_EMAC_RMII_PAD_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_emac_rmii_pad_ctrl;
#[doc = "SYS_HP_EMAC_RMII_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_emac_rmii_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_emac_rmii_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_emac_rmii_ctrl`] module"]
pub type SYS_HP_EMAC_RMII_CTRL = crate::Reg<sys_hp_emac_rmii_ctrl::SYS_HP_EMAC_RMII_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_emac_rmii_ctrl;
#[doc = "SYS_HP_EMAC_RX_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_emac_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_emac_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_emac_rx_ctrl`] module"]
pub type SYS_HP_EMAC_RX_CTRL = crate::Reg<sys_hp_emac_rx_ctrl::SYS_HP_EMAC_RX_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_emac_rx_ctrl;
#[doc = "SYS_HP_EMAC_TX_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_emac_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_emac_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_emac_tx_ctrl`] module"]
pub type SYS_HP_EMAC_TX_CTRL = crate::Reg<sys_hp_emac_tx_ctrl::SYS_HP_EMAC_TX_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_emac_tx_ctrl;
#[doc = "SYS_HP_EMAC_PTP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_emac_ptp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_emac_ptp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_hp_emac_ptp_ctrl`] module"]
pub type SYS_HP_EMAC_PTP_CTRL = crate::Reg<sys_hp_emac_ptp_ctrl::SYS_HP_EMAC_PTP_CTRL_SPEC>;
#[doc = ""]
pub mod sys_hp_emac_ptp_ctrl;
#[doc = "SYS_CNNT_IOMUX_CTRL0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_cnnt_iomux_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_cnnt_iomux_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_cnnt_iomux_ctrl0`] module"]
pub type SYS_CNNT_IOMUX_CTRL0 = crate::Reg<sys_cnnt_iomux_ctrl0::SYS_CNNT_IOMUX_CTRL0_SPEC>;
#[doc = ""]
pub mod sys_cnnt_iomux_ctrl0;
#[doc = "SYS_SPRAM_MEM_AUX_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_spram_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_spram_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_spram_mem_aux_ctrl`] module"]
pub type SYS_SPRAM_MEM_AUX_CTRL = crate::Reg<sys_spram_mem_aux_ctrl::SYS_SPRAM_MEM_AUX_CTRL_SPEC>;
#[doc = ""]
pub mod sys_spram_mem_aux_ctrl;
#[doc = "SYS_GMAC_CTRL0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_gmac_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_gmac_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_gmac_ctrl0`] module"]
pub type SYS_GMAC_CTRL0 = crate::Reg<sys_gmac_ctrl0::SYS_GMAC_CTRL0_SPEC>;
#[doc = ""]
pub mod sys_gmac_ctrl0;
#[doc = "SYS_GMAC_CTRL1 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_gmac_ctrl1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_gmac_ctrl1`] module"]
pub type SYS_GMAC_CTRL1 = crate::Reg<sys_gmac_ctrl1::SYS_GMAC_CTRL1_SPEC>;
#[doc = ""]
pub mod sys_gmac_ctrl1;
#[doc = "SYS_GMAC_CTRL2 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_gmac_ctrl2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_gmac_ctrl2`] module"]
pub type SYS_GMAC_CTRL2 = crate::Reg<sys_gmac_ctrl2::SYS_GMAC_CTRL2_SPEC>;
#[doc = ""]
pub mod sys_gmac_ctrl2;
