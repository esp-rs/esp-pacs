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
#[doc = "Field `SWD_WKEY` reader - need_des"]
pub type SWD_WKEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SWD_WKEY` writer - need_des"]
pub type SWD_WKEY_W<'a, const O: u8> = crate::FieldWriter<'a, SWD_WPROTECT_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn swd_wkey(&self) -> SWD_WKEY_R {
        SWD_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_WPROTECT")
            .field("swd_wkey", &format_args!("{}", self.swd_wkey().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SWD_WPROTECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
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
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd_wprotect](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWD_WPROTECT to value 0"]
impl crate::Resettable for SWD_WPROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
