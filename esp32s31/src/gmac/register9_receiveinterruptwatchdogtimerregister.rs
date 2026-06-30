#[doc = "Register `REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER` reader"]
pub type R = crate::R<REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC>;
#[doc = "Register `REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER` writer"]
pub type W = crate::W<REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC>;
#[doc = "Field `RIWT` reader - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\] When the watchdog timer runs out, the RI bit is set and the timer is stopped The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame"]
pub type RIWT_R = crate::FieldReader;
#[doc = "Field `RIWT` writer - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\] When the watchdog timer runs out, the RI bit is set and the timer is stopped The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame"]
pub type RIWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\] When the watchdog timer runs out, the RI bit is set and the timer is stopped The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame"]
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER")
            .field("riwt", &self.riwt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\] When the watchdog timer runs out, the RI bit is set and the timer is stopped The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame"]
    #[inline(always)]
    pub fn riwt(&mut self) -> RIWT_W<'_, REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC> {
        RIWT_W::new(self, 0)
    }
}
#[doc = "Watchdog timeout for Receive Interrupt _RI_ from DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register9_receiveinterruptwatchdogtimerregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register9_receiveinterruptwatchdogtimerregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register9_receiveinterruptwatchdogtimerregister::R`](R) reader structure"]
impl crate::Readable for REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register9_receiveinterruptwatchdogtimerregister::W`](W) writer structure"]
impl crate::Writable for REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER to value 0"]
impl crate::Resettable for REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC {}
