#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    int_config: INT_CONFIG,
    int_info: INT_INFO,
    _reserved2: [u8; 0x0ff8],
    int_ip: (),
    _reserved3: [u8; 0x01],
    int_ie: (),
    _reserved4: [u8; 0x01],
    int_attr: (),
    _reserved5: [u8; 0x01],
    int_ctl: (),
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn int_config(&self) -> &INT_CONFIG {
        &self.int_config
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn int_info(&self) -> &INT_INFO {
        &self.int_info
    }
    #[doc = "0x1000..0x1030 - Interrupt pending register."]
    #[inline(always)]
    pub const fn int_ip(&self, n: usize) -> &INT_IP {
        #[allow(clippy::no_effect)]
        [(); 48][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1030 - Interrupt pending register."]
    #[inline(always)]
    pub fn int_ip_iter(&self) -> impl Iterator<Item = &INT_IP> {
        (0..48).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1001..0x1031 - Interrupt enable register."]
    #[inline(always)]
    pub const fn int_ie(&self, n: usize) -> &INT_IE {
        #[allow(clippy::no_effect)]
        [(); 48][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4097)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1001..0x1031 - Interrupt enable register."]
    #[inline(always)]
    pub fn int_ie_iter(&self) -> impl Iterator<Item = &INT_IE> {
        (0..48).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4097)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1002..0x1032 - Interrupt attribute register."]
    #[inline(always)]
    pub const fn int_attr(&self, n: usize) -> &INT_ATTR {
        #[allow(clippy::no_effect)]
        [(); 48][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4098)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1002..0x1032 - Interrupt attribute register."]
    #[inline(always)]
    pub fn int_attr_iter(&self) -> impl Iterator<Item = &INT_ATTR> {
        (0..48).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4098)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1003..0x1033 - Interrupt level control register."]
    #[inline(always)]
    pub const fn int_ctl(&self, n: usize) -> &INT_CTL {
        #[allow(clippy::no_effect)]
        [(); 48][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4099)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1003..0x1033 - Interrupt level control register."]
    #[inline(always)]
    pub fn int_ctl_iter(&self) -> impl Iterator<Item = &INT_CTL> {
        (0..48).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4099)
                .add(4 * n)
                .cast()
        })
    }
}
#[doc = "INT_CONFIG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_config`] module"]
pub type INT_CONFIG = crate::Reg<int_config::INT_CONFIG_SPEC>;
#[doc = ""]
pub mod int_config;
#[doc = "INT_INFO (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_info`] module"]
pub type INT_INFO = crate::Reg<int_info::INT_INFO_SPEC>;
#[doc = ""]
pub mod int_info;
#[doc = "INT_IP (rw) register accessor: Interrupt pending register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ip`] module"]
pub type INT_IP = crate::Reg<int_ip::INT_IP_SPEC>;
#[doc = "Interrupt pending register."]
pub mod int_ip;
#[doc = "INT_IE (rw) register accessor: Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ie`] module"]
pub type INT_IE = crate::Reg<int_ie::INT_IE_SPEC>;
#[doc = "Interrupt enable register."]
pub mod int_ie;
#[doc = "INT_ATTR (rw) register accessor: Interrupt attribute register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_attr`] module"]
pub type INT_ATTR = crate::Reg<int_attr::INT_ATTR_SPEC>;
#[doc = "Interrupt attribute register."]
pub mod int_attr;
#[doc = "INT_CTL (rw) register accessor: Interrupt level control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ctl`] module"]
pub type INT_CTL = crate::Reg<int_ctl::INT_CTL_SPEC>;
#[doc = "Interrupt level control register."]
pub mod int_ctl;
