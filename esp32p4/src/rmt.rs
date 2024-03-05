#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    tx_chdata: [TX_CHDATA; 4],
    rx_chdata: [RX_CHDATA; 4],
    tx_chconf0: [TX_CHCONF0; 4],
    rx_chconf0: (),
    _reserved4: [u8; 0x04],
    rx_chconf1: (),
    _reserved5: [u8; 0x1c],
    tx_chstatus: [TX_CHSTATUS; 4],
    rx_chstatus: [RX_CHSTATUS; 4],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    chcarrier_duty: [CHCARRIER_DUTY; 4],
    ch_rx_carrier_rm: [CH_RX_CARRIER_RM; 4],
    ch_tx_lim: [CH_TX_LIM; 4],
    ch_rx_lim: [CH_RX_LIM; 4],
    sys_conf: SYS_CONF,
    tx_sim: TX_SIM,
    ref_cnt_rst: REF_CNT_RST,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - The read and write data register for CHANNEL%s by apb fifo access."]
    #[inline(always)]
    pub const fn tx_chdata(&self, n: usize) -> &TX_CHDATA {
        &self.tx_chdata[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - The read and write data register for CHANNEL%s by apb fifo access."]
    #[inline(always)]
    pub fn tx_chdata_iter(&self) -> impl Iterator<Item = &TX_CHDATA> {
        self.tx_chdata.iter()
    }
    #[doc = "0x00 - The read and write data register for CHANNEL0 by apb fifo access."]
    #[inline(always)]
    pub const fn tx_ch0data(&self) -> &TX_CHDATA {
        self.tx_chdata(0)
    }
    #[doc = "0x04 - The read and write data register for CHANNEL1 by apb fifo access."]
    #[inline(always)]
    pub const fn tx_ch1data(&self) -> &TX_CHDATA {
        self.tx_chdata(1)
    }
    #[doc = "0x08 - The read and write data register for CHANNEL2 by apb fifo access."]
    #[inline(always)]
    pub const fn tx_ch2data(&self) -> &TX_CHDATA {
        self.tx_chdata(2)
    }
    #[doc = "0x0c - The read and write data register for CHANNEL3 by apb fifo access."]
    #[inline(always)]
    pub const fn tx_ch3data(&self) -> &TX_CHDATA {
        self.tx_chdata(3)
    }
    #[doc = "0x10..0x20 - The read and write data register for CHANNEL$n by apb fifo access."]
    #[inline(always)]
    pub const fn rx_chdata(&self, n: usize) -> &RX_CHDATA {
        &self.rx_chdata[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - The read and write data register for CHANNEL$n by apb fifo access."]
    #[inline(always)]
    pub fn rx_chdata_iter(&self) -> impl Iterator<Item = &RX_CHDATA> {
        self.rx_chdata.iter()
    }
    #[doc = "0x10 - The read and write data register for CHANNEL$n by apb fifo access."]
    #[inline(always)]
    pub const fn rx_ch0data(&self) -> &RX_CHDATA {
        self.rx_chdata(0)
    }
    #[doc = "0x14 - The read and write data register for CHANNEL$n by apb fifo access."]
    #[inline(always)]
    pub const fn rx_ch1data(&self) -> &RX_CHDATA {
        self.rx_chdata(1)
    }
    #[doc = "0x18 - The read and write data register for CHANNEL$n by apb fifo access."]
    #[inline(always)]
    pub const fn rx_ch2data(&self) -> &RX_CHDATA {
        self.rx_chdata(2)
    }
    #[doc = "0x1c - The read and write data register for CHANNEL$n by apb fifo access."]
    #[inline(always)]
    pub const fn rx_ch3data(&self) -> &RX_CHDATA {
        self.rx_chdata(3)
    }
    #[doc = "0x20..0x30 - Channel %s configure register 0"]
    #[inline(always)]
    pub const fn tx_chconf0(&self, n: usize) -> &TX_CHCONF0 {
        &self.tx_chconf0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Channel %s configure register 0"]
    #[inline(always)]
    pub fn tx_chconf0_iter(&self) -> impl Iterator<Item = &TX_CHCONF0> {
        self.tx_chconf0.iter()
    }
    #[doc = "0x20 - Channel 0 configure register 0"]
    #[inline(always)]
    pub const fn tx_ch0conf0(&self) -> &TX_CHCONF0 {
        self.tx_chconf0(0)
    }
    #[doc = "0x24 - Channel 1 configure register 0"]
    #[inline(always)]
    pub const fn tx_ch1conf0(&self) -> &TX_CHCONF0 {
        self.tx_chconf0(1)
    }
    #[doc = "0x28 - Channel 2 configure register 0"]
    #[inline(always)]
    pub const fn tx_ch2conf0(&self) -> &TX_CHCONF0 {
        self.tx_chconf0(2)
    }
    #[doc = "0x2c - Channel 3 configure register 0"]
    #[inline(always)]
    pub const fn tx_ch3conf0(&self) -> &TX_CHCONF0 {
        self.tx_chconf0(3)
    }
    #[doc = "0x30..0x40 - Channel %s configure register 0"]
    #[inline(always)]
    pub const fn rx_chconf0(&self, n: usize) -> &RX_CHCONF0 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(48).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Channel %s configure register 0"]
    #[inline(always)]
    pub fn rx_chconf0_iter(&self) -> impl Iterator<Item = &RX_CHCONF0> {
        (0..4)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(48).add(8 * n).cast() })
    }
    #[doc = "0x30 - Channel 0 configure register 0"]
    #[inline(always)]
    pub const fn rx_ch0conf0(&self) -> &RX_CHCONF0 {
        self.rx_chconf0(0)
    }
    #[doc = "0x38 - Channel 1 configure register 0"]
    #[inline(always)]
    pub const fn rx_ch1conf0(&self) -> &RX_CHCONF0 {
        self.rx_chconf0(1)
    }
    #[doc = "0x40 - Channel 2 configure register 0"]
    #[inline(always)]
    pub const fn rx_ch2conf0(&self) -> &RX_CHCONF0 {
        self.rx_chconf0(2)
    }
    #[doc = "0x48 - Channel 3 configure register 0"]
    #[inline(always)]
    pub const fn rx_ch3conf0(&self) -> &RX_CHCONF0 {
        self.rx_chconf0(3)
    }
    #[doc = "0x34..0x44 - Channel %s configure register 1"]
    #[inline(always)]
    pub const fn rx_chconf1(&self, n: usize) -> &RX_CHCONF1 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(52).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x44 - Channel %s configure register 1"]
    #[inline(always)]
    pub fn rx_chconf1_iter(&self) -> impl Iterator<Item = &RX_CHCONF1> {
        (0..4)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(52).add(8 * n).cast() })
    }
    #[doc = "0x34 - Channel 0 configure register 1"]
    #[inline(always)]
    pub const fn rx_ch0conf1(&self) -> &RX_CHCONF1 {
        self.rx_chconf1(0)
    }
    #[doc = "0x3c - Channel 1 configure register 1"]
    #[inline(always)]
    pub const fn rx_ch1conf1(&self) -> &RX_CHCONF1 {
        self.rx_chconf1(1)
    }
    #[doc = "0x44 - Channel 2 configure register 1"]
    #[inline(always)]
    pub const fn rx_ch2conf1(&self) -> &RX_CHCONF1 {
        self.rx_chconf1(2)
    }
    #[doc = "0x4c - Channel 3 configure register 1"]
    #[inline(always)]
    pub const fn rx_ch3conf1(&self) -> &RX_CHCONF1 {
        self.rx_chconf1(3)
    }
    #[doc = "0x50..0x60 - Channel %s status register"]
    #[inline(always)]
    pub const fn tx_chstatus(&self, n: usize) -> &TX_CHSTATUS {
        &self.tx_chstatus[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - Channel %s status register"]
    #[inline(always)]
    pub fn tx_chstatus_iter(&self) -> impl Iterator<Item = &TX_CHSTATUS> {
        self.tx_chstatus.iter()
    }
    #[doc = "0x50 - Channel 0 status register"]
    #[inline(always)]
    pub const fn tx_ch0status(&self) -> &TX_CHSTATUS {
        self.tx_chstatus(0)
    }
    #[doc = "0x54 - Channel 1 status register"]
    #[inline(always)]
    pub const fn tx_ch1status(&self) -> &TX_CHSTATUS {
        self.tx_chstatus(1)
    }
    #[doc = "0x58 - Channel 2 status register"]
    #[inline(always)]
    pub const fn tx_ch2status(&self) -> &TX_CHSTATUS {
        self.tx_chstatus(2)
    }
    #[doc = "0x5c - Channel 3 status register"]
    #[inline(always)]
    pub const fn tx_ch3status(&self) -> &TX_CHSTATUS {
        self.tx_chstatus(3)
    }
    #[doc = "0x60..0x70 - Channel %s status register"]
    #[inline(always)]
    pub const fn rx_chstatus(&self, n: usize) -> &RX_CHSTATUS {
        &self.rx_chstatus[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - Channel %s status register"]
    #[inline(always)]
    pub fn rx_chstatus_iter(&self) -> impl Iterator<Item = &RX_CHSTATUS> {
        self.rx_chstatus.iter()
    }
    #[doc = "0x60 - Channel 0 status register"]
    #[inline(always)]
    pub const fn rx_ch0status(&self) -> &RX_CHSTATUS {
        self.rx_chstatus(0)
    }
    #[doc = "0x64 - Channel 1 status register"]
    #[inline(always)]
    pub const fn rx_ch1status(&self) -> &RX_CHSTATUS {
        self.rx_chstatus(1)
    }
    #[doc = "0x68 - Channel 2 status register"]
    #[inline(always)]
    pub const fn rx_ch2status(&self) -> &RX_CHSTATUS {
        self.rx_chstatus(2)
    }
    #[doc = "0x6c - Channel 3 status register"]
    #[inline(always)]
    pub const fn rx_ch3status(&self) -> &RX_CHSTATUS {
        self.rx_chstatus(3)
    }
    #[doc = "0x70 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x74 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x78 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x7c - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x80..0x90 - Channel %s duty cycle configuration register"]
    #[inline(always)]
    pub const fn chcarrier_duty(&self, n: usize) -> &CHCARRIER_DUTY {
        &self.chcarrier_duty[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - Channel %s duty cycle configuration register"]
    #[inline(always)]
    pub fn chcarrier_duty_iter(&self) -> impl Iterator<Item = &CHCARRIER_DUTY> {
        self.chcarrier_duty.iter()
    }
    #[doc = "0x80 - Channel 0 duty cycle configuration register"]
    #[inline(always)]
    pub const fn ch0carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(0)
    }
    #[doc = "0x84 - Channel 1 duty cycle configuration register"]
    #[inline(always)]
    pub const fn ch1carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(1)
    }
    #[doc = "0x88 - Channel 2 duty cycle configuration register"]
    #[inline(always)]
    pub const fn ch2carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(2)
    }
    #[doc = "0x8c - Channel 3 duty cycle configuration register"]
    #[inline(always)]
    pub const fn ch3carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(3)
    }
    #[doc = "0x90..0xa0 - Channel %s carrier remove register"]
    #[inline(always)]
    pub const fn ch_rx_carrier_rm(&self, n: usize) -> &CH_RX_CARRIER_RM {
        &self.ch_rx_carrier_rm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xa0 - Channel %s carrier remove register"]
    #[inline(always)]
    pub fn ch_rx_carrier_rm_iter(&self) -> impl Iterator<Item = &CH_RX_CARRIER_RM> {
        self.ch_rx_carrier_rm.iter()
    }
    #[doc = "0x90 - Channel 0 carrier remove register"]
    #[inline(always)]
    pub const fn ch0_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        self.ch_rx_carrier_rm(0)
    }
    #[doc = "0x94 - Channel 1 carrier remove register"]
    #[inline(always)]
    pub const fn ch1_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        self.ch_rx_carrier_rm(1)
    }
    #[doc = "0x98 - Channel 2 carrier remove register"]
    #[inline(always)]
    pub const fn ch2_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        self.ch_rx_carrier_rm(2)
    }
    #[doc = "0x9c - Channel 3 carrier remove register"]
    #[inline(always)]
    pub const fn ch3_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        self.ch_rx_carrier_rm(3)
    }
    #[doc = "0xa0..0xb0 - Channel %s Tx event configuration register"]
    #[inline(always)]
    pub const fn ch_tx_lim(&self, n: usize) -> &CH_TX_LIM {
        &self.ch_tx_lim[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xb0 - Channel %s Tx event configuration register"]
    #[inline(always)]
    pub fn ch_tx_lim_iter(&self) -> impl Iterator<Item = &CH_TX_LIM> {
        self.ch_tx_lim.iter()
    }
    #[doc = "0xa0 - Channel 0 Tx event configuration register"]
    #[inline(always)]
    pub const fn ch0_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(0)
    }
    #[doc = "0xa4 - Channel 1 Tx event configuration register"]
    #[inline(always)]
    pub const fn ch1_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(1)
    }
    #[doc = "0xa8 - Channel 2 Tx event configuration register"]
    #[inline(always)]
    pub const fn ch2_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(2)
    }
    #[doc = "0xac - Channel 3 Tx event configuration register"]
    #[inline(always)]
    pub const fn ch3_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(3)
    }
    #[doc = "0xb0..0xc0 - Channel %s Rx event configuration register"]
    #[inline(always)]
    pub const fn ch_rx_lim(&self, n: usize) -> &CH_RX_LIM {
        &self.ch_rx_lim[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0xc0 - Channel %s Rx event configuration register"]
    #[inline(always)]
    pub fn ch_rx_lim_iter(&self) -> impl Iterator<Item = &CH_RX_LIM> {
        self.ch_rx_lim.iter()
    }
    #[doc = "0xb0 - Channel 0 Rx event configuration register"]
    #[inline(always)]
    pub const fn ch0_rx_lim(&self) -> &CH_RX_LIM {
        self.ch_rx_lim(0)
    }
    #[doc = "0xb4 - Channel 1 Rx event configuration register"]
    #[inline(always)]
    pub const fn ch1_rx_lim(&self) -> &CH_RX_LIM {
        self.ch_rx_lim(1)
    }
    #[doc = "0xb8 - Channel 2 Rx event configuration register"]
    #[inline(always)]
    pub const fn ch2_rx_lim(&self) -> &CH_RX_LIM {
        self.ch_rx_lim(2)
    }
    #[doc = "0xbc - Channel 3 Rx event configuration register"]
    #[inline(always)]
    pub const fn ch3_rx_lim(&self) -> &CH_RX_LIM {
        self.ch_rx_lim(3)
    }
    #[doc = "0xc0 - RMT apb configuration register"]
    #[inline(always)]
    pub const fn sys_conf(&self) -> &SYS_CONF {
        &self.sys_conf
    }
    #[doc = "0xc4 - RMT TX synchronous register"]
    #[inline(always)]
    pub const fn tx_sim(&self) -> &TX_SIM {
        &self.tx_sim
    }
    #[doc = "0xc8 - RMT clock divider reset register"]
    #[inline(always)]
    pub const fn ref_cnt_rst(&self) -> &REF_CNT_RST {
        &self.ref_cnt_rst
    }
    #[doc = "0xcc - RMT version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "TX_CHDATA (r) register accessor: The read and write data register for CHANNEL%s by apb fifo access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_chdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_chdata`] module"]
pub type TX_CHDATA = crate::Reg<tx_chdata::TX_CHDATA_SPEC>;
#[doc = "The read and write data register for CHANNEL%s by apb fifo access."]
pub mod tx_chdata;
#[doc = "RX_CHDATA (r) register accessor: The read and write data register for CHANNEL$n by apb fifo access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_chdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_chdata`] module"]
pub type RX_CHDATA = crate::Reg<rx_chdata::RX_CHDATA_SPEC>;
#[doc = "The read and write data register for CHANNEL$n by apb fifo access."]
pub mod rx_chdata;
#[doc = "TX_CHCONF0 (rw) register accessor: Channel %s configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_chconf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_chconf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_chconf0`] module"]
pub type TX_CHCONF0 = crate::Reg<tx_chconf0::TX_CHCONF0_SPEC>;
#[doc = "Channel %s configure register 0"]
pub mod tx_chconf0;
#[doc = "RX_CHCONF0 (rw) register accessor: Channel %s configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_chconf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_chconf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_chconf0`] module"]
pub type RX_CHCONF0 = crate::Reg<rx_chconf0::RX_CHCONF0_SPEC>;
#[doc = "Channel %s configure register 0"]
pub mod rx_chconf0;
#[doc = "RX_CHCONF1 (rw) register accessor: Channel %s configure register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_chconf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_chconf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_chconf1`] module"]
pub type RX_CHCONF1 = crate::Reg<rx_chconf1::RX_CHCONF1_SPEC>;
#[doc = "Channel %s configure register 1"]
pub mod rx_chconf1;
#[doc = "TX_CHSTATUS (r) register accessor: Channel %s status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_chstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_chstatus`] module"]
pub type TX_CHSTATUS = crate::Reg<tx_chstatus::TX_CHSTATUS_SPEC>;
#[doc = "Channel %s status register"]
pub mod tx_chstatus;
#[doc = "RX_CHSTATUS (r) register accessor: Channel %s status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_chstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_chstatus`] module"]
pub type RX_CHSTATUS = crate::Reg<rx_chstatus::RX_CHSTATUS_SPEC>;
#[doc = "Channel %s status register"]
pub mod rx_chstatus;
#[doc = "INT_RAW (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CHCARRIER_DUTY (rw) register accessor: Channel %s duty cycle configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcarrier_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcarrier_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chcarrier_duty`] module"]
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
#[doc = "Channel %s duty cycle configuration register"]
pub mod chcarrier_duty;
#[doc = "CH_RX_CARRIER_RM (rw) register accessor: Channel %s carrier remove register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_carrier_rm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_carrier_rm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_rx_carrier_rm`] module"]
pub type CH_RX_CARRIER_RM = crate::Reg<ch_rx_carrier_rm::CH_RX_CARRIER_RM_SPEC>;
#[doc = "Channel %s carrier remove register"]
pub mod ch_rx_carrier_rm;
#[doc = "CH_TX_LIM (rw) register accessor: Channel %s Tx event configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_tx_lim`] module"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = "Channel %s Tx event configuration register"]
pub mod ch_tx_lim;
#[doc = "CH_RX_LIM (rw) register accessor: Channel %s Rx event configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_rx_lim`] module"]
pub type CH_RX_LIM = crate::Reg<ch_rx_lim::CH_RX_LIM_SPEC>;
#[doc = "Channel %s Rx event configuration register"]
pub mod ch_rx_lim;
#[doc = "SYS_CONF (rw) register accessor: RMT apb configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_conf`] module"]
pub type SYS_CONF = crate::Reg<sys_conf::SYS_CONF_SPEC>;
#[doc = "RMT apb configuration register"]
pub mod sys_conf;
#[doc = "TX_SIM (rw) register accessor: RMT TX synchronous register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_sim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_sim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_sim`] module"]
pub type TX_SIM = crate::Reg<tx_sim::TX_SIM_SPEC>;
#[doc = "RMT TX synchronous register"]
pub mod tx_sim;
#[doc = "REF_CNT_RST (w) register accessor: RMT clock divider reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_cnt_rst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_cnt_rst`] module"]
pub type REF_CNT_RST = crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>;
#[doc = "RMT clock divider reset register"]
pub mod ref_cnt_rst;
#[doc = "DATE (rw) register accessor: RMT version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RMT version register"]
pub mod date;
