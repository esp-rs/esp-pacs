#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    region_filter_en: REGION_FILTER_EN,
    region_addr_start: (),
    _reserved2: [u8; 0x04],
    region_addr_end: (),
    _reserved3: [u8; 0x04],
    region_attr: (),
    _reserved4: [u8; 0xb0],
    func_ctrl: FUNC_CTRL,
    _reserved_5_m: [u8; 0x50],
    bus_err_conf: BUS_ERR_CONF,
    _reserved7: [u8; 0x02e4],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Region filter enable register"]
    #[inline(always)]
    pub const fn region_filter_en(&self) -> &REGION_FILTER_EN {
        &self.region_filter_en
    }
    #[doc = "0x04..0x24 - Region address register"]
    #[inline(always)]
    pub const fn region_addr_start(&self, n: usize) -> &REGION_ADDR_START {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x24 - Region address register"]
    #[inline(always)]
    pub fn region_addr_start_iter(&self) -> impl Iterator<Item = &REGION_ADDR_START> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(12 * n)
                .cast()
        })
    }
    #[doc = "0x04 - Region address register"]
    #[inline(always)]
    pub const fn region0_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(0)
    }
    #[doc = "0x10 - Region address register"]
    #[inline(always)]
    pub const fn region1_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(1)
    }
    #[doc = "0x1c - Region address register"]
    #[inline(always)]
    pub const fn region2_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(2)
    }
    #[doc = "0x28 - Region address register"]
    #[inline(always)]
    pub const fn region3_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(3)
    }
    #[doc = "0x34 - Region address register"]
    #[inline(always)]
    pub const fn region4_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(4)
    }
    #[doc = "0x40 - Region address register"]
    #[inline(always)]
    pub const fn region5_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(5)
    }
    #[doc = "0x4c - Region address register"]
    #[inline(always)]
    pub const fn region6_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(6)
    }
    #[doc = "0x58 - Region address register"]
    #[inline(always)]
    pub const fn region7_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(7)
    }
    #[doc = "0x08..0x28 - Region address register"]
    #[inline(always)]
    pub const fn region_addr_end(&self, n: usize) -> &REGION_ADDR_END {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x28 - Region address register"]
    #[inline(always)]
    pub fn region_addr_end_iter(&self) -> impl Iterator<Item = &REGION_ADDR_END> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(12 * n)
                .cast()
        })
    }
    #[doc = "0x08 - Region address register"]
    #[inline(always)]
    pub const fn region0_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(0)
    }
    #[doc = "0x14 - Region address register"]
    #[inline(always)]
    pub const fn region1_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(1)
    }
    #[doc = "0x20 - Region address register"]
    #[inline(always)]
    pub const fn region2_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(2)
    }
    #[doc = "0x2c - Region address register"]
    #[inline(always)]
    pub const fn region3_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(3)
    }
    #[doc = "0x38 - Region address register"]
    #[inline(always)]
    pub const fn region4_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(4)
    }
    #[doc = "0x44 - Region address register"]
    #[inline(always)]
    pub const fn region5_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(5)
    }
    #[doc = "0x50 - Region address register"]
    #[inline(always)]
    pub const fn region6_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(6)
    }
    #[doc = "0x5c - Region address register"]
    #[inline(always)]
    pub const fn region7_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(7)
    }
    #[doc = "0x0c..0x2c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region_attr(&self, n: usize) -> &REGION_ATTR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x2c - Region access authority attribute register"]
    #[inline(always)]
    pub fn region_attr_iter(&self) -> impl Iterator<Item = &REGION_ATTR> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(12 * n)
                .cast()
        })
    }
    #[doc = "0x0c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region0_attr(&self) -> &REGION_ATTR {
        self.region_attr(0)
    }
    #[doc = "0x18 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region1_attr(&self) -> &REGION_ATTR {
        self.region_attr(1)
    }
    #[doc = "0x24 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region2_attr(&self) -> &REGION_ATTR {
        self.region_attr(2)
    }
    #[doc = "0x30 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region3_attr(&self) -> &REGION_ATTR {
        self.region_attr(3)
    }
    #[doc = "0x3c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region4_attr(&self) -> &REGION_ATTR {
        self.region_attr(4)
    }
    #[doc = "0x48 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region5_attr(&self) -> &REGION_ATTR {
        self.region_attr(5)
    }
    #[doc = "0x54 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region6_attr(&self) -> &REGION_ATTR {
        self.region_attr(6)
    }
    #[doc = "0x60 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region7_attr(&self) -> &REGION_ATTR {
        self.region_attr(7)
    }
    #[doc = "0xbc - APM function control register"]
    #[inline(always)]
    pub const fn func_ctrl(&self) -> &FUNC_CTRL {
        &self.func_ctrl
    }
    #[doc = "0xc0..0x110 - Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1"]
    #[inline(always)]
    pub const fn m(&self, n: usize) -> &M {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(192)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0x110 - Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1"]
    #[inline(always)]
    pub fn m_iter(&self) -> impl Iterator<Item = &M> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(192)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x10c - APM interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x110 - APM interrupt enable register"]
    #[inline(always)]
    pub const fn bus_err_conf(&self) -> &BUS_ERR_CONF {
        &self.bus_err_conf
    }
    #[doc = "0x3f8 - Clock gating register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "REGION_FILTER_EN (rw) register accessor: Region filter enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_filter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_filter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region_filter_en`] module"]
pub type REGION_FILTER_EN = crate::Reg<region_filter_en::REGION_FILTER_EN_SPEC>;
#[doc = "Region filter enable register"]
pub mod region_filter_en;
#[doc = "REGION_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_addr_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_addr_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region_addr_start`] module"]
pub type REGION_ADDR_START = crate::Reg<region_addr_start::REGION_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region_addr_start;
#[doc = "REGION_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_addr_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_addr_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region_addr_end`] module"]
pub type REGION_ADDR_END = crate::Reg<region_addr_end::REGION_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region_addr_end;
#[doc = "REGION_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region_attr`] module"]
pub type REGION_ATTR = crate::Reg<region_attr::REGION_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region_attr;
#[doc = "FUNC_CTRL (rw) register accessor: APM function control register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_ctrl`] module"]
pub type FUNC_CTRL = crate::Reg<func_ctrl::FUNC_CTRL_SPEC>;
#[doc = "APM function control register"]
pub mod func_ctrl;
#[doc = "Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1"]
pub use self::m::M;
#[doc = r"Cluster"]
#[doc = "Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1"]
pub mod m;
#[doc = "INT_EN (rw) register accessor: APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "APM interrupt enable register"]
pub mod int_en;
#[doc = "BUS_ERR_CONF (rw) register accessor: APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_err_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_err_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_err_conf`] module"]
pub type BUS_ERR_CONF = crate::Reg<bus_err_conf::BUS_ERR_CONF_SPEC>;
#[doc = "APM interrupt enable register"]
pub mod bus_err_conf;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
