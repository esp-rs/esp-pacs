#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: CONFIG,
    dqt_info: DQT_INFO,
    pic_size: PIC_SIZE,
    _reserved3: [u8; 0x04],
    t0qnr: T0QNR,
    t1qnr: T1QNR,
    t2qnr: T2QNR,
    t3qnr: T3QNR,
    decode_conf: DECODE_CONF,
    c0: C0,
    c1: C1,
    c2: C2,
    c3: C3,
    dht_info: DHT_INFO,
    int_raw: INT_RAW,
    int_ena: INT_ENA,
    int_st: INT_ST,
    int_clr: INT_CLR,
    status0: STATUS0,
    status2: STATUS2,
    status3: STATUS3,
    status4: STATUS4,
    dht_totlen_dc0: DHT_TOTLEN_DC0,
    dht_val_dc0: DHT_VAL_DC0,
    dht_totlen_ac0: DHT_TOTLEN_AC0,
    dht_val_ac0: DHT_VAL_AC0,
    dht_totlen_dc1: DHT_TOTLEN_DC1,
    dht_val_dc1: DHT_VAL_DC1,
    dht_totlen_ac1: DHT_TOTLEN_AC1,
    dht_val_ac1: DHT_VAL_AC1,
    dht_codemin_dc0: DHT_CODEMIN_DC0,
    dht_codemin_ac0: DHT_CODEMIN_AC0,
    dht_codemin_dc1: DHT_CODEMIN_DC1,
    dht_codemin_ac1: DHT_CODEMIN_AC1,
    decoder_status0: DECODER_STATUS0,
    decoder_status1: DECODER_STATUS1,
    decoder_status2: DECODER_STATUS2,
    decoder_status3: DECODER_STATUS3,
    decoder_status4: DECODER_STATUS4,
    decoder_status5: DECODER_STATUS5,
    status5: STATUS5,
    eco_low: ECO_LOW,
    eco_high: ECO_HIGH,
    _reserved42: [u8; 0x4c],
    sys: SYS,
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and configuration registers"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - Control and configuration registers"]
    #[inline(always)]
    pub const fn dqt_info(&self) -> &DQT_INFO {
        &self.dqt_info
    }
    #[doc = "0x08 - Control and configuration registers"]
    #[inline(always)]
    pub const fn pic_size(&self) -> &PIC_SIZE {
        &self.pic_size
    }
    #[doc = "0x10 - Control and configuration registers"]
    #[inline(always)]
    pub const fn t0qnr(&self) -> &T0QNR {
        &self.t0qnr
    }
    #[doc = "0x14 - Control and configuration registers"]
    #[inline(always)]
    pub const fn t1qnr(&self) -> &T1QNR {
        &self.t1qnr
    }
    #[doc = "0x18 - Control and configuration registers"]
    #[inline(always)]
    pub const fn t2qnr(&self) -> &T2QNR {
        &self.t2qnr
    }
    #[doc = "0x1c - Control and configuration registers"]
    #[inline(always)]
    pub const fn t3qnr(&self) -> &T3QNR {
        &self.t3qnr
    }
    #[doc = "0x20 - Control and configuration registers"]
    #[inline(always)]
    pub const fn decode_conf(&self) -> &DECODE_CONF {
        &self.decode_conf
    }
    #[doc = "0x24 - Control and configuration registers"]
    #[inline(always)]
    pub const fn c0(&self) -> &C0 {
        &self.c0
    }
    #[doc = "0x28 - Control and configuration registers"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x2c - Control and configuration registers"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x30 - Control and configuration registers"]
    #[inline(always)]
    pub const fn c3(&self) -> &C3 {
        &self.c3
    }
    #[doc = "0x34 - Control and configuration registers"]
    #[inline(always)]
    pub const fn dht_info(&self) -> &DHT_INFO {
        &self.dht_info
    }
    #[doc = "0x38 - Interrupt raw registers"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x3c - Interrupt enable registers"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x40 - Interrupt status registers"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x44 - Interrupt clear registers"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x48 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn status0(&self) -> &STATUS0 {
        &self.status0
    }
    #[doc = "0x4c - Trace and Debug registers"]
    #[inline(always)]
    pub const fn status2(&self) -> &STATUS2 {
        &self.status2
    }
    #[doc = "0x50 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn status3(&self) -> &STATUS3 {
        &self.status3
    }
    #[doc = "0x54 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn status4(&self) -> &STATUS4 {
        &self.status4
    }
    #[doc = "0x58 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_totlen_dc0(&self) -> &DHT_TOTLEN_DC0 {
        &self.dht_totlen_dc0
    }
    #[doc = "0x5c - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_val_dc0(&self) -> &DHT_VAL_DC0 {
        &self.dht_val_dc0
    }
    #[doc = "0x60 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_totlen_ac0(&self) -> &DHT_TOTLEN_AC0 {
        &self.dht_totlen_ac0
    }
    #[doc = "0x64 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_val_ac0(&self) -> &DHT_VAL_AC0 {
        &self.dht_val_ac0
    }
    #[doc = "0x68 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_totlen_dc1(&self) -> &DHT_TOTLEN_DC1 {
        &self.dht_totlen_dc1
    }
    #[doc = "0x6c - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_val_dc1(&self) -> &DHT_VAL_DC1 {
        &self.dht_val_dc1
    }
    #[doc = "0x70 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_totlen_ac1(&self) -> &DHT_TOTLEN_AC1 {
        &self.dht_totlen_ac1
    }
    #[doc = "0x74 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_val_ac1(&self) -> &DHT_VAL_AC1 {
        &self.dht_val_ac1
    }
    #[doc = "0x78 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_codemin_dc0(&self) -> &DHT_CODEMIN_DC0 {
        &self.dht_codemin_dc0
    }
    #[doc = "0x7c - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_codemin_ac0(&self) -> &DHT_CODEMIN_AC0 {
        &self.dht_codemin_ac0
    }
    #[doc = "0x80 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_codemin_dc1(&self) -> &DHT_CODEMIN_DC1 {
        &self.dht_codemin_dc1
    }
    #[doc = "0x84 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn dht_codemin_ac1(&self) -> &DHT_CODEMIN_AC1 {
        &self.dht_codemin_ac1
    }
    #[doc = "0x88 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn decoder_status0(&self) -> &DECODER_STATUS0 {
        &self.decoder_status0
    }
    #[doc = "0x8c - Trace and Debug registers"]
    #[inline(always)]
    pub const fn decoder_status1(&self) -> &DECODER_STATUS1 {
        &self.decoder_status1
    }
    #[doc = "0x90 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn decoder_status2(&self) -> &DECODER_STATUS2 {
        &self.decoder_status2
    }
    #[doc = "0x94 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn decoder_status3(&self) -> &DECODER_STATUS3 {
        &self.decoder_status3
    }
    #[doc = "0x98 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn decoder_status4(&self) -> &DECODER_STATUS4 {
        &self.decoder_status4
    }
    #[doc = "0x9c - Trace and Debug registers"]
    #[inline(always)]
    pub const fn decoder_status5(&self) -> &DECODER_STATUS5 {
        &self.decoder_status5
    }
    #[doc = "0xa0 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn status5(&self) -> &STATUS5 {
        &self.status5
    }
    #[doc = "0xa4 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn eco_low(&self) -> &ECO_LOW {
        &self.eco_low
    }
    #[doc = "0xa8 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn eco_high(&self) -> &ECO_HIGH {
        &self.eco_high
    }
    #[doc = "0xf8 - Trace and Debug registers"]
    #[inline(always)]
    pub const fn sys(&self) -> &SYS {
        &self.sys
    }
    #[doc = "0xfc - Trace and Debug registers"]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "CONFIG (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Control and configuration registers"]
pub mod config;
#[doc = "DQT_INFO (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dqt_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dqt_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dqt_info`] module"]
pub type DQT_INFO = crate::Reg<dqt_info::DQT_INFO_SPEC>;
#[doc = "Control and configuration registers"]
pub mod dqt_info;
#[doc = "PIC_SIZE (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pic_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pic_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pic_size`] module"]
pub type PIC_SIZE = crate::Reg<pic_size::PIC_SIZE_SPEC>;
#[doc = "Control and configuration registers"]
pub mod pic_size;
#[doc = "T0QNR (r) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0qnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0qnr`] module"]
pub type T0QNR = crate::Reg<t0qnr::T0QNR_SPEC>;
#[doc = "Control and configuration registers"]
pub mod t0qnr;
#[doc = "T1QNR (r) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1qnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1qnr`] module"]
pub type T1QNR = crate::Reg<t1qnr::T1QNR_SPEC>;
#[doc = "Control and configuration registers"]
pub mod t1qnr;
#[doc = "T2QNR (r) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2qnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2qnr`] module"]
pub type T2QNR = crate::Reg<t2qnr::T2QNR_SPEC>;
#[doc = "Control and configuration registers"]
pub mod t2qnr;
#[doc = "T3QNR (r) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3qnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3qnr`] module"]
pub type T3QNR = crate::Reg<t3qnr::T3QNR_SPEC>;
#[doc = "Control and configuration registers"]
pub mod t3qnr;
#[doc = "DECODE_CONF (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decode_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`decode_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decode_conf`] module"]
pub type DECODE_CONF = crate::Reg<decode_conf::DECODE_CONF_SPEC>;
#[doc = "Control and configuration registers"]
pub mod decode_conf;
#[doc = "C0 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0`] module"]
pub type C0 = crate::Reg<c0::C0_SPEC>;
#[doc = "Control and configuration registers"]
pub mod c0;
#[doc = "C1 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`] module"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "Control and configuration registers"]
pub mod c1;
#[doc = "C2 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`] module"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "Control and configuration registers"]
pub mod c2;
#[doc = "C3 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3`] module"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "Control and configuration registers"]
pub mod c3;
#[doc = "DHT_INFO (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dht_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_info`] module"]
pub type DHT_INFO = crate::Reg<dht_info::DHT_INFO_SPEC>;
#[doc = "Control and configuration registers"]
pub mod dht_info;
#[doc = "INT_RAW (rw) register accessor: Interrupt raw registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw registers"]
pub mod int_raw;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable registers"]
pub mod int_ena;
#[doc = "INT_ST (r) register accessor: Interrupt status registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt status registers"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: Interrupt clear registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear registers"]
pub mod int_clr;
#[doc = "STATUS0 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status0`] module"]
pub type STATUS0 = crate::Reg<status0::STATUS0_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod status0;
#[doc = "STATUS2 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status2`] module"]
pub type STATUS2 = crate::Reg<status2::STATUS2_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod status2;
#[doc = "STATUS3 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status3`] module"]
pub type STATUS3 = crate::Reg<status3::STATUS3_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod status3;
#[doc = "STATUS4 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status4`] module"]
pub type STATUS4 = crate::Reg<status4::STATUS4_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod status4;
#[doc = "DHT_TOTLEN_DC0 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_totlen_dc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_totlen_dc0`] module"]
pub type DHT_TOTLEN_DC0 = crate::Reg<dht_totlen_dc0::DHT_TOTLEN_DC0_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_totlen_dc0;
#[doc = "DHT_VAl_DC0 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_val_dc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_val_dc0`] module"]
#[doc(alias = "DHT_VAl_DC0")]
pub type DHT_VAL_DC0 = crate::Reg<dht_val_dc0::DHT_VAL_DC0_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_val_dc0;
#[doc = "DHT_TOTLEN_AC0 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_totlen_ac0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_totlen_ac0`] module"]
pub type DHT_TOTLEN_AC0 = crate::Reg<dht_totlen_ac0::DHT_TOTLEN_AC0_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_totlen_ac0;
#[doc = "DHT_VAl_AC0 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_val_ac0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_val_ac0`] module"]
#[doc(alias = "DHT_VAl_AC0")]
pub type DHT_VAL_AC0 = crate::Reg<dht_val_ac0::DHT_VAL_AC0_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_val_ac0;
#[doc = "DHT_TOTLEN_DC1 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_totlen_dc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_totlen_dc1`] module"]
pub type DHT_TOTLEN_DC1 = crate::Reg<dht_totlen_dc1::DHT_TOTLEN_DC1_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_totlen_dc1;
#[doc = "DHT_VAl_DC1 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_val_dc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_val_dc1`] module"]
#[doc(alias = "DHT_VAl_DC1")]
pub type DHT_VAL_DC1 = crate::Reg<dht_val_dc1::DHT_VAL_DC1_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_val_dc1;
#[doc = "DHT_TOTLEN_AC1 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_totlen_ac1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_totlen_ac1`] module"]
pub type DHT_TOTLEN_AC1 = crate::Reg<dht_totlen_ac1::DHT_TOTLEN_AC1_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_totlen_ac1;
#[doc = "DHT_VAl_AC1 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_val_ac1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_val_ac1`] module"]
#[doc(alias = "DHT_VAl_AC1")]
pub type DHT_VAL_AC1 = crate::Reg<dht_val_ac1::DHT_VAL_AC1_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_val_ac1;
#[doc = "DHT_CODEMIN_DC0 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_codemin_dc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_codemin_dc0`] module"]
pub type DHT_CODEMIN_DC0 = crate::Reg<dht_codemin_dc0::DHT_CODEMIN_DC0_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_codemin_dc0;
#[doc = "DHT_CODEMIN_AC0 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_codemin_ac0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_codemin_ac0`] module"]
pub type DHT_CODEMIN_AC0 = crate::Reg<dht_codemin_ac0::DHT_CODEMIN_AC0_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_codemin_ac0;
#[doc = "DHT_CODEMIN_DC1 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_codemin_dc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_codemin_dc1`] module"]
pub type DHT_CODEMIN_DC1 = crate::Reg<dht_codemin_dc1::DHT_CODEMIN_DC1_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_codemin_dc1;
#[doc = "DHT_CODEMIN_AC1 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_codemin_ac1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dht_codemin_ac1`] module"]
pub type DHT_CODEMIN_AC1 = crate::Reg<dht_codemin_ac1::DHT_CODEMIN_AC1_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod dht_codemin_ac1;
#[doc = "DECODER_STATUS0 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decoder_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decoder_status0`] module"]
pub type DECODER_STATUS0 = crate::Reg<decoder_status0::DECODER_STATUS0_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod decoder_status0;
#[doc = "DECODER_STATUS1 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decoder_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decoder_status1`] module"]
pub type DECODER_STATUS1 = crate::Reg<decoder_status1::DECODER_STATUS1_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod decoder_status1;
#[doc = "DECODER_STATUS2 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decoder_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decoder_status2`] module"]
pub type DECODER_STATUS2 = crate::Reg<decoder_status2::DECODER_STATUS2_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod decoder_status2;
#[doc = "DECODER_STATUS3 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decoder_status3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decoder_status3`] module"]
pub type DECODER_STATUS3 = crate::Reg<decoder_status3::DECODER_STATUS3_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod decoder_status3;
#[doc = "DECODER_STATUS4 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decoder_status4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decoder_status4`] module"]
pub type DECODER_STATUS4 = crate::Reg<decoder_status4::DECODER_STATUS4_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod decoder_status4;
#[doc = "DECODER_STATUS5 (rw) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decoder_status5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`decoder_status5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decoder_status5`] module"]
pub type DECODER_STATUS5 = crate::Reg<decoder_status5::DECODER_STATUS5_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod decoder_status5;
#[doc = "STATUS5 (r) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status5`] module"]
pub type STATUS5 = crate::Reg<status5::STATUS5_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod status5;
#[doc = "ECO_LOW (rw) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eco_low`] module"]
pub type ECO_LOW = crate::Reg<eco_low::ECO_LOW_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod eco_low;
#[doc = "ECO_HIGH (rw) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eco_high`] module"]
pub type ECO_HIGH = crate::Reg<eco_high::ECO_HIGH_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod eco_high;
#[doc = "SYS (rw) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys`] module"]
pub type SYS = crate::Reg<sys::SYS_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod sys;
#[doc = "VERSION (rw) register accessor: Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Trace and Debug registers"]
pub mod version;
