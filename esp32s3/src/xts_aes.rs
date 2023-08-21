#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - Plaintext register %s"]
    pub plain_: [PLAIN_; 16],
    #[doc = "0x40 - XTS-AES line-size register"]
    pub linesize: LINESIZE,
    #[doc = "0x44 - XTS-AES destination register"]
    pub destination: DESTINATION,
    #[doc = "0x48 - physical address"]
    pub physical_address: PHYSICAL_ADDRESS,
    #[doc = "0x4c - XTS-AES trigger register"]
    pub trigger: TRIGGER,
    #[doc = "0x50 - XTS-AES release control register"]
    pub release: RELEASE,
    #[doc = "0x54 - XTS-AES destroy control register"]
    pub destroy: DESTROY,
    #[doc = "0x58 - XTS-AES status register"]
    pub state: STATE,
    #[doc = "0x5c - XTS-AES version control register"]
    pub date: DATE,
}
#[doc = "PLAIN_ (rw) register accessor: Plaintext register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plain_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plain_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`plain_`] module"]
pub type PLAIN_ = crate::Reg<plain_::PLAIN__SPEC>;
#[doc = "Plaintext register %s"]
pub mod plain_;
#[doc = "LINESIZE (rw) register accessor: XTS-AES line-size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linesize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linesize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`linesize`] module"]
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
#[doc = "XTS-AES line-size register"]
pub mod linesize;
#[doc = "DESTINATION (rw) register accessor: XTS-AES destination register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`destination`] module"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "XTS-AES destination register"]
pub mod destination;
#[doc = "PHYSICAL_ADDRESS (rw) register accessor: physical address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`physical_address`] module"]
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
#[doc = "physical address"]
pub mod physical_address;
#[doc = "TRIGGER (w) register accessor: XTS-AES trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "XTS-AES trigger register"]
pub mod trigger;
#[doc = "RELEASE (w) register accessor: XTS-AES release control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`release`] module"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "XTS-AES release control register"]
pub mod release;
#[doc = "DESTROY (w) register accessor: XTS-AES destroy control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`destroy`] module"]
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
#[doc = "XTS-AES destroy control register"]
pub mod destroy;
#[doc = "STATE (r) register accessor: XTS-AES status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "XTS-AES status register"]
pub mod state;
#[doc = "DATE (rw) register accessor: XTS-AES version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "XTS-AES version control register"]
pub mod date;
