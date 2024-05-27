#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    ///0x100..0x140 - Plaintext register %s
    #[inline(always)]
    pub const fn plain_(&self, n: usize) -> &PLAIN_ {
        &self.plain_[n]
    }
    ///Iterator for array of:
    ///0x100..0x140 - Plaintext register %s
    #[inline(always)]
    pub fn plain__iter(&self) -> impl Iterator<Item = &PLAIN_> {
        self.plain_.iter()
    }
    ///0x140 - Configures the size of target memory space
    #[inline(always)]
    pub const fn linesize(&self) -> &LINESIZE {
        &self.linesize
    }
    ///0x144 - Configures the type of the external memory
    #[inline(always)]
    pub const fn destination(&self) -> &DESTINATION {
        &self.destination
    }
    ///0x148 - Physical address
    #[inline(always)]
    pub const fn physical_address(&self) -> &PHYSICAL_ADDRESS {
        &self.physical_address
    }
    ///0x14c - Activates AES algorithm
    #[inline(always)]
    pub const fn trigger(&self) -> &TRIGGER {
        &self.trigger
    }
    ///0x150 - Release control
    #[inline(always)]
    pub const fn release(&self) -> &RELEASE {
        &self.release
    }
    ///0x154 - Destroys control
    #[inline(always)]
    pub const fn destroy(&self) -> &DESTROY {
        &self.destroy
    }
    ///0x158 - Status register
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    ///0x15c - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**PLAIN_ (rw) register accessor: Plaintext register %s

You can [`read`](crate::generic::Reg::read) this register and get [`plain_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plain_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@plain_`] module*/
pub type PLAIN_ = crate::Reg<plain_::PLAIN__SPEC>;
///Plaintext register %s
pub mod plain_;
/**LINESIZE (rw) register accessor: Configures the size of target memory space

You can [`read`](crate::generic::Reg::read) this register and get [`linesize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linesize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@linesize`] module*/
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
///Configures the size of target memory space
pub mod linesize;
/**DESTINATION (rw) register accessor: Configures the type of the external memory

You can [`read`](crate::generic::Reg::read) this register and get [`destination::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@destination`] module*/
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
///Configures the type of the external memory
pub mod destination;
/**PHYSICAL_ADDRESS (rw) register accessor: Physical address

You can [`read`](crate::generic::Reg::read) this register and get [`physical_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@physical_address`] module*/
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
///Physical address
pub mod physical_address;
/**TRIGGER (w) register accessor: Activates AES algorithm

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trigger`] module*/
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
///Activates AES algorithm
pub mod trigger;
/**RELEASE (w) register accessor: Release control

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@release`] module*/
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
///Release control
pub mod release;
/**DESTROY (w) register accessor: Destroys control

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@destroy`] module*/
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
///Destroys control
pub mod destroy;
/**STATE (r) register accessor: Status register

You can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state`] module*/
pub type STATE = crate::Reg<state::STATE_SPEC>;
///Status register
pub mod state;
/**DATE (r) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
