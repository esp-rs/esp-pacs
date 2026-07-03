#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    entry: [ENTRY; 128],
}
impl RegisterBlock {
    #[doc = "0x00..0x200 - MMU table entry"]
    #[inline(always)]
    pub const fn entry(&self, n: usize) -> &ENTRY {
        &self.entry[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x200 - MMU table entry"]
    #[inline(always)]
    pub fn entry_iter(&self) -> impl Iterator<Item = &ENTRY> {
        self.entry.iter()
    }
}
#[doc = "ENTRY (rw) register accessor: MMU table entry\n\nYou can [`read`](crate::Reg::read) this register and get [`entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry`] module"]
pub type ENTRY = crate::Reg<entry::ENTRY_SPEC>;
#[doc = "MMU table entry"]
pub mod entry;
