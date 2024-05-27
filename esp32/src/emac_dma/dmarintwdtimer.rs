#[doc = "Register `DMARINTWDTIMER` reader"]
pub type R = crate::R<DMARINTWDTIMER_SPEC>;
#[doc = "Register `DMARINTWDTIMER` writer"]
pub type W = crate::W<DMARINTWDTIMER_SPEC>;
#[doc = "Field `RIWTC` reader - This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI(RECV_INT) status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame."]
pub type RIWTC_R = crate::FieldReader;
#[doc = "Field `RIWTC` writer - This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI(RECV_INT) status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame."]
pub type RIWTC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI(RECV_INT) status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame."]
    #[inline(always)]
    pub fn riwtc(&self) -> RIWTC_R {
        RIWTC_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARINTWDTIMER")
            .field("riwtc", &self.riwtc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI(RECV_INT) status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame."]
    #[inline(always)]
    #[must_use]
    pub fn riwtc(&mut self) -> RIWTC_W<DMARINTWDTIMER_SPEC> {
        RIWTC_W::new(self, 0)
    }
}
#[doc = "Watchdog timer count on receive\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarintwdtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarintwdtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARINTWDTIMER_SPEC;
impl crate::RegisterSpec for DMARINTWDTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarintwdtimer::R`](R) reader structure"]
impl crate::Readable for DMARINTWDTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmarintwdtimer::W`](W) writer structure"]
impl crate::Writable for DMARINTWDTIMER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARINTWDTIMER to value 0"]
impl crate::Resettable for DMARINTWDTIMER_SPEC {
    const RESET_VALUE: u32 = 0;
}
