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
    _reserved4: [u8; 0xb8],
    func_ctrl: FUNC_CTRL,
    m0_status: M0_STATUS,
    m0_status_clr: M0_STATUS_CLR,
    m0_exception_info0: M0_EXCEPTION_INFO0,
    m0_exception_info1: M0_EXCEPTION_INFO1,
    m1_status: M1_STATUS,
    m1_status_clr: M1_STATUS_CLR,
    m1_exception_info0: M1_EXCEPTION_INFO0,
    m1_exception_info1: M1_EXCEPTION_INFO1,
    m2_status: M2_STATUS,
    m2_status_clr: M2_STATUS_CLR,
    m2_exception_info0: M2_EXCEPTION_INFO0,
    m2_exception_info1: M2_EXCEPTION_INFO1,
    m3_status: M3_STATUS,
    m3_status_clr: M3_STATUS_CLR,
    m3_exception_info0: M3_EXCEPTION_INFO0,
    m3_exception_info1: M3_EXCEPTION_INFO1,
    m4_status: M4_STATUS,
    m4_status_clr: M4_STATUS_CLR,
    m4_exception_info0: M4_EXCEPTION_INFO0,
    m4_exception_info1: M4_EXCEPTION_INFO1,
    int_en: INT_EN,
    _reserved26: [u8; 0x06dc],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Region filter enable register"]
    #[inline(always)]
    pub const fn region_filter_en(&self) -> &REGION_FILTER_EN {
        &self.region_filter_en
    }
    #[doc = "0x04..0x44 - Region address register"]
    #[inline(always)]
    pub const fn region_addr_start(&self, n: usize) -> &REGION_ADDR_START {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x44 - Region address register"]
    #[inline(always)]
    pub fn region_addr_start_iter(&self) -> impl Iterator<Item = &REGION_ADDR_START> {
        (0..16).map(move |n| unsafe {
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
    #[doc = "0x64 - Region address register"]
    #[inline(always)]
    pub const fn region8_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(8)
    }
    #[doc = "0x70 - Region address register"]
    #[inline(always)]
    pub const fn region9_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(9)
    }
    #[doc = "0x7c - Region address register"]
    #[inline(always)]
    pub const fn region10_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(10)
    }
    #[doc = "0x88 - Region address register"]
    #[inline(always)]
    pub const fn region11_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(11)
    }
    #[doc = "0x94 - Region address register"]
    #[inline(always)]
    pub const fn region12_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(12)
    }
    #[doc = "0xa0 - Region address register"]
    #[inline(always)]
    pub const fn region13_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(13)
    }
    #[doc = "0xac - Region address register"]
    #[inline(always)]
    pub const fn region14_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(14)
    }
    #[doc = "0xb8 - Region address register"]
    #[inline(always)]
    pub const fn region15_addr_start(&self) -> &REGION_ADDR_START {
        self.region_addr_start(15)
    }
    #[doc = "0x08..0x48 - Region address register"]
    #[inline(always)]
    pub const fn region_addr_end(&self, n: usize) -> &REGION_ADDR_END {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x48 - Region address register"]
    #[inline(always)]
    pub fn region_addr_end_iter(&self) -> impl Iterator<Item = &REGION_ADDR_END> {
        (0..16).map(move |n| unsafe {
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
    #[doc = "0x68 - Region address register"]
    #[inline(always)]
    pub const fn region8_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(8)
    }
    #[doc = "0x74 - Region address register"]
    #[inline(always)]
    pub const fn region9_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(9)
    }
    #[doc = "0x80 - Region address register"]
    #[inline(always)]
    pub const fn region10_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(10)
    }
    #[doc = "0x8c - Region address register"]
    #[inline(always)]
    pub const fn region11_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(11)
    }
    #[doc = "0x98 - Region address register"]
    #[inline(always)]
    pub const fn region12_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(12)
    }
    #[doc = "0xa4 - Region address register"]
    #[inline(always)]
    pub const fn region13_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(13)
    }
    #[doc = "0xb0 - Region address register"]
    #[inline(always)]
    pub const fn region14_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(14)
    }
    #[doc = "0xbc - Region address register"]
    #[inline(always)]
    pub const fn region15_addr_end(&self) -> &REGION_ADDR_END {
        self.region_addr_end(15)
    }
    #[doc = "0x0c..0x4c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region_attr(&self, n: usize) -> &REGION_ATTR {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x4c - Region access authority attribute register"]
    #[inline(always)]
    pub fn region_attr_iter(&self) -> impl Iterator<Item = &REGION_ATTR> {
        (0..16).map(move |n| unsafe {
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
    #[doc = "0x6c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region8_attr(&self) -> &REGION_ATTR {
        self.region_attr(8)
    }
    #[doc = "0x78 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region9_attr(&self) -> &REGION_ATTR {
        self.region_attr(9)
    }
    #[doc = "0x84 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region10_attr(&self) -> &REGION_ATTR {
        self.region_attr(10)
    }
    #[doc = "0x90 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region11_attr(&self) -> &REGION_ATTR {
        self.region_attr(11)
    }
    #[doc = "0x9c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region12_attr(&self) -> &REGION_ATTR {
        self.region_attr(12)
    }
    #[doc = "0xa8 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region13_attr(&self) -> &REGION_ATTR {
        self.region_attr(13)
    }
    #[doc = "0xb4 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region14_attr(&self) -> &REGION_ATTR {
        self.region_attr(14)
    }
    #[doc = "0xc0 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region15_attr(&self) -> &REGION_ATTR {
        self.region_attr(15)
    }
    #[doc = "0xc4 - APM function control register"]
    #[inline(always)]
    pub const fn func_ctrl(&self) -> &FUNC_CTRL {
        &self.func_ctrl
    }
    #[doc = "0xc8 - M0 status register"]
    #[inline(always)]
    pub const fn m0_status(&self) -> &M0_STATUS {
        &self.m0_status
    }
    #[doc = "0xcc - M0 status clear register"]
    #[inline(always)]
    pub const fn m0_status_clr(&self) -> &M0_STATUS_CLR {
        &self.m0_status_clr
    }
    #[doc = "0xd0 - M0 exception_info0 register"]
    #[inline(always)]
    pub const fn m0_exception_info0(&self) -> &M0_EXCEPTION_INFO0 {
        &self.m0_exception_info0
    }
    #[doc = "0xd4 - M0 exception_info1 register"]
    #[inline(always)]
    pub const fn m0_exception_info1(&self) -> &M0_EXCEPTION_INFO1 {
        &self.m0_exception_info1
    }
    #[doc = "0xd8 - M1 status register"]
    #[inline(always)]
    pub const fn m1_status(&self) -> &M1_STATUS {
        &self.m1_status
    }
    #[doc = "0xdc - M1 status clear register"]
    #[inline(always)]
    pub const fn m1_status_clr(&self) -> &M1_STATUS_CLR {
        &self.m1_status_clr
    }
    #[doc = "0xe0 - M1 exception_info0 register"]
    #[inline(always)]
    pub const fn m1_exception_info0(&self) -> &M1_EXCEPTION_INFO0 {
        &self.m1_exception_info0
    }
    #[doc = "0xe4 - M1 exception_info1 register"]
    #[inline(always)]
    pub const fn m1_exception_info1(&self) -> &M1_EXCEPTION_INFO1 {
        &self.m1_exception_info1
    }
    #[doc = "0xe8 - M2 status register"]
    #[inline(always)]
    pub const fn m2_status(&self) -> &M2_STATUS {
        &self.m2_status
    }
    #[doc = "0xec - M2 status clear register"]
    #[inline(always)]
    pub const fn m2_status_clr(&self) -> &M2_STATUS_CLR {
        &self.m2_status_clr
    }
    #[doc = "0xf0 - M2 exception_info0 register"]
    #[inline(always)]
    pub const fn m2_exception_info0(&self) -> &M2_EXCEPTION_INFO0 {
        &self.m2_exception_info0
    }
    #[doc = "0xf4 - M2 exception_info1 register"]
    #[inline(always)]
    pub const fn m2_exception_info1(&self) -> &M2_EXCEPTION_INFO1 {
        &self.m2_exception_info1
    }
    #[doc = "0xf8 - M3 status register"]
    #[inline(always)]
    pub const fn m3_status(&self) -> &M3_STATUS {
        &self.m3_status
    }
    #[doc = "0xfc - M3 status clear register"]
    #[inline(always)]
    pub const fn m3_status_clr(&self) -> &M3_STATUS_CLR {
        &self.m3_status_clr
    }
    #[doc = "0x100 - M3 exception_info0 register"]
    #[inline(always)]
    pub const fn m3_exception_info0(&self) -> &M3_EXCEPTION_INFO0 {
        &self.m3_exception_info0
    }
    #[doc = "0x104 - M3 exception_info1 register"]
    #[inline(always)]
    pub const fn m3_exception_info1(&self) -> &M3_EXCEPTION_INFO1 {
        &self.m3_exception_info1
    }
    #[doc = "0x108 - M4 status register"]
    #[inline(always)]
    pub const fn m4_status(&self) -> &M4_STATUS {
        &self.m4_status
    }
    #[doc = "0x10c - M4 status clear register"]
    #[inline(always)]
    pub const fn m4_status_clr(&self) -> &M4_STATUS_CLR {
        &self.m4_status_clr
    }
    #[doc = "0x110 - M4 exception_info0 register"]
    #[inline(always)]
    pub const fn m4_exception_info0(&self) -> &M4_EXCEPTION_INFO0 {
        &self.m4_exception_info0
    }
    #[doc = "0x114 - M4 exception_info1 register"]
    #[inline(always)]
    pub const fn m4_exception_info1(&self) -> &M4_EXCEPTION_INFO1 {
        &self.m4_exception_info1
    }
    #[doc = "0x118 - APM interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
    #[doc = "0x7f8 - Clock gating register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x7fc - Version control register"]
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
#[doc = "M0_STATUS (r) register accessor: M0 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_status`] module"]
pub type M0_STATUS = crate::Reg<m0_status::M0_STATUS_SPEC>;
#[doc = "M0 status register"]
pub mod m0_status;
#[doc = "M0_STATUS_CLR (w) register accessor: M0 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_status_clr`] module"]
pub type M0_STATUS_CLR = crate::Reg<m0_status_clr::M0_STATUS_CLR_SPEC>;
#[doc = "M0 status clear register"]
pub mod m0_status_clr;
#[doc = "M0_EXCEPTION_INFO0 (r) register accessor: M0 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0_exception_info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_exception_info0`] module"]
pub type M0_EXCEPTION_INFO0 = crate::Reg<m0_exception_info0::M0_EXCEPTION_INFO0_SPEC>;
#[doc = "M0 exception_info0 register"]
pub mod m0_exception_info0;
#[doc = "M0_EXCEPTION_INFO1 (r) register accessor: M0 exception_info1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0_exception_info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_exception_info1`] module"]
pub type M0_EXCEPTION_INFO1 = crate::Reg<m0_exception_info1::M0_EXCEPTION_INFO1_SPEC>;
#[doc = "M0 exception_info1 register"]
pub mod m0_exception_info1;
#[doc = "M1_STATUS (r) register accessor: M1 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_status`] module"]
pub type M1_STATUS = crate::Reg<m1_status::M1_STATUS_SPEC>;
#[doc = "M1 status register"]
pub mod m1_status;
#[doc = "M1_STATUS_CLR (w) register accessor: M1 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_status_clr`] module"]
pub type M1_STATUS_CLR = crate::Reg<m1_status_clr::M1_STATUS_CLR_SPEC>;
#[doc = "M1 status clear register"]
pub mod m1_status_clr;
#[doc = "M1_EXCEPTION_INFO0 (r) register accessor: M1 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1_exception_info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_exception_info0`] module"]
pub type M1_EXCEPTION_INFO0 = crate::Reg<m1_exception_info0::M1_EXCEPTION_INFO0_SPEC>;
#[doc = "M1 exception_info0 register"]
pub mod m1_exception_info0;
#[doc = "M1_EXCEPTION_INFO1 (r) register accessor: M1 exception_info1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1_exception_info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_exception_info1`] module"]
pub type M1_EXCEPTION_INFO1 = crate::Reg<m1_exception_info1::M1_EXCEPTION_INFO1_SPEC>;
#[doc = "M1 exception_info1 register"]
pub mod m1_exception_info1;
#[doc = "M2_STATUS (r) register accessor: M2 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_status`] module"]
pub type M2_STATUS = crate::Reg<m2_status::M2_STATUS_SPEC>;
#[doc = "M2 status register"]
pub mod m2_status;
#[doc = "M2_STATUS_CLR (w) register accessor: M2 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_status_clr`] module"]
pub type M2_STATUS_CLR = crate::Reg<m2_status_clr::M2_STATUS_CLR_SPEC>;
#[doc = "M2 status clear register"]
pub mod m2_status_clr;
#[doc = "M2_EXCEPTION_INFO0 (r) register accessor: M2 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2_exception_info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_exception_info0`] module"]
pub type M2_EXCEPTION_INFO0 = crate::Reg<m2_exception_info0::M2_EXCEPTION_INFO0_SPEC>;
#[doc = "M2 exception_info0 register"]
pub mod m2_exception_info0;
#[doc = "M2_EXCEPTION_INFO1 (r) register accessor: M2 exception_info1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2_exception_info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_exception_info1`] module"]
pub type M2_EXCEPTION_INFO1 = crate::Reg<m2_exception_info1::M2_EXCEPTION_INFO1_SPEC>;
#[doc = "M2 exception_info1 register"]
pub mod m2_exception_info1;
#[doc = "M3_STATUS (r) register accessor: M3 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_status`] module"]
pub type M3_STATUS = crate::Reg<m3_status::M3_STATUS_SPEC>;
#[doc = "M3 status register"]
pub mod m3_status;
#[doc = "M3_STATUS_CLR (w) register accessor: M3 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_status_clr`] module"]
pub type M3_STATUS_CLR = crate::Reg<m3_status_clr::M3_STATUS_CLR_SPEC>;
#[doc = "M3 status clear register"]
pub mod m3_status_clr;
#[doc = "M3_EXCEPTION_INFO0 (r) register accessor: M3 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3_exception_info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_exception_info0`] module"]
pub type M3_EXCEPTION_INFO0 = crate::Reg<m3_exception_info0::M3_EXCEPTION_INFO0_SPEC>;
#[doc = "M3 exception_info0 register"]
pub mod m3_exception_info0;
#[doc = "M3_EXCEPTION_INFO1 (r) register accessor: M3 exception_info1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3_exception_info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_exception_info1`] module"]
pub type M3_EXCEPTION_INFO1 = crate::Reg<m3_exception_info1::M3_EXCEPTION_INFO1_SPEC>;
#[doc = "M3 exception_info1 register"]
pub mod m3_exception_info1;
#[doc = "M4_STATUS (r) register accessor: M4 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4_status`] module"]
pub type M4_STATUS = crate::Reg<m4_status::M4_STATUS_SPEC>;
#[doc = "M4 status register"]
pub mod m4_status;
#[doc = "M4_STATUS_CLR (w) register accessor: M4 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4_status_clr`] module"]
pub type M4_STATUS_CLR = crate::Reg<m4_status_clr::M4_STATUS_CLR_SPEC>;
#[doc = "M4 status clear register"]
pub mod m4_status_clr;
#[doc = "M4_EXCEPTION_INFO0 (r) register accessor: M4 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4_exception_info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4_exception_info0`] module"]
pub type M4_EXCEPTION_INFO0 = crate::Reg<m4_exception_info0::M4_EXCEPTION_INFO0_SPEC>;
#[doc = "M4 exception_info0 register"]
pub mod m4_exception_info0;
#[doc = "M4_EXCEPTION_INFO1 (r) register accessor: M4 exception_info1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4_exception_info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4_exception_info1`] module"]
pub type M4_EXCEPTION_INFO1 = crate::Reg<m4_exception_info1::M4_EXCEPTION_INFO1_SPEC>;
#[doc = "M4 exception_info1 register"]
pub mod m4_exception_info1;
#[doc = "INT_EN (rw) register accessor: APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "APM interrupt enable register"]
pub mod int_en;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
