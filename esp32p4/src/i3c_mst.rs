#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    device_ctrl: DEVICE_CTRL,
    _reserved1: [u8; 0x18],
    buffer_thld_ctrl: BUFFER_THLD_CTRL,
    data_buffer_thld_ctrl: DATA_BUFFER_THLD_CTRL,
    ibi_notify_ctrl: IBI_NOTIFY_CTRL,
    ibi_sir_req_payload: IBI_SIR_REQ_PAYLOAD,
    ibi_sir_req_reject: IBI_SIR_REQ_REJECT,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_st_ena: INT_ST_ENA,
    _reserved10: [u8; 0x04],
    reset_ctrl: RESET_CTRL,
    buffer_status_level: BUFFER_STATUS_LEVEL,
    data_buffer_status_level: DATA_BUFFER_STATUS_LEVEL,
    present_state0: PRESENT_STATE0,
    present_state1: PRESENT_STATE1,
    device_table: DEVICE_TABLE,
    time_out_value: TIME_OUT_VALUE,
    scl_i3c_mst_od_time: SCL_I3C_MST_OD_TIME,
    scl_i3c_mst_pp_time: SCL_I3C_MST_PP_TIME,
    scl_i2c_fm_time: SCL_I2C_FM_TIME,
    scl_i2c_fmp_time: SCL_I2C_FMP_TIME,
    scl_ext_low_time: SCL_EXT_LOW_TIME,
    sda_sample_time: SDA_SAMPLE_TIME,
    sda_hold_time: SDA_HOLD_TIME,
    scl_start_hold: SCL_START_HOLD,
    scl_rstart_setup: SCL_RSTART_SETUP,
    scl_stop_hold: SCL_STOP_HOLD,
    scl_stop_setup: SCL_STOP_SETUP,
    _reserved28: [u8; 0x04],
    bus_free_time: BUS_FREE_TIME,
    scl_termn_t_ext_low_time: SCL_TERMN_T_EXT_LOW_TIME,
    _reserved30: [u8; 0x08],
    ver_id: VER_ID,
    ver_type: VER_TYPE,
    _reserved32: [u8; 0x04],
    fpga_debug_probe: FPGA_DEBUG_PROBE,
    rnd_eco_cs: RND_ECO_CS,
    rnd_eco_low: RND_ECO_LOW,
    rnd_eco_high: RND_ECO_HIGH,
}
impl RegisterBlock {
    #[doc = "0x00 - DEVICE_CTRL register controls the transfer properties and disposition of controllers capabilities."]
    #[inline(always)]
    pub const fn device_ctrl(&self) -> &DEVICE_CTRL {
        &self.device_ctrl
    }
    #[doc = "0x1c - In-Band Interrupt Status Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI status entries in the IBI buffer that trigger the IBI_STATUS_THLD_STAT interrupt."]
    #[inline(always)]
    pub const fn buffer_thld_ctrl(&self) -> &BUFFER_THLD_CTRL {
        &self.buffer_thld_ctrl
    }
    #[doc = "0x20 - NA"]
    #[inline(always)]
    pub const fn data_buffer_thld_ctrl(&self) -> &DATA_BUFFER_THLD_CTRL {
        &self.data_buffer_thld_ctrl
    }
    #[doc = "0x24 - NA"]
    #[inline(always)]
    pub const fn ibi_notify_ctrl(&self) -> &IBI_NOTIFY_CTRL {
        &self.ibi_notify_ctrl
    }
    #[doc = "0x28 - NA"]
    #[inline(always)]
    pub const fn ibi_sir_req_payload(&self) -> &IBI_SIR_REQ_PAYLOAD {
        &self.ibi_sir_req_payload
    }
    #[doc = "0x2c - NA"]
    #[inline(always)]
    pub const fn ibi_sir_req_reject(&self) -> &IBI_SIR_REQ_REJECT {
        &self.ibi_sir_req_reject
    }
    #[doc = "0x30 - NA"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x34 - NA"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x3c - The Interrupt status will be updated in INTR_STATUS register if corresponding Status Enable bit set."]
    #[inline(always)]
    pub const fn int_st_ena(&self) -> &INT_ST_ENA {
        &self.int_st_ena
    }
    #[doc = "0x44 - NA"]
    #[inline(always)]
    pub const fn reset_ctrl(&self) -> &RESET_CTRL {
        &self.reset_ctrl
    }
    #[doc = "0x48 - BUFFER_STATUS_LEVEL reflects the status level of Buffers in the controller."]
    #[inline(always)]
    pub const fn buffer_status_level(&self) -> &BUFFER_STATUS_LEVEL {
        &self.buffer_status_level
    }
    #[doc = "0x4c - DATA_BUFFER_STATUS_LEVEL reflects the status level of the Buffers in the controller."]
    #[inline(always)]
    pub const fn data_buffer_status_level(&self) -> &DATA_BUFFER_STATUS_LEVEL {
        &self.data_buffer_status_level
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn present_state0(&self) -> &PRESENT_STATE0 {
        &self.present_state0
    }
    #[doc = "0x54 - NA"]
    #[inline(always)]
    pub const fn present_state1(&self) -> &PRESENT_STATE1 {
        &self.present_state1
    }
    #[doc = "0x58 - Pointer for Device Address Table"]
    #[inline(always)]
    pub const fn device_table(&self) -> &DEVICE_TABLE {
        &self.device_table
    }
    #[doc = "0x5c - NA"]
    #[inline(always)]
    pub const fn time_out_value(&self) -> &TIME_OUT_VALUE {
        &self.time_out_value
    }
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn scl_i3c_mst_od_time(&self) -> &SCL_I3C_MST_OD_TIME {
        &self.scl_i3c_mst_od_time
    }
    #[doc = "0x64 - NA"]
    #[inline(always)]
    pub const fn scl_i3c_mst_pp_time(&self) -> &SCL_I3C_MST_PP_TIME {
        &self.scl_i3c_mst_pp_time
    }
    #[doc = "0x68 - NA"]
    #[inline(always)]
    pub const fn scl_i2c_fm_time(&self) -> &SCL_I2C_FM_TIME {
        &self.scl_i2c_fm_time
    }
    #[doc = "0x6c - NA"]
    #[inline(always)]
    pub const fn scl_i2c_fmp_time(&self) -> &SCL_I2C_FMP_TIME {
        &self.scl_i2c_fmp_time
    }
    #[doc = "0x70 - NA"]
    #[inline(always)]
    pub const fn scl_ext_low_time(&self) -> &SCL_EXT_LOW_TIME {
        &self.scl_ext_low_time
    }
    #[doc = "0x74 - NA"]
    #[inline(always)]
    pub const fn sda_sample_time(&self) -> &SDA_SAMPLE_TIME {
        &self.sda_sample_time
    }
    #[doc = "0x78 - NA"]
    #[inline(always)]
    pub const fn sda_hold_time(&self) -> &SDA_HOLD_TIME {
        &self.sda_hold_time
    }
    #[doc = "0x7c - NA"]
    #[inline(always)]
    pub const fn scl_start_hold(&self) -> &SCL_START_HOLD {
        &self.scl_start_hold
    }
    #[doc = "0x80 - NA"]
    #[inline(always)]
    pub const fn scl_rstart_setup(&self) -> &SCL_RSTART_SETUP {
        &self.scl_rstart_setup
    }
    #[doc = "0x84 - NA"]
    #[inline(always)]
    pub const fn scl_stop_hold(&self) -> &SCL_STOP_HOLD {
        &self.scl_stop_hold
    }
    #[doc = "0x88 - NA"]
    #[inline(always)]
    pub const fn scl_stop_setup(&self) -> &SCL_STOP_SETUP {
        &self.scl_stop_setup
    }
    #[doc = "0x90 - NA"]
    #[inline(always)]
    pub const fn bus_free_time(&self) -> &BUS_FREE_TIME {
        &self.bus_free_time
    }
    #[doc = "0x94 - NA"]
    #[inline(always)]
    pub const fn scl_termn_t_ext_low_time(&self) -> &SCL_TERMN_T_EXT_LOW_TIME {
        &self.scl_termn_t_ext_low_time
    }
    #[doc = "0xa0 - NA"]
    #[inline(always)]
    pub const fn ver_id(&self) -> &VER_ID {
        &self.ver_id
    }
    #[doc = "0xa4 - NA"]
    #[inline(always)]
    pub const fn ver_type(&self) -> &VER_TYPE {
        &self.ver_type
    }
    #[doc = "0xac - NA"]
    #[inline(always)]
    pub const fn fpga_debug_probe(&self) -> &FPGA_DEBUG_PROBE {
        &self.fpga_debug_probe
    }
    #[doc = "0xb0 - NA"]
    #[inline(always)]
    pub const fn rnd_eco_cs(&self) -> &RND_ECO_CS {
        &self.rnd_eco_cs
    }
    #[doc = "0xb4 - NA"]
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> &RND_ECO_LOW {
        &self.rnd_eco_low
    }
    #[doc = "0xb8 - NA"]
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> &RND_ECO_HIGH {
        &self.rnd_eco_high
    }
}
#[doc = "DEVICE_CTRL (rw) register accessor: DEVICE_CTRL register controls the transfer properties and disposition of controllers capabilities.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_ctrl`] module"]
pub type DEVICE_CTRL = crate::Reg<device_ctrl::DEVICE_CTRL_SPEC>;
#[doc = "DEVICE_CTRL register controls the transfer properties and disposition of controllers capabilities."]
pub mod device_ctrl;
#[doc = "BUFFER_THLD_CTRL (rw) register accessor: In-Band Interrupt Status Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI status entries in the IBI buffer that trigger the IBI_STATUS_THLD_STAT interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buffer_thld_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffer_thld_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffer_thld_ctrl`] module"]
pub type BUFFER_THLD_CTRL = crate::Reg<buffer_thld_ctrl::BUFFER_THLD_CTRL_SPEC>;
#[doc = "In-Band Interrupt Status Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI status entries in the IBI buffer that trigger the IBI_STATUS_THLD_STAT interrupt."]
pub mod buffer_thld_ctrl;
#[doc = "DATA_BUFFER_THLD_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_buffer_thld_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_buffer_thld_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_buffer_thld_ctrl`] module"]
pub type DATA_BUFFER_THLD_CTRL = crate::Reg<data_buffer_thld_ctrl::DATA_BUFFER_THLD_CTRL_SPEC>;
#[doc = "NA"]
pub mod data_buffer_thld_ctrl;
#[doc = "IBI_NOTIFY_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibi_notify_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibi_notify_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibi_notify_ctrl`] module"]
pub type IBI_NOTIFY_CTRL = crate::Reg<ibi_notify_ctrl::IBI_NOTIFY_CTRL_SPEC>;
#[doc = "NA"]
pub mod ibi_notify_ctrl;
#[doc = "IBI_SIR_REQ_PAYLOAD (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibi_sir_req_payload::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibi_sir_req_payload::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibi_sir_req_payload`] module"]
pub type IBI_SIR_REQ_PAYLOAD = crate::Reg<ibi_sir_req_payload::IBI_SIR_REQ_PAYLOAD_SPEC>;
#[doc = "NA"]
pub mod ibi_sir_req_payload;
#[doc = "IBI_SIR_REQ_REJECT (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibi_sir_req_reject::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibi_sir_req_reject::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibi_sir_req_reject`] module"]
pub type IBI_SIR_REQ_REJECT = crate::Reg<ibi_sir_req_reject::IBI_SIR_REQ_REJECT_SPEC>;
#[doc = "NA"]
pub mod ibi_sir_req_reject;
#[doc = "INT_CLR (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "NA"]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "NA"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "NA"]
pub mod int_st;
#[doc = "INT_ST_ENA (rw) register accessor: The Interrupt status will be updated in INTR_STATUS register if corresponding Status Enable bit set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_st_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_ena`] module"]
pub type INT_ST_ENA = crate::Reg<int_st_ena::INT_ST_ENA_SPEC>;
#[doc = "The Interrupt status will be updated in INTR_STATUS register if corresponding Status Enable bit set."]
pub mod int_st_ena;
#[doc = "RESET_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_ctrl`] module"]
pub type RESET_CTRL = crate::Reg<reset_ctrl::RESET_CTRL_SPEC>;
#[doc = "NA"]
pub mod reset_ctrl;
#[doc = "BUFFER_STATUS_LEVEL (r) register accessor: BUFFER_STATUS_LEVEL reflects the status level of Buffers in the controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buffer_status_level::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffer_status_level`] module"]
pub type BUFFER_STATUS_LEVEL = crate::Reg<buffer_status_level::BUFFER_STATUS_LEVEL_SPEC>;
#[doc = "BUFFER_STATUS_LEVEL reflects the status level of Buffers in the controller."]
pub mod buffer_status_level;
#[doc = "DATA_BUFFER_STATUS_LEVEL (r) register accessor: DATA_BUFFER_STATUS_LEVEL reflects the status level of the Buffers in the controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_buffer_status_level::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_buffer_status_level`] module"]
pub type DATA_BUFFER_STATUS_LEVEL =
    crate::Reg<data_buffer_status_level::DATA_BUFFER_STATUS_LEVEL_SPEC>;
#[doc = "DATA_BUFFER_STATUS_LEVEL reflects the status level of the Buffers in the controller."]
pub mod data_buffer_status_level;
#[doc = "PRESENT_STATE0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`present_state0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@present_state0`] module"]
pub type PRESENT_STATE0 = crate::Reg<present_state0::PRESENT_STATE0_SPEC>;
#[doc = "NA"]
pub mod present_state0;
#[doc = "PRESENT_STATE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`present_state1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@present_state1`] module"]
pub type PRESENT_STATE1 = crate::Reg<present_state1::PRESENT_STATE1_SPEC>;
#[doc = "NA"]
pub mod present_state1;
#[doc = "DEVICE_TABLE (rw) register accessor: Pointer for Device Address Table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_table::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_table::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_table`] module"]
pub type DEVICE_TABLE = crate::Reg<device_table::DEVICE_TABLE_SPEC>;
#[doc = "Pointer for Device Address Table"]
pub mod device_table;
#[doc = "TIME_OUT_VALUE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_out_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_out_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_out_value`] module"]
pub type TIME_OUT_VALUE = crate::Reg<time_out_value::TIME_OUT_VALUE_SPEC>;
#[doc = "NA"]
pub mod time_out_value;
#[doc = "SCL_I3C_MST_OD_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_i3c_mst_od_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_i3c_mst_od_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_i3c_mst_od_time`] module"]
pub type SCL_I3C_MST_OD_TIME = crate::Reg<scl_i3c_mst_od_time::SCL_I3C_MST_OD_TIME_SPEC>;
#[doc = "NA"]
pub mod scl_i3c_mst_od_time;
#[doc = "SCL_I3C_MST_PP_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_i3c_mst_pp_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_i3c_mst_pp_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_i3c_mst_pp_time`] module"]
pub type SCL_I3C_MST_PP_TIME = crate::Reg<scl_i3c_mst_pp_time::SCL_I3C_MST_PP_TIME_SPEC>;
#[doc = "NA"]
pub mod scl_i3c_mst_pp_time;
#[doc = "SCL_I2C_FM_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_i2c_fm_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_i2c_fm_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_i2c_fm_time`] module"]
pub type SCL_I2C_FM_TIME = crate::Reg<scl_i2c_fm_time::SCL_I2C_FM_TIME_SPEC>;
#[doc = "NA"]
pub mod scl_i2c_fm_time;
#[doc = "SCL_I2C_FMP_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_i2c_fmp_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_i2c_fmp_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_i2c_fmp_time`] module"]
pub type SCL_I2C_FMP_TIME = crate::Reg<scl_i2c_fmp_time::SCL_I2C_FMP_TIME_SPEC>;
#[doc = "NA"]
pub mod scl_i2c_fmp_time;
#[doc = "SCL_EXT_LOW_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_ext_low_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_ext_low_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_ext_low_time`] module"]
pub type SCL_EXT_LOW_TIME = crate::Reg<scl_ext_low_time::SCL_EXT_LOW_TIME_SPEC>;
#[doc = "NA"]
pub mod scl_ext_low_time;
#[doc = "SDA_SAMPLE_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_sample_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_sample_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_sample_time`] module"]
pub type SDA_SAMPLE_TIME = crate::Reg<sda_sample_time::SDA_SAMPLE_TIME_SPEC>;
#[doc = "NA"]
pub mod sda_sample_time;
#[doc = "SDA_HOLD_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_hold_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_hold_time`] module"]
pub type SDA_HOLD_TIME = crate::Reg<sda_hold_time::SDA_HOLD_TIME_SPEC>;
#[doc = "NA"]
pub mod sda_hold_time;
#[doc = "SCL_START_HOLD (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_start_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_start_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_start_hold`] module"]
pub type SCL_START_HOLD = crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>;
#[doc = "NA"]
pub mod scl_start_hold;
#[doc = "SCL_RSTART_SETUP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_rstart_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_rstart_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_rstart_setup`] module"]
pub type SCL_RSTART_SETUP = crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>;
#[doc = "NA"]
pub mod scl_rstart_setup;
#[doc = "SCL_STOP_HOLD (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_stop_hold`] module"]
pub type SCL_STOP_HOLD = crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>;
#[doc = "NA"]
pub mod scl_stop_hold;
#[doc = "SCL_STOP_SETUP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_stop_setup`] module"]
pub type SCL_STOP_SETUP = crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>;
#[doc = "NA"]
pub mod scl_stop_setup;
#[doc = "BUS_FREE_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_free_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_free_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_free_time`] module"]
pub type BUS_FREE_TIME = crate::Reg<bus_free_time::BUS_FREE_TIME_SPEC>;
#[doc = "NA"]
pub mod bus_free_time;
#[doc = "SCL_TERMN_T_EXT_LOW_TIME (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_termn_t_ext_low_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_termn_t_ext_low_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_termn_t_ext_low_time`] module"]
pub type SCL_TERMN_T_EXT_LOW_TIME =
    crate::Reg<scl_termn_t_ext_low_time::SCL_TERMN_T_EXT_LOW_TIME_SPEC>;
#[doc = "NA"]
pub mod scl_termn_t_ext_low_time;
#[doc = "VER_ID (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_id`] module"]
pub type VER_ID = crate::Reg<ver_id::VER_ID_SPEC>;
#[doc = "NA"]
pub mod ver_id;
#[doc = "VER_TYPE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_type`] module"]
pub type VER_TYPE = crate::Reg<ver_type::VER_TYPE_SPEC>;
#[doc = "NA"]
pub mod ver_type;
#[doc = "FPGA_DEBUG_PROBE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpga_debug_probe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpga_debug_probe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpga_debug_probe`] module"]
pub type FPGA_DEBUG_PROBE = crate::Reg<fpga_debug_probe::FPGA_DEBUG_PROBE_SPEC>;
#[doc = "NA"]
pub mod fpga_debug_probe;
#[doc = "RND_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_cs`] module"]
pub type RND_ECO_CS = crate::Reg<rnd_eco_cs::RND_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod rnd_eco_cs;
#[doc = "RND_ECO_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_low`] module"]
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
#[doc = "NA"]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_high`] module"]
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
#[doc = "NA"]
pub mod rnd_eco_high;
