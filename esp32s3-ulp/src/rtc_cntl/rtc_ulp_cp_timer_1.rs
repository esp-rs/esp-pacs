#[doc = "Register `RTC_ULP_CP_TIMER_1` reader"]
pub type R = crate::R<RTC_ULP_CP_TIMER_1_SPEC>;
#[doc = "Register `RTC_ULP_CP_TIMER_1` writer"]
pub type W = crate::W<RTC_ULP_CP_TIMER_1_SPEC>;
#[doc = "Field `ULP_CP_TIMER_SLP_CYCLE` reader - sleep cycles for ULP-coprocessor timer"]
pub type ULP_CP_TIMER_SLP_CYCLE_R = crate::FieldReader<u32>;
#[doc = "Field `ULP_CP_TIMER_SLP_CYCLE` writer - sleep cycles for ULP-coprocessor timer"]
pub type ULP_CP_TIMER_SLP_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - sleep cycles for ULP-coprocessor timer"]
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
    #[doc = "Bits 8:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&mut self) -> ULP_CP_TIMER_SLP_CYCLE_W<RTC_ULP_CP_TIMER_1_SPEC> {
        ULP_CP_TIMER_SLP_CYCLE_W::new(self, 8)
    }
}
#[doc = "configure ulp sleep time\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_ulp_cp_timer_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_ulp_cp_timer_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_ULP_CP_TIMER_1_SPEC;
impl crate::RegisterSpec for RTC_ULP_CP_TIMER_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_ulp_cp_timer_1::R`](R) reader structure"]
impl crate::Readable for RTC_ULP_CP_TIMER_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_ulp_cp_timer_1::W`](W) writer structure"]
impl crate::Writable for RTC_ULP_CP_TIMER_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_ULP_CP_TIMER_1 to value 0xc800"]
impl crate::Resettable for RTC_ULP_CP_TIMER_1_SPEC {
    const RESET_VALUE: u32 = 0xc800;
}
