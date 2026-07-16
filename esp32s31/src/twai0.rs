#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    device_id_version: DEVICE_ID_VERSION,
    mode_settings: MODE_SETTINGS,
    status: STATUS,
    command: COMMAND,
    int_stat: INT_STAT,
    int_ena_set: INT_ENA_SET,
    int_ena_clr: INT_ENA_CLR,
    int_mask_set: INT_MASK_SET,
    int_mask_clr: INT_MASK_CLR,
    btr: BTR,
    btr_fd: BTR_FD,
    ewl_erp_fault_state: EWL_ERP_FAULT_STATE,
    rec_tec: REC_TEC,
    err_norm_err_fd: ERR_NORM_ERR_FD,
    ctr_pres: CTR_PRES,
    filter_a_mask: FILTER_A_MASK,
    filter_a_val: FILTER_A_VAL,
    filter_b_mask: FILTER_B_MASK,
    filter_b_val: FILTER_B_VAL,
    filter_c_mask: FILTER_C_MASK,
    filter_c_val: FILTER_C_VAL,
    filter_ran_low: FILTER_RAN_LOW,
    filter_ran_high: FILTER_RAN_HIGH,
    filter_control_filter_status: FILTER_CONTROL_FILTER_STATUS,
    rx_mem_info: RX_MEM_INFO,
    rx_pointers: RX_POINTERS,
    rx_status_rx_settings: RX_STATUS_RX_SETTINGS,
    rx_data: RX_DATA,
    tx_status: TX_STATUS,
    tx_command_txtb_info: TX_COMMAND_TXTB_INFO,
    tx_priority: TX_PRIORITY,
    err_capt_retr_ctr_alc_ts_info: ERR_CAPT_RETR_CTR_ALC_TS_INFO,
    trv_delay_ssp_cfg: TRV_DELAY_SSP_CFG,
    rx_fr_ctr: RX_FR_CTR,
    tx_fr_ctr: TX_FR_CTR,
    debug: DEBUG,
    yolo: YOLO,
    timestamp_low: TIMESTAMP_LOW,
    timestamp_high: TIMESTAMP_HIGH,
    _reserved39: [u8; 0x0f38],
    timer_clk_en: TIMER_CLK_EN,
    timer_int_raw: TIMER_INT_RAW,
    timer_int_st: TIMER_INT_ST,
    timer_int_ena: TIMER_INT_ENA,
    timer_int_clr: TIMER_INT_CLR,
    timer_cfg: TIMER_CFG,
    timer_ld_val_l: TIMER_LD_VAL_L,
    timer_ld_val_h: TIMER_LD_VAL_H,
    timer_ct_val_l: TIMER_CT_VAL_L,
    timer_ct_val_h: TIMER_CT_VAL_H,
    date_ver: DATE_VER,
}
impl RegisterBlock {
    #[doc = "0x00 - TWAI FD device id status register"]
    #[inline(always)]
    pub const fn device_id_version(&self) -> &DEVICE_ID_VERSION {
        &self.device_id_version
    }
    #[doc = "0x04 - TWAI FD mode setting register"]
    #[inline(always)]
    pub const fn mode_settings(&self) -> &MODE_SETTINGS {
        &self.mode_settings
    }
    #[doc = "0x08 - TWAI FD status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x0c - TWAI FD command register"]
    #[inline(always)]
    pub const fn command(&self) -> &COMMAND {
        &self.command
    }
    #[doc = "0x10 - TWAI FD command register"]
    #[inline(always)]
    pub const fn int_stat(&self) -> &INT_STAT {
        &self.int_stat
    }
    #[doc = "0x14 - TWAI FD interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena_set(&self) -> &INT_ENA_SET {
        &self.int_ena_set
    }
    #[doc = "0x18 - TWAI FD interrupt enable clear register"]
    #[inline(always)]
    pub const fn int_ena_clr(&self) -> &INT_ENA_CLR {
        &self.int_ena_clr
    }
    #[doc = "0x1c - TWAI FD interrupt mask register"]
    #[inline(always)]
    pub const fn int_mask_set(&self) -> &INT_MASK_SET {
        &self.int_mask_set
    }
    #[doc = "0x20 - TWAI FD interrupt mask clear register"]
    #[inline(always)]
    pub const fn int_mask_clr(&self) -> &INT_MASK_CLR {
        &self.int_mask_clr
    }
    #[doc = "0x24 - TWAI FD bit-timing register"]
    #[inline(always)]
    pub const fn btr(&self) -> &BTR {
        &self.btr
    }
    #[doc = "0x28 - TWAI FD bit-timing of FD register"]
    #[inline(always)]
    pub const fn btr_fd(&self) -> &BTR_FD {
        &self.btr_fd
    }
    #[doc = "0x2c - TWAI FD error threshold and status register"]
    #[inline(always)]
    pub const fn ewl_erp_fault_state(&self) -> &EWL_ERP_FAULT_STATE {
        &self.ewl_erp_fault_state
    }
    #[doc = "0x30 - TWAI FD error counters status register"]
    #[inline(always)]
    pub const fn rec_tec(&self) -> &REC_TEC {
        &self.rec_tec
    }
    #[doc = "0x34 - TWAI FD special error counters status register"]
    #[inline(always)]
    pub const fn err_norm_err_fd(&self) -> &ERR_NORM_ERR_FD {
        &self.err_norm_err_fd
    }
    #[doc = "0x38 - TWAI FD error counters pre-define configuration register"]
    #[inline(always)]
    pub const fn ctr_pres(&self) -> &CTR_PRES {
        &self.ctr_pres
    }
    #[doc = "0x3c - TWAI FD filter A mask value register"]
    #[inline(always)]
    pub const fn filter_a_mask(&self) -> &FILTER_A_MASK {
        &self.filter_a_mask
    }
    #[doc = "0x40 - TWAI FD filter A bit value register"]
    #[inline(always)]
    pub const fn filter_a_val(&self) -> &FILTER_A_VAL {
        &self.filter_a_val
    }
    #[doc = "0x44 - TWAI FD filter B mask value register"]
    #[inline(always)]
    pub const fn filter_b_mask(&self) -> &FILTER_B_MASK {
        &self.filter_b_mask
    }
    #[doc = "0x48 - TWAI FD filter B bit value register"]
    #[inline(always)]
    pub const fn filter_b_val(&self) -> &FILTER_B_VAL {
        &self.filter_b_val
    }
    #[doc = "0x4c - TWAI FD filter C mask value register"]
    #[inline(always)]
    pub const fn filter_c_mask(&self) -> &FILTER_C_MASK {
        &self.filter_c_mask
    }
    #[doc = "0x50 - TWAI FD filter C bit value register"]
    #[inline(always)]
    pub const fn filter_c_val(&self) -> &FILTER_C_VAL {
        &self.filter_c_val
    }
    #[doc = "0x54 - TWAI FD filter range low value register"]
    #[inline(always)]
    pub const fn filter_ran_low(&self) -> &FILTER_RAN_LOW {
        &self.filter_ran_low
    }
    #[doc = "0x58 - TWAI FD filter range high value register"]
    #[inline(always)]
    pub const fn filter_ran_high(&self) -> &FILTER_RAN_HIGH {
        &self.filter_ran_high
    }
    #[doc = "0x5c - TWAI FD filter control register"]
    #[inline(always)]
    pub const fn filter_control_filter_status(&self) -> &FILTER_CONTROL_FILTER_STATUS {
        &self.filter_control_filter_status
    }
    #[doc = "0x60 - TWAI FD rx memory information register"]
    #[inline(always)]
    pub const fn rx_mem_info(&self) -> &RX_MEM_INFO {
        &self.rx_mem_info
    }
    #[doc = "0x64 - TWAI FD rx memory pointer information register"]
    #[inline(always)]
    pub const fn rx_pointers(&self) -> &RX_POINTERS {
        &self.rx_pointers
    }
    #[doc = "0x68 - TWAI FD rx status & setting register"]
    #[inline(always)]
    pub const fn rx_status_rx_settings(&self) -> &RX_STATUS_RX_SETTINGS {
        &self.rx_status_rx_settings
    }
    #[doc = "0x6c - TWAI FD received data register"]
    #[inline(always)]
    pub const fn rx_data(&self) -> &RX_DATA {
        &self.rx_data
    }
    #[doc = "0x70 - TWAI FD TX buffer status register"]
    #[inline(always)]
    pub const fn tx_status(&self) -> &TX_STATUS {
        &self.tx_status
    }
    #[doc = "0x74 - TWAI FD TXT buffer command & information register"]
    #[inline(always)]
    pub const fn tx_command_txtb_info(&self) -> &TX_COMMAND_TXTB_INFO {
        &self.tx_command_txtb_info
    }
    #[doc = "0x78 - TWAI FD TXT buffer command & information register"]
    #[inline(always)]
    pub const fn tx_priority(&self) -> &TX_PRIORITY {
        &self.tx_priority
    }
    #[doc = "0x7c - TWAI FD error capture & retransmit counter & arbitration lost & timestamp integration information register"]
    #[inline(always)]
    pub const fn err_capt_retr_ctr_alc_ts_info(&self) -> &ERR_CAPT_RETR_CTR_ALC_TS_INFO {
        &self.err_capt_retr_ctr_alc_ts_info
    }
    #[doc = "0x80 - TWAI FD transmit delay & secondary sample point configuration register"]
    #[inline(always)]
    pub const fn trv_delay_ssp_cfg(&self) -> &TRV_DELAY_SSP_CFG {
        &self.trv_delay_ssp_cfg
    }
    #[doc = "0x84 - TWAI FD received frame counter register"]
    #[inline(always)]
    pub const fn rx_fr_ctr(&self) -> &RX_FR_CTR {
        &self.rx_fr_ctr
    }
    #[doc = "0x88 - TWAI FD transmitted frame counter register"]
    #[inline(always)]
    pub const fn tx_fr_ctr(&self) -> &TX_FR_CTR {
        &self.tx_fr_ctr
    }
    #[doc = "0x8c - TWAI FD debug register"]
    #[inline(always)]
    pub const fn debug(&self) -> &DEBUG {
        &self.debug
    }
    #[doc = "0x90 - TWAI FD transmitted frame counter register"]
    #[inline(always)]
    pub const fn yolo(&self) -> &YOLO {
        &self.yolo
    }
    #[doc = "0x94 - TWAI FD transmitted frame counter register"]
    #[inline(always)]
    pub const fn timestamp_low(&self) -> &TIMESTAMP_LOW {
        &self.timestamp_low
    }
    #[doc = "0x98 - TWAI FD transmitted frame counter register"]
    #[inline(always)]
    pub const fn timestamp_high(&self) -> &TIMESTAMP_HIGH {
        &self.timestamp_high
    }
    #[doc = "0xfd4 - TWAIFD timer clock force enable register."]
    #[inline(always)]
    pub const fn timer_clk_en(&self) -> &TIMER_CLK_EN {
        &self.timer_clk_en
    }
    #[doc = "0xfd8 - TWAIFD raw interrupt register."]
    #[inline(always)]
    pub const fn timer_int_raw(&self) -> &TIMER_INT_RAW {
        &self.timer_int_raw
    }
    #[doc = "0xfdc - TWAIFD interrupt status register."]
    #[inline(always)]
    pub const fn timer_int_st(&self) -> &TIMER_INT_ST {
        &self.timer_int_st
    }
    #[doc = "0xfe0 - TWAIFD interrupt enable register."]
    #[inline(always)]
    pub const fn timer_int_ena(&self) -> &TIMER_INT_ENA {
        &self.timer_int_ena
    }
    #[doc = "0xfe4 - TWAIFD interrupt clear register."]
    #[inline(always)]
    pub const fn timer_int_clr(&self) -> &TIMER_INT_CLR {
        &self.timer_int_clr
    }
    #[doc = "0xfe8 - TWAI FD timer configure register."]
    #[inline(always)]
    pub const fn timer_cfg(&self) -> &TIMER_CFG {
        &self.timer_cfg
    }
    #[doc = "0xfec - TWAI FD timer pre-load value low register."]
    #[inline(always)]
    pub const fn timer_ld_val_l(&self) -> &TIMER_LD_VAL_L {
        &self.timer_ld_val_l
    }
    #[doc = "0xff0 - TWAI FD timer pre-load value high register."]
    #[inline(always)]
    pub const fn timer_ld_val_h(&self) -> &TIMER_LD_VAL_H {
        &self.timer_ld_val_h
    }
    #[doc = "0xff4 - TWAI FD timer count-to value low register."]
    #[inline(always)]
    pub const fn timer_ct_val_l(&self) -> &TIMER_CT_VAL_L {
        &self.timer_ct_val_l
    }
    #[doc = "0xff8 - TWAI FD timer count-to value high register."]
    #[inline(always)]
    pub const fn timer_ct_val_h(&self) -> &TIMER_CT_VAL_H {
        &self.timer_ct_val_h
    }
    #[doc = "0xffc - TWAI FD date version"]
    #[inline(always)]
    pub const fn date_ver(&self) -> &DATE_VER {
        &self.date_ver
    }
}
#[doc = "DEVICE_ID_VERSION (r) register accessor: TWAI FD device id status register\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_id_version`] module"]
pub type DEVICE_ID_VERSION = crate::Reg<device_id_version::DEVICE_ID_VERSION_SPEC>;
#[doc = "TWAI FD device id status register"]
pub mod device_id_version;
#[doc = "MODE_SETTINGS (rw) register accessor: TWAI FD mode setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode_settings::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode_settings::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_settings`] module"]
pub type MODE_SETTINGS = crate::Reg<mode_settings::MODE_SETTINGS_SPEC>;
#[doc = "TWAI FD mode setting register"]
pub mod mode_settings;
#[doc = "STATUS (r) register accessor: TWAI FD status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "TWAI FD status register"]
pub mod status;
#[doc = "COMMAND (w) register accessor: TWAI FD command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "TWAI FD command register"]
pub mod command;
#[doc = "INT_STAT (rw) register accessor: TWAI FD command register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_stat`] module"]
pub type INT_STAT = crate::Reg<int_stat::INT_STAT_SPEC>;
#[doc = "TWAI FD command register"]
pub mod int_stat;
#[doc = "INT_ENA_SET (rw) register accessor: TWAI FD interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_set`] module"]
pub type INT_ENA_SET = crate::Reg<int_ena_set::INT_ENA_SET_SPEC>;
#[doc = "TWAI FD interrupt enable register"]
pub mod int_ena_set;
#[doc = "INT_ENA_CLR (w) register accessor: TWAI FD interrupt enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_clr`] module"]
pub type INT_ENA_CLR = crate::Reg<int_ena_clr::INT_ENA_CLR_SPEC>;
#[doc = "TWAI FD interrupt enable clear register"]
pub mod int_ena_clr;
#[doc = "INT_MASK_SET (rw) register accessor: TWAI FD interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask_set`] module"]
pub type INT_MASK_SET = crate::Reg<int_mask_set::INT_MASK_SET_SPEC>;
#[doc = "TWAI FD interrupt mask register"]
pub mod int_mask_set;
#[doc = "INT_MASK_CLR (w) register accessor: TWAI FD interrupt mask clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask_clr`] module"]
pub type INT_MASK_CLR = crate::Reg<int_mask_clr::INT_MASK_CLR_SPEC>;
#[doc = "TWAI FD interrupt mask clear register"]
pub mod int_mask_clr;
#[doc = "BTR (rw) register accessor: TWAI FD bit-timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`btr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr`] module"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "TWAI FD bit-timing register"]
pub mod btr;
#[doc = "BTR_FD (rw) register accessor: TWAI FD bit-timing of FD register\n\nYou can [`read`](crate::Reg::read) this register and get [`btr_fd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr_fd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr_fd`] module"]
pub type BTR_FD = crate::Reg<btr_fd::BTR_FD_SPEC>;
#[doc = "TWAI FD bit-timing of FD register"]
pub mod btr_fd;
#[doc = "EWL_ERP_FAULT_STATE (rw) register accessor: TWAI FD error threshold and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewl_erp_fault_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewl_erp_fault_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewl_erp_fault_state`] module"]
pub type EWL_ERP_FAULT_STATE = crate::Reg<ewl_erp_fault_state::EWL_ERP_FAULT_STATE_SPEC>;
#[doc = "TWAI FD error threshold and status register"]
pub mod ewl_erp_fault_state;
#[doc = "REC_TEC (r) register accessor: TWAI FD error counters status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rec_tec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rec_tec`] module"]
pub type REC_TEC = crate::Reg<rec_tec::REC_TEC_SPEC>;
#[doc = "TWAI FD error counters status register"]
pub mod rec_tec;
#[doc = "ERR_NORM_ERR_FD (r) register accessor: TWAI FD special error counters status register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_norm_err_fd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_norm_err_fd`] module"]
pub type ERR_NORM_ERR_FD = crate::Reg<err_norm_err_fd::ERR_NORM_ERR_FD_SPEC>;
#[doc = "TWAI FD special error counters status register"]
pub mod err_norm_err_fd;
#[doc = "CTR_PRES (w) register accessor: TWAI FD error counters pre-define configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr_pres::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr_pres`] module"]
pub type CTR_PRES = crate::Reg<ctr_pres::CTR_PRES_SPEC>;
#[doc = "TWAI FD error counters pre-define configuration register"]
pub mod ctr_pres;
#[doc = "FILTER_A_MASK (rw) register accessor: TWAI FD filter A mask value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_a_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_a_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_a_mask`] module"]
pub type FILTER_A_MASK = crate::Reg<filter_a_mask::FILTER_A_MASK_SPEC>;
#[doc = "TWAI FD filter A mask value register"]
pub mod filter_a_mask;
#[doc = "FILTER_A_VAL (rw) register accessor: TWAI FD filter A bit value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_a_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_a_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_a_val`] module"]
pub type FILTER_A_VAL = crate::Reg<filter_a_val::FILTER_A_VAL_SPEC>;
#[doc = "TWAI FD filter A bit value register"]
pub mod filter_a_val;
#[doc = "FILTER_B_MASK (rw) register accessor: TWAI FD filter B mask value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_b_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_b_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_b_mask`] module"]
pub type FILTER_B_MASK = crate::Reg<filter_b_mask::FILTER_B_MASK_SPEC>;
#[doc = "TWAI FD filter B mask value register"]
pub mod filter_b_mask;
#[doc = "FILTER_B_VAL (rw) register accessor: TWAI FD filter B bit value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_b_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_b_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_b_val`] module"]
pub type FILTER_B_VAL = crate::Reg<filter_b_val::FILTER_B_VAL_SPEC>;
#[doc = "TWAI FD filter B bit value register"]
pub mod filter_b_val;
#[doc = "FILTER_C_MASK (rw) register accessor: TWAI FD filter C mask value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_c_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_c_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_c_mask`] module"]
pub type FILTER_C_MASK = crate::Reg<filter_c_mask::FILTER_C_MASK_SPEC>;
#[doc = "TWAI FD filter C mask value register"]
pub mod filter_c_mask;
#[doc = "FILTER_C_VAL (rw) register accessor: TWAI FD filter C bit value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_c_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_c_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_c_val`] module"]
pub type FILTER_C_VAL = crate::Reg<filter_c_val::FILTER_C_VAL_SPEC>;
#[doc = "TWAI FD filter C bit value register"]
pub mod filter_c_val;
#[doc = "FILTER_RAN_LOW (rw) register accessor: TWAI FD filter range low value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ran_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ran_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ran_low`] module"]
pub type FILTER_RAN_LOW = crate::Reg<filter_ran_low::FILTER_RAN_LOW_SPEC>;
#[doc = "TWAI FD filter range low value register"]
pub mod filter_ran_low;
#[doc = "FILTER_RAN_HIGH (rw) register accessor: TWAI FD filter range high value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ran_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ran_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ran_high`] module"]
pub type FILTER_RAN_HIGH = crate::Reg<filter_ran_high::FILTER_RAN_HIGH_SPEC>;
#[doc = "TWAI FD filter range high value register"]
pub mod filter_ran_high;
#[doc = "FILTER_CONTROL_FILTER_STATUS (rw) register accessor: TWAI FD filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_control_filter_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_control_filter_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_control_filter_status`] module"]
pub type FILTER_CONTROL_FILTER_STATUS =
    crate::Reg<filter_control_filter_status::FILTER_CONTROL_FILTER_STATUS_SPEC>;
#[doc = "TWAI FD filter control register"]
pub mod filter_control_filter_status;
#[doc = "RX_MEM_INFO (r) register accessor: TWAI FD rx memory information register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_mem_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_mem_info`] module"]
pub type RX_MEM_INFO = crate::Reg<rx_mem_info::RX_MEM_INFO_SPEC>;
#[doc = "TWAI FD rx memory information register"]
pub mod rx_mem_info;
#[doc = "RX_POINTERS (r) register accessor: TWAI FD rx memory pointer information register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_pointers::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_pointers`] module"]
pub type RX_POINTERS = crate::Reg<rx_pointers::RX_POINTERS_SPEC>;
#[doc = "TWAI FD rx memory pointer information register"]
pub mod rx_pointers;
#[doc = "RX_STATUS_RX_SETTINGS (rw) register accessor: TWAI FD rx status & setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_status_rx_settings::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_status_rx_settings::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_status_rx_settings`] module"]
pub type RX_STATUS_RX_SETTINGS = crate::Reg<rx_status_rx_settings::RX_STATUS_RX_SETTINGS_SPEC>;
#[doc = "TWAI FD rx status & setting register"]
pub mod rx_status_rx_settings;
#[doc = "RX_DATA (r) register accessor: TWAI FD received data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data`] module"]
pub type RX_DATA = crate::Reg<rx_data::RX_DATA_SPEC>;
#[doc = "TWAI FD received data register"]
pub mod rx_data;
#[doc = "TX_STATUS (r) register accessor: TWAI FD TX buffer status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_status`] module"]
pub type TX_STATUS = crate::Reg<tx_status::TX_STATUS_SPEC>;
#[doc = "TWAI FD TX buffer status register"]
pub mod tx_status;
#[doc = "TX_COMMAND_TXTB_INFO (rw) register accessor: TWAI FD TXT buffer command & information register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_command_txtb_info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_command_txtb_info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_command_txtb_info`] module"]
pub type TX_COMMAND_TXTB_INFO = crate::Reg<tx_command_txtb_info::TX_COMMAND_TXTB_INFO_SPEC>;
#[doc = "TWAI FD TXT buffer command & information register"]
pub mod tx_command_txtb_info;
#[doc = "TX_PRIORITY (rw) register accessor: TWAI FD TXT buffer command & information register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_priority`] module"]
pub type TX_PRIORITY = crate::Reg<tx_priority::TX_PRIORITY_SPEC>;
#[doc = "TWAI FD TXT buffer command & information register"]
pub mod tx_priority;
#[doc = "ERR_CAPT_RETR_CTR_ALC_TS_INFO (r) register accessor: TWAI FD error capture & retransmit counter & arbitration lost & timestamp integration information register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_capt_retr_ctr_alc_ts_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_capt_retr_ctr_alc_ts_info`] module"]
pub type ERR_CAPT_RETR_CTR_ALC_TS_INFO =
    crate::Reg<err_capt_retr_ctr_alc_ts_info::ERR_CAPT_RETR_CTR_ALC_TS_INFO_SPEC>;
#[doc = "TWAI FD error capture & retransmit counter & arbitration lost & timestamp integration information register"]
pub mod err_capt_retr_ctr_alc_ts_info;
#[doc = "TRV_DELAY_SSP_CFG (rw) register accessor: TWAI FD transmit delay & secondary sample point configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`trv_delay_ssp_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trv_delay_ssp_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trv_delay_ssp_cfg`] module"]
pub type TRV_DELAY_SSP_CFG = crate::Reg<trv_delay_ssp_cfg::TRV_DELAY_SSP_CFG_SPEC>;
#[doc = "TWAI FD transmit delay & secondary sample point configuration register"]
pub mod trv_delay_ssp_cfg;
#[doc = "RX_FR_CTR (r) register accessor: TWAI FD received frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fr_ctr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fr_ctr`] module"]
pub type RX_FR_CTR = crate::Reg<rx_fr_ctr::RX_FR_CTR_SPEC>;
#[doc = "TWAI FD received frame counter register"]
pub mod rx_fr_ctr;
#[doc = "TX_FR_CTR (r) register accessor: TWAI FD transmitted frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fr_ctr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fr_ctr`] module"]
pub type TX_FR_CTR = crate::Reg<tx_fr_ctr::TX_FR_CTR_SPEC>;
#[doc = "TWAI FD transmitted frame counter register"]
pub mod tx_fr_ctr;
#[doc = "DEBUG (r) register accessor: TWAI FD debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`] module"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "TWAI FD debug register"]
pub mod debug;
#[doc = "YOLO (r) register accessor: TWAI FD transmitted frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`yolo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@yolo`] module"]
pub type YOLO = crate::Reg<yolo::YOLO_SPEC>;
#[doc = "TWAI FD transmitted frame counter register"]
pub mod yolo;
#[doc = "TIMESTAMP_LOW (r) register accessor: TWAI FD transmitted frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_low::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_low`] module"]
pub type TIMESTAMP_LOW = crate::Reg<timestamp_low::TIMESTAMP_LOW_SPEC>;
#[doc = "TWAI FD transmitted frame counter register"]
pub mod timestamp_low;
#[doc = "TIMESTAMP_HIGH (r) register accessor: TWAI FD transmitted frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_high::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_high`] module"]
pub type TIMESTAMP_HIGH = crate::Reg<timestamp_high::TIMESTAMP_HIGH_SPEC>;
#[doc = "TWAI FD transmitted frame counter register"]
pub mod timestamp_high;
#[doc = "TIMER_CLK_EN (rw) register accessor: TWAIFD timer clock force enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_clk_en`] module"]
pub type TIMER_CLK_EN = crate::Reg<timer_clk_en::TIMER_CLK_EN_SPEC>;
#[doc = "TWAIFD timer clock force enable register."]
pub mod timer_clk_en;
#[doc = "TIMER_INT_RAW (r) register accessor: TWAIFD raw interrupt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_int_raw`] module"]
pub type TIMER_INT_RAW = crate::Reg<timer_int_raw::TIMER_INT_RAW_SPEC>;
#[doc = "TWAIFD raw interrupt register."]
pub mod timer_int_raw;
#[doc = "TIMER_INT_ST (r) register accessor: TWAIFD interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_int_st`] module"]
pub type TIMER_INT_ST = crate::Reg<timer_int_st::TIMER_INT_ST_SPEC>;
#[doc = "TWAIFD interrupt status register."]
pub mod timer_int_st;
#[doc = "TIMER_INT_ENA (rw) register accessor: TWAIFD interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_int_ena`] module"]
pub type TIMER_INT_ENA = crate::Reg<timer_int_ena::TIMER_INT_ENA_SPEC>;
#[doc = "TWAIFD interrupt enable register."]
pub mod timer_int_ena;
#[doc = "TIMER_INT_CLR (w) register accessor: TWAIFD interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_int_clr`] module"]
pub type TIMER_INT_CLR = crate::Reg<timer_int_clr::TIMER_INT_CLR_SPEC>;
#[doc = "TWAIFD interrupt clear register."]
pub mod timer_int_clr;
#[doc = "TIMER_CFG (rw) register accessor: TWAI FD timer configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cfg`] module"]
pub type TIMER_CFG = crate::Reg<timer_cfg::TIMER_CFG_SPEC>;
#[doc = "TWAI FD timer configure register."]
pub mod timer_cfg;
#[doc = "TIMER_LD_VAL_L (rw) register accessor: TWAI FD timer pre-load value low register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ld_val_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ld_val_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ld_val_l`] module"]
pub type TIMER_LD_VAL_L = crate::Reg<timer_ld_val_l::TIMER_LD_VAL_L_SPEC>;
#[doc = "TWAI FD timer pre-load value low register."]
pub mod timer_ld_val_l;
#[doc = "TIMER_LD_VAL_H (rw) register accessor: TWAI FD timer pre-load value high register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ld_val_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ld_val_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ld_val_h`] module"]
pub type TIMER_LD_VAL_H = crate::Reg<timer_ld_val_h::TIMER_LD_VAL_H_SPEC>;
#[doc = "TWAI FD timer pre-load value high register."]
pub mod timer_ld_val_h;
#[doc = "TIMER_CT_VAL_L (rw) register accessor: TWAI FD timer count-to value low register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ct_val_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ct_val_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ct_val_l`] module"]
pub type TIMER_CT_VAL_L = crate::Reg<timer_ct_val_l::TIMER_CT_VAL_L_SPEC>;
#[doc = "TWAI FD timer count-to value low register."]
pub mod timer_ct_val_l;
#[doc = "TIMER_CT_VAL_H (rw) register accessor: TWAI FD timer count-to value high register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ct_val_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ct_val_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ct_val_h`] module"]
pub type TIMER_CT_VAL_H = crate::Reg<timer_ct_val_h::TIMER_CT_VAL_H_SPEC>;
#[doc = "TWAI FD timer count-to value high register."]
pub mod timer_ct_val_h;
#[doc = "DATE_VER (rw) register accessor: TWAI FD date version\n\nYou can [`read`](crate::Reg::read) this register and get [`date_ver::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date_ver::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date_ver`] module"]
pub type DATE_VER = crate::Reg<date_ver::DATE_VER_SPEC>;
#[doc = "TWAI FD date version"]
pub mod date_ver;
