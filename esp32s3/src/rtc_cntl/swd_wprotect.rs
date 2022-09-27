#[doc = "Register `SWD_WPROTECT` reader"]
pub struct R(crate::R<SWD_WPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWD_WPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWD_WPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWD_WPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWD_WPROTECT` writer"]
pub struct W(crate::W<SWD_WPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWD_WPROTECT_SPEC>;
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
impl From<crate::W<SWD_WPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWD_WPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWD_WKEY` reader - super watch dog key"]
pub type SWD_WKEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SWD_WKEY` writer - super watch dog key"]
pub type SWD_WKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWD_WPROTECT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - super watch dog key"]
    #[inline(always)]
    pub fn swd_wkey(&self) -> SWD_WKEY_R {
        SWD_WKEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - super watch dog key"]
    #[inline(always)]
    pub fn swd_wkey(&mut self) -> SWD_WKEY_W<0> {
        SWD_WKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "super watch dog key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd_wprotect](index.html) module"]
pub struct SWD_WPROTECT_SPEC;
impl crate::RegisterSpec for SWD_WPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swd_wprotect::R](R) reader structure"]
impl crate::Readable for SWD_WPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swd_wprotect::W](W) writer structure"]
impl crate::Writable for SWD_WPROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWD_WPROTECT to value 0x8f1d_312a"]
impl crate::Resettable for SWD_WPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8f1d_312a
    }
}
