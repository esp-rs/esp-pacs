#[doc = "Register `CK_GLITCH_CNTL` reader"]
pub struct R(crate::R<CK_GLITCH_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CK_GLITCH_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CK_GLITCH_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CK_GLITCH_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CK_GLITCH_CNTL` writer"]
pub struct W(crate::W<CK_GLITCH_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CK_GLITCH_CNTL_SPEC>;
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
impl From<crate::W<CK_GLITCH_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CK_GLITCH_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK_GLITCH_RESET_ENA` reader - need_des"]
pub type CK_GLITCH_RESET_ENA_R = crate::BitReader;
#[doc = "Field `CK_GLITCH_RESET_ENA` writer - need_des"]
pub type CK_GLITCH_RESET_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CK_GLITCH_CNTL_SPEC, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ck_glitch_reset_ena(&self) -> CK_GLITCH_RESET_ENA_R {
        CK_GLITCH_RESET_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CK_GLITCH_CNTL")
            .field(
                "ck_glitch_reset_ena",
                &format_args!("{}", self.ck_glitch_reset_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CK_GLITCH_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_glitch_reset_ena(&mut self) -> CK_GLITCH_RESET_ENA_W<31> {
        CK_GLITCH_RESET_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ck_glitch_cntl](index.html) module"]
pub struct CK_GLITCH_CNTL_SPEC;
impl crate::RegisterSpec for CK_GLITCH_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ck_glitch_cntl::R](R) reader structure"]
impl crate::Readable for CK_GLITCH_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ck_glitch_cntl::W](W) writer structure"]
impl crate::Writable for CK_GLITCH_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CK_GLITCH_CNTL to value 0"]
impl crate::Resettable for CK_GLITCH_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
