#[doc = "Register `WDTCONFIG3` reader"]
pub type R = crate::R<WDTCONFIG3_SPEC>;
#[doc = "Register `WDTCONFIG3` writer"]
pub type W = crate::W<WDTCONFIG3_SPEC>;
#[doc = "Field `WDT_STG1_HOLD` reader - reg_wdt_stg1_hold."]
pub type WDT_STG1_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG1_HOLD` writer - reg_wdt_stg1_hold."]
pub type WDT_STG1_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_wdt_stg1_hold."]
    #[inline(always)]
    pub fn wdt_stg1_hold(&self) -> WDT_STG1_HOLD_R {
        WDT_STG1_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG3")
            .field(
                "wdt_stg1_hold",
                &format_args!("{}", self.wdt_stg1_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_wdt_stg1_hold."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1_hold(&mut self) -> WDT_STG1_HOLD_W<WDTCONFIG3_SPEC> {
        WDT_STG1_HOLD_W::new(self, 0)
    }
}
#[doc = "TIMG_WDTCONFIG3_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG3_SPEC;
impl crate::RegisterSpec for WDTCONFIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig3::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig3::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCONFIG3 to value 0x07ff_ffff"]
impl crate::Resettable for WDTCONFIG3_SPEC {
    const RESET_VALUE: u32 = 0x07ff_ffff;
}
