#[doc = "Register `WDTCONFIG5` reader"]
pub struct R(crate::R<WDTCONFIG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG5` writer"]
pub struct W(crate::W<WDTCONFIG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG5_SPEC>;
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
impl From<crate::W<WDTCONFIG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_STG3_HOLD` reader - Stage 3 timeout value, in MWDT clock cycles."]
pub type WDT_STG3_HOLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDT_STG3_HOLD` writer - Stage 3 timeout value, in MWDT clock cycles."]
pub type WDT_STG3_HOLD_W<'a> = crate::FieldWriter<'a, u32, WDTCONFIG5_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Stage 3 timeout value, in MWDT clock cycles."]
    #[inline(always)]
    pub fn wdt_stg3_hold(&self) -> WDT_STG3_HOLD_R {
        WDT_STG3_HOLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stage 3 timeout value, in MWDT clock cycles."]
    #[inline(always)]
    pub fn wdt_stg3_hold(&mut self) -> WDT_STG3_HOLD_W {
        WDT_STG3_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog timer stage 3 timeout value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig5](index.html) module"]
pub struct WDTCONFIG5_SPEC;
impl crate::RegisterSpec for WDTCONFIG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig5::R](R) reader structure"]
impl crate::Readable for WDTCONFIG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig5::W](W) writer structure"]
impl crate::Writable for WDTCONFIG5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCONFIG5 to value 0x000f_ffff"]
impl crate::Resettable for WDTCONFIG5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_ffff
    }
}
