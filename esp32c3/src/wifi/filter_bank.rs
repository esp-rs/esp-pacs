#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces."]
pub struct FILTER_BANK {
    addr_low: (),
    _reserved1: [u8; 0x04],
    addr_high: (),
    _reserved2: [u8; 0x1c],
    mask_low: (),
    _reserved3: [u8; 0x04],
    mask_high: (),
    _reserved_end: [u8; 0x1c],
}
impl FILTER_BANK {
    #[doc = "0x00..0x10 - First 4 bytes of BSSID MAC address filter"]
    #[inline(always)]
    pub const fn addr_low(&self, n: usize) -> &ADDR_LOW {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - First 4 bytes of BSSID MAC address filter"]
    #[inline(always)]
    pub fn addr_low_iter(&self) -> impl Iterator<Item = &ADDR_LOW> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8 * n).cast() })
    }
    #[doc = "0x04..0x14 - last 2 bytes of BSSID MAC address filter"]
    #[inline(always)]
    pub const fn addr_high(&self, n: usize) -> &ADDR_HIGH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - last 2 bytes of BSSID MAC address filter"]
    #[inline(always)]
    pub fn addr_high_iter(&self) -> impl Iterator<Item = &ADDR_HIGH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x20..0x30 - First 4 bytes of BSSID MAC address filter mask"]
    #[inline(always)]
    pub const fn mask_low(&self, n: usize) -> &MASK_LOW {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - First 4 bytes of BSSID MAC address filter mask"]
    #[inline(always)]
    pub fn mask_low_iter(&self) -> impl Iterator<Item = &MASK_LOW> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x24..0x34 - last 2 bytes of BSSID MAC address filter mask"]
    #[inline(always)]
    pub const fn mask_high(&self, n: usize) -> &MASK_HIGH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x34 - last 2 bytes of BSSID MAC address filter mask"]
    #[inline(always)]
    pub fn mask_high_iter(&self) -> impl Iterator<Item = &MASK_HIGH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36)
                .add(8 * n)
                .cast()
        })
    }
}
#[doc = "ADDR_LOW (rw) register accessor: First 4 bytes of BSSID MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_low`] module"]
pub type ADDR_LOW = crate::Reg<addr_low::ADDR_LOW_SPEC>;
#[doc = "First 4 bytes of BSSID MAC address filter"]
pub mod addr_low;
#[doc = "ADDR_HIGH (rw) register accessor: last 2 bytes of BSSID MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_high`] module"]
pub type ADDR_HIGH = crate::Reg<addr_high::ADDR_HIGH_SPEC>;
#[doc = "last 2 bytes of BSSID MAC address filter"]
pub mod addr_high;
#[doc = "MASK_LOW (rw) register accessor: First 4 bytes of BSSID MAC address filter mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_low`] module"]
pub type MASK_LOW = crate::Reg<mask_low::MASK_LOW_SPEC>;
#[doc = "First 4 bytes of BSSID MAC address filter mask"]
pub mod mask_low;
#[doc = "MASK_HIGH (rw) register accessor: last 2 bytes of BSSID MAC address filter mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_high`] module"]
pub type MASK_HIGH = crate::Reg<mask_high::MASK_HIGH_SPEC>;
#[doc = "last 2 bytes of BSSID MAC address filter mask"]
pub mod mask_high;
