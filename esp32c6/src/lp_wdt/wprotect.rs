#[doc = "Register `WPROTECT` reader"]
pub struct R(crate::R<WPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPROTECT` writer"]
pub struct W(crate::W<WPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPROTECT_SPEC>;
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
impl From<crate::W<WPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_WKEY` reader - need_des"]
pub type WDT_WKEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDT_WKEY` writer - need_des"]
pub type WDT_WKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPROTECT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W<0> {
        WDT_WKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wprotect](index.html) module"]
pub struct WPROTECT_SPEC;
impl crate::RegisterSpec for WPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wprotect::R](R) reader structure"]
impl crate::Readable for WPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wprotect::W](W) writer structure"]
impl crate::Writable for WPROTECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPROTECT to value 0"]
impl crate::Resettable for WPROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
