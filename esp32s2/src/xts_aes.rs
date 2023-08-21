#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100..0x140 - Plaintext register %s"]
    pub plain_: [PLAIN_; 16],
    #[doc = "0x140 - Configures the size of target memory space"]
    pub linesize: LINESIZE,
    #[doc = "0x144 - Configures the type of the external memory"]
    pub destination: DESTINATION,
    #[doc = "0x148 - Physical address"]
    pub physical_address: PHYSICAL_ADDRESS,
    #[doc = "0x14c - Activates AES algorithm"]
    pub trigger: TRIGGER,
    #[doc = "0x150 - Release control"]
    pub release: RELEASE,
    #[doc = "0x154 - Destroys control"]
    pub destroy: DESTROY,
    #[doc = "0x158 - Status register"]
    pub state: STATE,
    #[doc = "0x15c - Version control register"]
    pub date: DATE,
}
#[doc = "PLAIN_ (rw) register accessor: Plaintext register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plain_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plain_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`plain_`] module"]
pub type PLAIN_ = crate::Reg<plain_::PLAIN__SPEC>;
#[doc = "Plaintext register %s"]
pub mod plain_;
#[doc = "LINESIZE (rw) register accessor: Configures the size of target memory space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linesize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linesize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`linesize`] module"]
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
#[doc = "Configures the size of target memory space"]
pub mod linesize;
#[doc = "DESTINATION (rw) register accessor: Configures the type of the external memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`destination`] module"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "Configures the type of the external memory"]
pub mod destination;
#[doc = "PHYSICAL_ADDRESS (rw) register accessor: Physical address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`physical_address`] module"]
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
#[doc = "Physical address"]
pub mod physical_address;
#[doc = "TRIGGER (w) register accessor: Activates AES algorithm\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Activates AES algorithm"]
pub mod trigger;
#[doc = "RELEASE (w) register accessor: Release control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`release`] module"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "Release control"]
pub mod release;
#[doc = "DESTROY (w) register accessor: Destroys control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`destroy`] module"]
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
#[doc = "Destroys control"]
pub mod destroy;
#[doc = "STATE (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Status register"]
pub mod state;
#[doc = "DATE (r) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
