#[doc = "Register `CHANNEL_3_TIMER1` reader"]
pub type R = crate::R<CHANNEL_3_TIMER1_SPEC>;
#[doc = "Field `CHANNEL_3_TIMER1` reader - record the time while detect the second zero det int in channel 3"]
pub type CHANNEL_3_TIMER1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - record the time while detect the second zero det int in channel 3"]
    #[inline(always)]
    pub fn channel_3_timer1(&self) -> CHANNEL_3_TIMER1_R {
        CHANNEL_3_TIMER1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHANNEL_3_TIMER1")
            .field("channel_3_timer1", &self.channel_3_timer1())
            .finish()
    }
}
#[doc = "record timer reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_3_timer1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANNEL_3_TIMER1_SPEC;
impl crate::RegisterSpec for CHANNEL_3_TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel_3_timer1::R`](R) reader structure"]
impl crate::Readable for CHANNEL_3_TIMER1_SPEC {}
#[doc = "`reset()` method sets CHANNEL_3_TIMER1 to value 0"]
impl crate::Resettable for CHANNEL_3_TIMER1_SPEC {}
