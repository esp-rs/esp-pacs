#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "LEDC gamma fade configuration RAM for channel %s (16 entries, 64 bytes)."]
pub struct CHANNEL {
    entry: [ENTRY; 16],
}
impl CHANNEL {
    #[doc = "0x00..0x40 - LEDC gamma fade configuration RAM entry %s."]
    #[inline(always)]
    pub const fn entry(&self, n: usize) -> &ENTRY {
        &self.entry[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - LEDC gamma fade configuration RAM entry %s."]
    #[inline(always)]
    pub fn entry_iter(&self) -> impl Iterator<Item = &ENTRY> {
        self.entry.iter()
    }
}
#[doc = "ENTRY (rw) register accessor: LEDC gamma fade configuration RAM entry %s.\n\nYou can [`read`](crate::Reg::read) this register and get [`entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry`] module"]
pub type ENTRY = crate::Reg<entry::ENTRY_SPEC>;
#[doc = "LEDC gamma fade configuration RAM entry %s."]
pub mod entry;
