#[doc = "Register `WDTCONFIG4` reader"]
pub type R = crate::R<WDTCONFIG4_SPEC>;
#[doc = "Register `WDTCONFIG4` writer"]
pub type W = crate::W<WDTCONFIG4_SPEC>;
#[doc = "Field `WDT_STG3_HOLD` reader - Need add desc"]
pub type WDT_STG3_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG3_HOLD` writer - Need add desc"]
pub type WDT_STG3_HOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn wdt_stg3_hold(&self) -> WDT_STG3_HOLD_R {
        WDT_STG3_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG4")
            .field(
                "wdt_stg3_hold",
                &format_args!("{}", self.wdt_stg3_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3_hold(&mut self) -> WDT_STG3_HOLD_W<WDTCONFIG4_SPEC, 0> {
        WDT_STG3_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG4_SPEC;
impl crate::RegisterSpec for WDTCONFIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig4::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig4::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG4 to value 0x0fff"]
impl crate::Resettable for WDTCONFIG4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
