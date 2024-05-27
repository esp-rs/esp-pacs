///Register `COCPU_DISABLE` reader
pub type R = crate::R<COCPU_DISABLE_SPEC>;
///Register `COCPU_DISABLE` writer
pub type W = crate::W<COCPU_DISABLE_SPEC>;
///Field `DISABLE_RTC_CPU` reader - configure ulp diable
pub type DISABLE_RTC_CPU_R = crate::BitReader;
///Field `DISABLE_RTC_CPU` writer - configure ulp diable
pub type DISABLE_RTC_CPU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - configure ulp diable
    #[inline(always)]
    pub fn disable_rtc_cpu(&self) -> DISABLE_RTC_CPU_R {
        DISABLE_RTC_CPU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COCPU_DISABLE")
            .field("disable_rtc_cpu", &self.disable_rtc_cpu())
            .finish()
    }
}
impl W {
    ///Bit 31 - configure ulp diable
    #[inline(always)]
    #[must_use]
    pub fn disable_rtc_cpu(&mut self) -> DISABLE_RTC_CPU_W<COCPU_DISABLE_SPEC> {
        DISABLE_RTC_CPU_W::new(self, 31)
    }
}
/**configure ulp diable

You can [`read`](crate::generic::Reg::read) this register and get [`cocpu_disable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cocpu_disable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COCPU_DISABLE_SPEC;
impl crate::RegisterSpec for COCPU_DISABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cocpu_disable::R`](R) reader structure
impl crate::Readable for COCPU_DISABLE_SPEC {}
///`write(|w| ..)` method takes [`cocpu_disable::W`](W) writer structure
impl crate::Writable for COCPU_DISABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COCPU_DISABLE to value 0
impl crate::Resettable for COCPU_DISABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
