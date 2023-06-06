#[doc = "Register `WDTCONFIG4` reader"]
pub struct R(crate::R<WDTCONFIG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG4` writer"]
pub struct W(crate::W<WDTCONFIG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WDTCONFIG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_STG3_HOLD` reader - the hold time of stage3"]
pub type WDT_STG3_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG3_HOLD` writer - the hold time of stage3"]
pub type WDT_STG3_HOLD_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG4_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - the hold time of stage3"]
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
    #[doc = "Bits 0:31 - the hold time of stage3"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3_hold(&mut self) -> WDT_STG3_HOLD_W<0> {
        WDT_STG3_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig4](index.html) module"]
pub struct WDTCONFIG4_SPEC;
impl crate::RegisterSpec for WDTCONFIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig4::R](R) reader structure"]
impl crate::Readable for WDTCONFIG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig4::W](W) writer structure"]
impl crate::Writable for WDTCONFIG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG4 to value 0x0fff"]
impl crate::Resettable for WDTCONFIG4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
