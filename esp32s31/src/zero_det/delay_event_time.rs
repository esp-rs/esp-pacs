#[doc = "Register `DELAY_EVENT_TIME` reader"]
pub type R = crate::R<DELAY_EVENT_TIME_SPEC>;
#[doc = "Register `DELAY_EVENT_TIME` writer"]
pub type W = crate::W<DELAY_EVENT_TIME_SPEC>;
#[doc = "Field `DELAY_EVENT_TIME` reader - delay time after zero det int to trigger event for each channel ,it must be greater than or equal to 1"]
pub type DELAY_EVENT_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_EVENT_TIME` writer - delay time after zero det int to trigger event for each channel ,it must be greater than or equal to 1"]
pub type DELAY_EVENT_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - delay time after zero det int to trigger event for each channel ,it must be greater than or equal to 1"]
    #[inline(always)]
    pub fn delay_event_time(&self) -> DELAY_EVENT_TIME_R {
        DELAY_EVENT_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DELAY_EVENT_TIME")
            .field("delay_event_time", &self.delay_event_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - delay time after zero det int to trigger event for each channel ,it must be greater than or equal to 1"]
    #[inline(always)]
    pub fn delay_event_time(&mut self) -> DELAY_EVENT_TIME_W<'_, DELAY_EVENT_TIME_SPEC> {
        DELAY_EVENT_TIME_W::new(self, 0)
    }
}
#[doc = "delay time reg\n\nYou can [`read`](crate::Reg::read) this register and get [`delay_event_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay_event_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DELAY_EVENT_TIME_SPEC;
impl crate::RegisterSpec for DELAY_EVENT_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay_event_time::R`](R) reader structure"]
impl crate::Readable for DELAY_EVENT_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`delay_event_time::W`](W) writer structure"]
impl crate::Writable for DELAY_EVENT_TIME_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DELAY_EVENT_TIME to value 0x0f"]
impl crate::Resettable for DELAY_EVENT_TIME_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
