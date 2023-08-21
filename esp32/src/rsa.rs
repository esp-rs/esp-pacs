#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - Represents M"]
    pub m_mem: [M_MEM; 128],
    _reserved1: [u8; 0x0180],
    #[doc = "0x200..0x280 - Represents Z"]
    pub z_mem: [Z_MEM; 128],
    _reserved2: [u8; 0x0180],
    #[doc = "0x400..0x480 - Represents Y"]
    pub y_mem: [Y_MEM; 128],
    _reserved3: [u8; 0x0180],
    #[doc = "0x600..0x680 - Represents X"]
    pub x_mem: [X_MEM; 128],
    _reserved4: [u8; 0x0180],
    #[doc = "0x800 - "]
    pub m_prime: M_PRIME,
    #[doc = "0x804 - "]
    pub modexp_mode: MODEXP_MODE,
    #[doc = "0x808 - "]
    pub modexp_start: MODEXP_START,
    #[doc = "0x80c - "]
    pub mult_mode: MULT_MODE,
    #[doc = "0x810 - "]
    pub mult_start: MULT_START,
    #[doc = "0x814 - "]
    pub interrupt: INTERRUPT,
    #[doc = "0x818 - "]
    pub clean: CLEAN,
}
#[doc = "M_PRIME (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_prime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_prime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m_prime`] module"]
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
#[doc = ""]
pub mod m_prime;
#[doc = "MODEXP_MODE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modexp_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modexp_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`modexp_mode`] module"]
pub type MODEXP_MODE = crate::Reg<modexp_mode::MODEXP_MODE_SPEC>;
#[doc = ""]
pub mod modexp_mode;
#[doc = "MODEXP_START (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modexp_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`modexp_start`] module"]
pub type MODEXP_START = crate::Reg<modexp_start::MODEXP_START_SPEC>;
#[doc = ""]
pub mod modexp_start;
#[doc = "MULT_MODE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_mode`] module"]
pub type MULT_MODE = crate::Reg<mult_mode::MULT_MODE_SPEC>;
#[doc = ""]
pub mod mult_mode;
#[doc = "MULT_START (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_start`] module"]
pub type MULT_START = crate::Reg<mult_start::MULT_START_SPEC>;
#[doc = ""]
pub mod mult_start;
#[doc = "INTERRUPT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`interrupt`] module"]
pub type INTERRUPT = crate::Reg<interrupt::INTERRUPT_SPEC>;
#[doc = ""]
pub mod interrupt;
#[doc = "CLEAN (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clean::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clean`] module"]
pub type CLEAN = crate::Reg<clean::CLEAN_SPEC>;
#[doc = ""]
pub mod clean;
#[doc = "M_MEM (rw) register accessor: Represents M\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m_mem`] module"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "Represents M"]
pub mod m_mem;
#[doc = "Z_MEM (rw) register accessor: Represents Z\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`z_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`z_mem`] module"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "Represents Z"]
pub mod z_mem;
#[doc = "Y_MEM (rw) register accessor: Represents Y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`y_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`y_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`y_mem`] module"]
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
#[doc = "Represents Y"]
pub mod y_mem;
#[doc = "X_MEM (rw) register accessor: Represents X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`x_mem`] module"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "Represents X"]
pub mod x_mem;
