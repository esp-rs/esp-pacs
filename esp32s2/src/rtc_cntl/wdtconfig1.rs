#[doc = "Register `WDTCONFIG1` reader"]
pub type R = crate::R<WDTCONFIG1_SPEC>;
#[doc = "Register `WDTCONFIG1` writer"]
pub type W = crate::W<WDTCONFIG1_SPEC>;
#[doc = "Field `WDT_STG0_HOLD` reader - Configures the hold time of RTC watchdog at level 1."]
pub type WDT_STG0_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG0_HOLD` writer - Configures the hold time of RTC watchdog at level 1."]
pub type WDT_STG0_HOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the hold time of RTC watchdog at level 1."]
    #[inline(always)]
    pub fn wdt_stg0_hold(&self) -> WDT_STG0_HOLD_R {
        WDT_STG0_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG1")
            .field(
                "wdt_stg0_hold",
                &format_args!("{}", self.wdt_stg0_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the hold time of RTC watchdog at level 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg0_hold(&mut self) -> WDT_STG0_HOLD_W<WDTCONFIG1_SPEC, 0> {
        WDT_STG0_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configures the hold time of RTC watchdog at level 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG1_SPEC;
impl crate::RegisterSpec for WDTCONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig1::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig1::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG1 to value 0x0003_0d40"]
impl crate::Resettable for WDTCONFIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0d40;
}
