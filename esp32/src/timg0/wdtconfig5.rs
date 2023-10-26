#[doc = "Register `WDTCONFIG5` reader"]
pub type R = crate::R<WDTCONFIG5_SPEC>;
#[doc = "Register `WDTCONFIG5` writer"]
pub type W = crate::W<WDTCONFIG5_SPEC>;
#[doc = "Field `WDT_STG3_HOLD` reader - Stage 3 timeout value in SWDT clock cycles"]
pub type WDT_STG3_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG3_HOLD` writer - Stage 3 timeout value in SWDT clock cycles"]
pub type WDT_STG3_HOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Stage 3 timeout value in SWDT clock cycles"]
    #[inline(always)]
    pub fn wdt_stg3_hold(&self) -> WDT_STG3_HOLD_R {
        WDT_STG3_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG5")
            .field(
                "wdt_stg3_hold",
                &format_args!("{}", self.wdt_stg3_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stage 3 timeout value in SWDT clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3_hold(&mut self) -> WDT_STG3_HOLD_W<WDTCONFIG5_SPEC, 0> {
        WDT_STG3_HOLD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG5_SPEC;
impl crate::RegisterSpec for WDTCONFIG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig5::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig5::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG5 to value 0x000f_ffff"]
impl crate::Resettable for WDTCONFIG5_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_ffff;
}
