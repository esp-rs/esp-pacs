///Register `RTC_ULP_CP_TIMER_1` reader
pub type R = crate::R<RTC_ULP_CP_TIMER_1_SPEC>;
///Register `RTC_ULP_CP_TIMER_1` writer
pub type W = crate::W<RTC_ULP_CP_TIMER_1_SPEC>;
///Field `ULP_CP_TIMER_SLP_CYCLE` reader - sleep cycles for ULP-coprocessor timer
pub type ULP_CP_TIMER_SLP_CYCLE_R = crate::FieldReader<u32>;
///Field `ULP_CP_TIMER_SLP_CYCLE` writer - sleep cycles for ULP-coprocessor timer
pub type ULP_CP_TIMER_SLP_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 8:31 - sleep cycles for ULP-coprocessor timer
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&self) -> ULP_CP_TIMER_SLP_CYCLE_R {
        ULP_CP_TIMER_SLP_CYCLE_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_ULP_CP_TIMER_1")
            .field("ulp_cp_timer_slp_cycle", &self.ulp_cp_timer_slp_cycle())
            .finish()
    }
}
impl W {
    ///Bits 8:31 - sleep cycles for ULP-coprocessor timer
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_timer_slp_cycle(&mut self) -> ULP_CP_TIMER_SLP_CYCLE_W<RTC_ULP_CP_TIMER_1_SPEC> {
        ULP_CP_TIMER_SLP_CYCLE_W::new(self, 8)
    }
}
/**configure ulp sleep time

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_ulp_cp_timer_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_ulp_cp_timer_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RTC_ULP_CP_TIMER_1_SPEC;
impl crate::RegisterSpec for RTC_ULP_CP_TIMER_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rtc_ulp_cp_timer_1::R`](R) reader structure
impl crate::Readable for RTC_ULP_CP_TIMER_1_SPEC {}
///`write(|w| ..)` method takes [`rtc_ulp_cp_timer_1::W`](W) writer structure
impl crate::Writable for RTC_ULP_CP_TIMER_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTC_ULP_CP_TIMER_1 to value 0xc800
impl crate::Resettable for RTC_ULP_CP_TIMER_1_SPEC {
    const RESET_VALUE: u32 = 0xc800;
}
