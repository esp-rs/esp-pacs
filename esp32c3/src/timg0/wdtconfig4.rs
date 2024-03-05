#[doc = "Register `WDTCONFIG4` reader"]
pub type R = crate::R<WDTCONFIG4_SPEC>;
#[doc = "Register `WDTCONFIG4` writer"]
pub type W = crate::W<WDTCONFIG4_SPEC>;
#[doc = "Field `WDT_STG2_HOLD` reader - reg_wdt_stg2_hold."]
pub type WDT_STG2_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG2_HOLD` writer - reg_wdt_stg2_hold."]
pub type WDT_STG2_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_wdt_stg2_hold."]
    #[inline(always)]
    pub fn wdt_stg2_hold(&self) -> WDT_STG2_HOLD_R {
        WDT_STG2_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG4")
            .field(
                "wdt_stg2_hold",
                &format_args!("{}", self.wdt_stg2_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_wdt_stg2_hold."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2_hold(&mut self) -> WDT_STG2_HOLD_W<WDTCONFIG4_SPEC> {
        WDT_STG2_HOLD_W::new(self, 0)
    }
}
#[doc = "TIMG_WDTCONFIG4_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG4_SPEC;
impl crate::RegisterSpec for WDTCONFIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig4::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig4::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCONFIG4 to value 0x000f_ffff"]
impl crate::Resettable for WDTCONFIG4_SPEC {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
