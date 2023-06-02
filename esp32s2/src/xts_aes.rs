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
#[doc = "PLAIN_ (rw) register accessor: an alias for `Reg<PLAIN__SPEC>`"]
pub type PLAIN_ = crate::Reg<plain_::PLAIN__SPEC>;
#[doc = "Plaintext register %s"]
pub mod plain_;
#[doc = "LINESIZE (rw) register accessor: an alias for `Reg<LINESIZE_SPEC>`"]
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
#[doc = "Configures the size of target memory space"]
pub mod linesize;
#[doc = "DESTINATION (rw) register accessor: an alias for `Reg<DESTINATION_SPEC>`"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "Configures the type of the external memory"]
pub mod destination;
#[doc = "PHYSICAL_ADDRESS (rw) register accessor: an alias for `Reg<PHYSICAL_ADDRESS_SPEC>`"]
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
#[doc = "Physical address"]
pub mod physical_address;
#[doc = "TRIGGER (w) register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Activates AES algorithm"]
pub mod trigger;
#[doc = "RELEASE (w) register accessor: an alias for `Reg<RELEASE_SPEC>`"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "Release control"]
pub mod release;
#[doc = "DESTROY (w) register accessor: an alias for `Reg<DESTROY_SPEC>`"]
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
#[doc = "Destroys control"]
pub mod destroy;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Status register"]
pub mod state;
#[doc = "DATE (r) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
