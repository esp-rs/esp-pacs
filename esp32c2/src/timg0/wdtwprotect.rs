#[doc = "Register `WDTWPROTECT` reader"]
pub struct R(crate::R<WDTWPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTWPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTWPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTWPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTWPROTECT` writer"]
pub struct W(crate::W<WDTWPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTWPROTECT_SPEC>;
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
impl From<crate::W<WDTWPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTWPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_WKEY` reader - If the register contains a different value than its reset value, write protection is enabled."]
pub type WDT_WKEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDT_WKEY` writer - If the register contains a different value than its reset value, write protection is enabled."]
pub type WDT_WKEY_W<'a> = crate::FieldWriter<'a, u32, WDTWPROTECT_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - If the register contains a different value than its reset value, write protection is enabled."]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - If the register contains a different value than its reset value, write protection is enabled."]
    #[inline(always)]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W {
        WDT_WKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog write protect register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtwprotect](index.html) module"]
pub struct WDTWPROTECT_SPEC;
impl crate::RegisterSpec for WDTWPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtwprotect::R](R) reader structure"]
impl crate::Readable for WDTWPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtwprotect::W](W) writer structure"]
impl crate::Writable for WDTWPROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTWPROTECT to value 0x50d8_3aa1"]
impl crate::Resettable for WDTWPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x50d8_3aa1
    }
}
