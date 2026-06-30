#[doc = "Register `REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER` reader"]
pub type R = crate::R<REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC>;
#[doc = "Register `REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER` writer"]
pub type W = crate::W<REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC>;
#[doc = "Field `CH2_RIWT` reader - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\] When the watchdog timer runs out, the RI bit is set and the timer is stopped The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame"]
pub type CH2_RIWT_R = crate::FieldReader;
#[doc = "Field `CH2_RIWT` writer - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\] When the watchdog timer runs out, the RI bit is set and the timer is stopped The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame"]
pub type CH2_RIWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\] When the watchdog timer runs out, the RI bit is set and the timer is stopped The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame"]
    #[inline(always)]
    pub fn ch2_riwt(&self) -> CH2_RIWT_R {
        CH2_RIWT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER")
            .field("ch2_riwt", &self.ch2_riwt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\] When the watchdog timer runs out, the RI bit is set and the timer is stopped The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame"]
    #[inline(always)]
    pub fn ch2_riwt(
        &mut self,
    ) -> CH2_RIWT_W<'_, REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC> {
        CH2_RIWT_W::new(self, 0)
    }
}
#[doc = "Watchdog timeout for Receive Interrupt _RI_ from DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register137_channel2receiveinterruptwatchdogtimerregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register137_channel2receiveinterruptwatchdogtimerregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register137_channel2receiveinterruptwatchdogtimerregister::R`](R) reader structure"]
impl crate::Readable for REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register137_channel2receiveinterruptwatchdogtimerregister::W`](W) writer structure"]
impl crate::Writable for REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER to value 0"]
impl crate::Resettable for REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC {}
