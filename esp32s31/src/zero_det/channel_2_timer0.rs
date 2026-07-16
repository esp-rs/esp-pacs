#[doc = "Register `CHANNEL_2_TIMER0` reader"]
pub type R = crate::R<CHANNEL_2_TIMER0_SPEC>;
#[doc = "Field `CHANNEL_2_TIMER0` reader - record the time while detect the frist zero det int in channel 2"]
pub type CHANNEL_2_TIMER0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - record the time while detect the frist zero det int in channel 2"]
    #[inline(always)]
    pub fn channel_2_timer0(&self) -> CHANNEL_2_TIMER0_R {
        CHANNEL_2_TIMER0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHANNEL_2_TIMER0")
            .field("channel_2_timer0", &self.channel_2_timer0())
            .finish()
    }
}
#[doc = "record timer reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_2_timer0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANNEL_2_TIMER0_SPEC;
impl crate::RegisterSpec for CHANNEL_2_TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel_2_timer0::R`](R) reader structure"]
impl crate::Readable for CHANNEL_2_TIMER0_SPEC {}
#[doc = "`reset()` method sets CHANNEL_2_TIMER0 to value 0"]
impl crate::Resettable for CHANNEL_2_TIMER0_SPEC {}
