#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - EDGE_INT_ENABLE"]
    pub edge_int_enable: EDGE_INT_ENABLE,
    _reserved1: [u8; 0x04],
    #[doc = "0x0c - Controls SPI memory-mapped caching"]
    pub spi_cache: SPI_CACHE,
    _reserved2: [u8; 0x04],
    #[doc = "0x14 - DPORT_CTL"]
    pub dport_ctl: DPORT_CTL,
    _reserved3: [u8; 0x08],
    #[doc = "0x20 - SPI interrupt type register"]
    pub spi_interrupt_type: SPI_INTERRUPT_TYPE,
    #[doc = "0x24 - Control where the cache is mapped (unconfirmed)"]
    pub spi_cache_target: SPI_CACHE_TARGET,
    #[doc = "0x28 - IO Swap register"]
    pub ioswap: IOSWAP,
}
#[doc = "EDGE_INT_ENABLE (rw) register accessor: an alias for `Reg<EDGE_INT_ENABLE_SPEC>`"]
pub type EDGE_INT_ENABLE = crate::Reg<edge_int_enable::EDGE_INT_ENABLE_SPEC>;
#[doc = "EDGE_INT_ENABLE"]
pub mod edge_int_enable;
#[doc = "DPORT_CTL (rw) register accessor: an alias for `Reg<DPORT_CTL_SPEC>`"]
pub type DPORT_CTL = crate::Reg<dport_ctl::DPORT_CTL_SPEC>;
#[doc = "DPORT_CTL"]
pub mod dport_ctl;
#[doc = "IOSWAP (rw) register accessor: an alias for `Reg<IOSWAP_SPEC>`"]
pub type IOSWAP = crate::Reg<ioswap::IOSWAP_SPEC>;
#[doc = "IO Swap register"]
pub mod ioswap;
#[doc = "SPI_CACHE (rw) register accessor: an alias for `Reg<SPI_CACHE_SPEC>`"]
pub type SPI_CACHE = crate::Reg<spi_cache::SPI_CACHE_SPEC>;
#[doc = "Controls SPI memory-mapped caching"]
pub mod spi_cache;
#[doc = "SPI_INTERRUPT_TYPE (r) register accessor: an alias for `Reg<SPI_INTERRUPT_TYPE_SPEC>`"]
pub type SPI_INTERRUPT_TYPE = crate::Reg<spi_interrupt_type::SPI_INTERRUPT_TYPE_SPEC>;
#[doc = "SPI interrupt type register"]
pub mod spi_interrupt_type;
#[doc = "SPI_CACHE_TARGET (rw) register accessor: an alias for `Reg<SPI_CACHE_TARGET_SPEC>`"]
pub type SPI_CACHE_TARGET = crate::Reg<spi_cache_target::SPI_CACHE_TARGET_SPEC>;
#[doc = "Control where the cache is mapped (unconfirmed)"]
pub mod spi_cache_target;
