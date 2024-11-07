#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    chdata: [CHDATA; 4],
    chconf0: (),
    _reserved2: [u8; 0x04],
    chconf1: (),
    _reserved3: [u8; 0x1c],
    chstatus: [CHSTATUS; 4],
    chaddr: [CHADDR; 4],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    chcarrier_duty: [CHCARRIER_DUTY; 4],
    ch_tx_lim: [CH_TX_LIM; 4],
    apb_conf: APB_CONF,
    tx_sim: TX_SIM,
    ref_cnt_rst: REF_CNT_RST,
    ch_rx_carrier_rm: [CH_RX_CARRIER_RM; 4],
    _reserved15: [u8; 0x60],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - The read and write data register for CHANNEL%s by apb fifo access."]
    #[inline(always)]
    pub const fn chdata(&self, n: usize) -> &CHDATA {
        &self.chdata[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - The read and write data register for CHANNEL%s by apb fifo access."]
    #[inline(always)]
    pub fn chdata_iter(&self) -> impl Iterator<Item = &CHDATA> {
        self.chdata.iter()
    }
    #[doc = "0x00 - The read and write data register for CHANNEL0 by apb fifo access."]
    #[inline(always)]
    pub const fn ch0data(&self) -> &CHDATA {
        self.chdata(0)
    }
    #[doc = "0x04 - The read and write data register for CHANNEL1 by apb fifo access."]
    #[inline(always)]
    pub const fn ch1data(&self) -> &CHDATA {
        self.chdata(1)
    }
    #[doc = "0x08 - The read and write data register for CHANNEL2 by apb fifo access."]
    #[inline(always)]
    pub const fn ch2data(&self) -> &CHDATA {
        self.chdata(2)
    }
    #[doc = "0x0c - The read and write data register for CHANNEL3 by apb fifo access."]
    #[inline(always)]
    pub const fn ch3data(&self) -> &CHDATA {
        self.chdata(3)
    }
    #[doc = "0x10..0x20 - Channel %s configure register 0"]
    #[inline(always)]
    pub const fn chconf0(&self, n: usize) -> &CHCONF0 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Channel %s configure register 0"]
    #[inline(always)]
    pub fn chconf0_iter(&self) -> impl Iterator<Item = &CHCONF0> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x10 - Channel 0 configure register 0"]
    #[inline(always)]
    pub const fn ch0conf0(&self) -> &CHCONF0 {
        self.chconf0(0)
    }
    #[doc = "0x18 - Channel 1 configure register 0"]
    #[inline(always)]
    pub const fn ch1conf0(&self) -> &CHCONF0 {
        self.chconf0(1)
    }
    #[doc = "0x20 - Channel 2 configure register 0"]
    #[inline(always)]
    pub const fn ch2conf0(&self) -> &CHCONF0 {
        self.chconf0(2)
    }
    #[doc = "0x28 - Channel 3 configure register 0"]
    #[inline(always)]
    pub const fn ch3conf0(&self) -> &CHCONF0 {
        self.chconf0(3)
    }
    #[doc = "0x14..0x24 - Channel %s configure register 1"]
    #[inline(always)]
    pub const fn chconf1(&self, n: usize) -> &CHCONF1 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(20)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x24 - Channel %s configure register 1"]
    #[inline(always)]
    pub fn chconf1_iter(&self) -> impl Iterator<Item = &CHCONF1> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(20)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x14 - Channel 0 configure register 1"]
    #[inline(always)]
    pub const fn ch0conf1(&self) -> &CHCONF1 {
        self.chconf1(0)
    }
    #[doc = "0x1c - Channel 1 configure register 1"]
    #[inline(always)]
    pub const fn ch1conf1(&self) -> &CHCONF1 {
        self.chconf1(1)
    }
    #[doc = "0x24 - Channel 2 configure register 1"]
    #[inline(always)]
    pub const fn ch2conf1(&self) -> &CHCONF1 {
        self.chconf1(2)
    }
    #[doc = "0x2c - Channel 3 configure register 1"]
    #[inline(always)]
    pub const fn ch3conf1(&self) -> &CHCONF1 {
        self.chconf1(3)
    }
    #[doc = "0x30..0x40 - Channel %s status register"]
    #[inline(always)]
    pub const fn chstatus(&self, n: usize) -> &CHSTATUS {
        &self.chstatus[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Channel %s status register"]
    #[inline(always)]
    pub fn chstatus_iter(&self) -> impl Iterator<Item = &CHSTATUS> {
        self.chstatus.iter()
    }
    #[doc = "0x30 - Channel 0 status register"]
    #[inline(always)]
    pub const fn ch0status(&self) -> &CHSTATUS {
        self.chstatus(0)
    }
    #[doc = "0x34 - Channel 1 status register"]
    #[inline(always)]
    pub const fn ch1status(&self) -> &CHSTATUS {
        self.chstatus(1)
    }
    #[doc = "0x38 - Channel 2 status register"]
    #[inline(always)]
    pub const fn ch2status(&self) -> &CHSTATUS {
        self.chstatus(2)
    }
    #[doc = "0x3c - Channel 3 status register"]
    #[inline(always)]
    pub const fn ch3status(&self) -> &CHSTATUS {
        self.chstatus(3)
    }
    #[doc = "0x40..0x50 - Channel %s address register"]
    #[inline(always)]
    pub const fn chaddr(&self, n: usize) -> &CHADDR {
        &self.chaddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Channel %s address register"]
    #[inline(always)]
    pub fn chaddr_iter(&self) -> impl Iterator<Item = &CHADDR> {
        self.chaddr.iter()
    }
    #[doc = "0x40 - Channel 0 address register"]
    #[inline(always)]
    pub const fn ch0addr(&self) -> &CHADDR {
        self.chaddr(0)
    }
    #[doc = "0x44 - Channel 1 address register"]
    #[inline(always)]
    pub const fn ch1addr(&self) -> &CHADDR {
        self.chaddr(1)
    }
    #[doc = "0x48 - Channel 2 address register"]
    #[inline(always)]
    pub const fn ch2addr(&self) -> &CHADDR {
        self.chaddr(2)
    }
    #[doc = "0x4c - Channel 3 address register"]
    #[inline(always)]
    pub const fn ch3addr(&self) -> &CHADDR {
        self.chaddr(3)
    }
    #[doc = "0x50 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x54 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x58 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x5c - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x60..0x70 - Channel %s duty cycle configuration register"]
    #[inline(always)]
    pub const fn chcarrier_duty(&self, n: usize) -> &CHCARRIER_DUTY {
        &self.chcarrier_duty[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - Channel %s duty cycle configuration register"]
    #[inline(always)]
    pub fn chcarrier_duty_iter(&self) -> impl Iterator<Item = &CHCARRIER_DUTY> {
        self.chcarrier_duty.iter()
    }
    #[doc = "0x60 - Channel 0 duty cycle configuration register"]
    #[inline(always)]
    pub const fn ch0carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(0)
    }
    #[doc = "0x64 - Channel 1 duty cycle configuration register"]
    #[inline(always)]
    pub const fn ch1carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(1)
    }
    #[doc = "0x68 - Channel 2 duty cycle configuration register"]
    #[inline(always)]
    pub const fn ch2carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(2)
    }
    #[doc = "0x6c - Channel 3 duty cycle configuration register"]
    #[inline(always)]
    pub const fn ch3carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(3)
    }
    #[doc = "0x70..0x80 - Channel %s Tx event configuration register"]
    #[inline(always)]
    pub const fn ch_tx_lim(&self, n: usize) -> &CH_TX_LIM {
        &self.ch_tx_lim[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - Channel %s Tx event configuration register"]
    #[inline(always)]
    pub fn ch_tx_lim_iter(&self) -> impl Iterator<Item = &CH_TX_LIM> {
        self.ch_tx_lim.iter()
    }
    #[doc = "0x70 - Channel 0 Tx event configuration register"]
    #[inline(always)]
    pub const fn ch0_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(0)
    }
    #[doc = "0x74 - Channel 1 Tx event configuration register"]
    #[inline(always)]
    pub const fn ch1_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(1)
    }
    #[doc = "0x78 - Channel 2 Tx event configuration register"]
    #[inline(always)]
    pub const fn ch2_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(2)
    }
    #[doc = "0x7c - Channel 3 Tx event configuration register"]
    #[inline(always)]
    pub const fn ch3_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(3)
    }
    #[doc = "0x80 - RMT apb configuration register"]
    #[inline(always)]
    pub const fn apb_conf(&self) -> &APB_CONF {
        &self.apb_conf
    }
    #[doc = "0x84 - RMT TX synchronous register"]
    #[inline(always)]
    pub const fn tx_sim(&self) -> &TX_SIM {
        &self.tx_sim
    }
    #[doc = "0x88 - RMT clock divider reset register"]
    #[inline(always)]
    pub const fn ref_cnt_rst(&self) -> &REF_CNT_RST {
        &self.ref_cnt_rst
    }
    #[doc = "0x8c..0x9c - Channel %s carrier remove register"]
    #[inline(always)]
    pub const fn ch_rx_carrier_rm(&self, n: usize) -> &CH_RX_CARRIER_RM {
        &self.ch_rx_carrier_rm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8c..0x9c - Channel %s carrier remove register"]
    #[inline(always)]
    pub fn ch_rx_carrier_rm_iter(&self) -> impl Iterator<Item = &CH_RX_CARRIER_RM> {
        self.ch_rx_carrier_rm.iter()
    }
    #[doc = "0x8c - Channel 0 carrier remove register"]
    #[inline(always)]
    pub const fn ch0_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        self.ch_rx_carrier_rm(0)
    }
    #[doc = "0x90 - Channel 1 carrier remove register"]
    #[inline(always)]
    pub const fn ch1_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        self.ch_rx_carrier_rm(1)
    }
    #[doc = "0x94 - Channel 2 carrier remove register"]
    #[inline(always)]
    pub const fn ch2_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        self.ch_rx_carrier_rm(2)
    }
    #[doc = "0x98 - Channel 3 carrier remove register"]
    #[inline(always)]
    pub const fn ch3_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        self.ch_rx_carrier_rm(3)
    }
    #[doc = "0xfc - RMT version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CHDATA (rw) register accessor: The read and write data register for CHANNEL%s by apb fifo access.\n\nYou can [`read`](crate::Reg::read) this register and get [`chdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdata`] module"]
pub type CHDATA = crate::Reg<chdata::CHDATA_SPEC>;
#[doc = "The read and write data register for CHANNEL%s by apb fifo access."]
pub mod chdata;
#[doc = "CHCONF0 (rw) register accessor: Channel %s configure register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chconf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chconf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chconf0`] module"]
pub type CHCONF0 = crate::Reg<chconf0::CHCONF0_SPEC>;
#[doc = "Channel %s configure register 0"]
pub mod chconf0;
#[doc = "CHCONF1 (rw) register accessor: Channel %s configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chconf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chconf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chconf1`] module"]
pub type CHCONF1 = crate::Reg<chconf1::CHCONF1_SPEC>;
#[doc = "Channel %s configure register 1"]
pub mod chconf1;
#[doc = "CHSTATUS (r) register accessor: Channel %s status register\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`] module"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel %s status register"]
pub mod chstatus;
#[doc = "CHADDR (r) register accessor: Channel %s address register\n\nYou can [`read`](crate::Reg::read) this register and get [`chaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chaddr`] module"]
pub type CHADDR = crate::Reg<chaddr::CHADDR_SPEC>;
#[doc = "Channel %s address register"]
pub mod chaddr;
#[doc = "INT_RAW (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CHCARRIER_DUTY (rw) register accessor: Channel %s duty cycle configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`chcarrier_duty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chcarrier_duty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chcarrier_duty`] module"]
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
#[doc = "Channel %s duty cycle configuration register"]
pub mod chcarrier_duty;
#[doc = "CH_TX_LIM (rw) register accessor: Channel %s Tx event configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_tx_lim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_tx_lim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_tx_lim`] module"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = "Channel %s Tx event configuration register"]
pub mod ch_tx_lim;
#[doc = "APB_CONF (rw) register accessor: RMT apb configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_conf`] module"]
pub type APB_CONF = crate::Reg<apb_conf::APB_CONF_SPEC>;
#[doc = "RMT apb configuration register"]
pub mod apb_conf;
#[doc = "TX_SIM (rw) register accessor: RMT TX synchronous register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_sim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_sim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_sim`] module"]
pub type TX_SIM = crate::Reg<tx_sim::TX_SIM_SPEC>;
#[doc = "RMT TX synchronous register"]
pub mod tx_sim;
#[doc = "REF_CNT_RST (rw) register accessor: RMT clock divider reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_cnt_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_cnt_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_cnt_rst`] module"]
pub type REF_CNT_RST = crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>;
#[doc = "RMT clock divider reset register"]
pub mod ref_cnt_rst;
#[doc = "CH_RX_CARRIER_RM (rw) register accessor: Channel %s carrier remove register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_rx_carrier_rm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_rx_carrier_rm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_rx_carrier_rm`] module"]
pub type CH_RX_CARRIER_RM = crate::Reg<ch_rx_carrier_rm::CH_RX_CARRIER_RM_SPEC>;
#[doc = "Channel %s carrier remove register"]
pub mod ch_rx_carrier_rm;
#[doc = "DATE (rw) register accessor: RMT version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RMT version register"]
pub mod date;
