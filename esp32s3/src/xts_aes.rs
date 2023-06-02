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
#[doc = "PLAIN_ (rw) register accessor: an alias for `Reg<PLAIN__SPEC>`"]
pub type PLAIN_ = crate::Reg<plain_::PLAIN__SPEC>;
#[doc = "Plaintext register %s"]
pub mod plain_;
#[doc = "LINESIZE (rw) register accessor: an alias for `Reg<LINESIZE_SPEC>`"]
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
#[doc = "XTS-AES line-size register"]
pub mod linesize;
#[doc = "DESTINATION (rw) register accessor: an alias for `Reg<DESTINATION_SPEC>`"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "XTS-AES destination register"]
pub mod destination;
#[doc = "PHYSICAL_ADDRESS (rw) register accessor: an alias for `Reg<PHYSICAL_ADDRESS_SPEC>`"]
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
#[doc = "physical address"]
pub mod physical_address;
#[doc = "TRIGGER (w) register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "XTS-AES trigger register"]
pub mod trigger;
#[doc = "RELEASE (w) register accessor: an alias for `Reg<RELEASE_SPEC>`"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "XTS-AES release control register"]
pub mod release;
#[doc = "DESTROY (w) register accessor: an alias for `Reg<DESTROY_SPEC>`"]
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
#[doc = "XTS-AES destroy control register"]
pub mod destroy;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "XTS-AES status register"]
pub mod state;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "XTS-AES version control register"]
pub mod date;
