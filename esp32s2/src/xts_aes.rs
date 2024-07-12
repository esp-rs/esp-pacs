#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    plain_: [PLAIN_; 16],
    linesize: LINESIZE,
    destination: DESTINATION,
    physical_address: PHYSICAL_ADDRESS,
    trigger: TRIGGER,
    release: RELEASE,
    destroy: DESTROY,
    state: STATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x100..0x140 - Plaintext register %s"]
    #[inline(always)]
    pub const fn plain_(&self, n: usize) -> &PLAIN_ {
        &self.plain_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - Plaintext register %s"]
    #[inline(always)]
    pub fn plain__iter(&self) -> impl Iterator<Item = &PLAIN_> {
        self.plain_.iter()
    }
    #[doc = "0x140 - Configures the size of target memory space"]
    #[inline(always)]
    pub const fn linesize(&self) -> &LINESIZE {
        &self.linesize
    }
    #[doc = "0x144 - Configures the type of the external memory"]
    #[inline(always)]
    pub const fn destination(&self) -> &DESTINATION {
        &self.destination
    }
    #[doc = "0x148 - Physical address"]
    #[inline(always)]
    pub const fn physical_address(&self) -> &PHYSICAL_ADDRESS {
        &self.physical_address
    }
    #[doc = "0x14c - Activates AES algorithm"]
    #[inline(always)]
    pub const fn trigger(&self) -> &TRIGGER {
        &self.trigger
    }
    #[doc = "0x150 - Release control"]
    #[inline(always)]
    pub const fn release(&self) -> &RELEASE {
        &self.release
    }
    #[doc = "0x154 - Destroys control"]
    #[inline(always)]
    pub const fn destroy(&self) -> &DESTROY {
        &self.destroy
    }
    #[doc = "0x158 - Status register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x15c - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "PLAIN_ (rw) register accessor: Plaintext register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`plain_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plain_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plain_`] module"]
pub type PLAIN_ = crate::Reg<plain_::PLAIN__SPEC>;
#[doc = "Plaintext register %s"]
pub mod plain_;
#[doc = "LINESIZE (rw) register accessor: Configures the size of target memory space\n\nYou can [`read`](crate::Reg::read) this register and get [`linesize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linesize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linesize`] module"]
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
#[doc = "Configures the size of target memory space"]
pub mod linesize;
#[doc = "DESTINATION (rw) register accessor: Configures the type of the external memory\n\nYou can [`read`](crate::Reg::read) this register and get [`destination::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`destination::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination`] module"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "Configures the type of the external memory"]
pub mod destination;
#[doc = "PHYSICAL_ADDRESS (rw) register accessor: Physical address\n\nYou can [`read`](crate::Reg::read) this register and get [`physical_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physical_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_address`] module"]
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
#[doc = "Physical address"]
pub mod physical_address;
#[doc = "TRIGGER (w) register accessor: Activates AES algorithm\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Activates AES algorithm"]
pub mod trigger;
#[doc = "RELEASE (w) register accessor: Release control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@release`] module"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "Release control"]
pub mod release;
#[doc = "DESTROY (w) register accessor: Destroys control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destroy`] module"]
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
#[doc = "Destroys control"]
pub mod destroy;
#[doc = "STATE (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Status register"]
pub mod state;
#[doc = "DATE (r) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
