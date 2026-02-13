#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    channel: [CHANNEL; 6],
}
impl RegisterBlock {
    #[doc = "0x00..0x180 - LEDC gamma fade configuration RAM for channel %s (16 entries, 64 bytes)."]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &CHANNEL {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x180 - LEDC gamma fade configuration RAM for channel %s (16 entries, 64 bytes)."]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &CHANNEL> {
        self.channel.iter()
    }
}
#[doc = "LEDC gamma fade configuration RAM for channel %s (16 entries, 64 bytes)."]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "LEDC gamma fade configuration RAM for channel %s (16 entries, 64 bytes)."]
pub mod channel;
