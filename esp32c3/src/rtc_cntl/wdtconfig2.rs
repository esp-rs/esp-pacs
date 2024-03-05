#[doc = "Register `WDTCONFIG2` reader"]
pub type R = crate::R<WDTCONFIG2_SPEC>;
#[doc = "Register `WDTCONFIG2` writer"]
pub type W = crate::W<WDTCONFIG2_SPEC>;
#[doc = "Field `WDT_STG1_HOLD` reader - the hold time of stage1"]
pub type WDT_STG1_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG1_HOLD` writer - the hold time of stage1"]
pub type WDT_STG1_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the hold time of stage1"]
    #[inline(always)]
    pub fn wdt_stg1_hold(&self) -> WDT_STG1_HOLD_R {
        WDT_STG1_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG2")
            .field(
                "wdt_stg1_hold",
                &format_args!("{}", self.wdt_stg1_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the hold time of stage1"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1_hold(&mut self) -> WDT_STG1_HOLD_W<WDTCONFIG2_SPEC> {
        WDT_STG1_HOLD_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG2_SPEC;
impl crate::RegisterSpec for WDTCONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig2::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig2::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCONFIG2 to value 0x0001_3880"]
impl crate::Resettable for WDTCONFIG2_SPEC {
    const RESET_VALUE: u32 = 0x0001_3880;
}
