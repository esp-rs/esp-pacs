#[doc = "Register `DOEPDMAB6` reader"]
pub struct R(crate::R<DOEPDMAB6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMAB6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMAB6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMAB6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMAB6` writer"]
pub struct W(crate::W<DOEPDMAB6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMAB6_SPEC>;
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
impl From<crate::W<DOEPDMAB6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMAB6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMABUFFERADDR6` reader - "]
pub type DMABUFFERADDR6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMABUFFERADDR6` writer - "]
pub type DMABUFFERADDR6_W<'a, const O: u8> =
    crate::FieldWriter<'a, DOEPDMAB6_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr6(&self) -> DMABUFFERADDR6_R {
        DMABUFFERADDR6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB6")
            .field(
                "dmabufferaddr6",
                &format_args!("{}", self.dmabufferaddr6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr6(&mut self) -> DMABUFFERADDR6_W<0> {
        DMABUFFERADDR6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdmab6](index.html) module"]
pub struct DOEPDMAB6_SPEC;
impl crate::RegisterSpec for DOEPDMAB6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdmab6::R](R) reader structure"]
impl crate::Readable for DOEPDMAB6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdmab6::W](W) writer structure"]
impl crate::Writable for DOEPDMAB6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMAB6 to value 0"]
impl crate::Resettable for DOEPDMAB6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
