#[doc = "Register `DIEPTSIZ3` reader"]
pub struct R(crate::R<DIEPTSIZ3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTSIZ3` writer"]
pub struct W(crate::W<DIEPTSIZ3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ3_SPEC>;
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
impl From<crate::W<DIEPTSIZ3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERSIZE3` reader - "]
pub type D_XFERSIZE3_R = crate::FieldReader;
#[doc = "Field `D_XFERSIZE3` writer - "]
pub type D_XFERSIZE3_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTSIZ3_SPEC, 7, O>;
#[doc = "Field `D_PKTCNT3` reader - "]
pub type D_PKTCNT3_R = crate::FieldReader;
#[doc = "Field `D_PKTCNT3` writer - "]
pub type D_PKTCNT3_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTSIZ3_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize3(&self) -> D_XFERSIZE3_R {
        D_XFERSIZE3_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt3(&self) -> D_PKTCNT3_R {
        D_PKTCNT3_R::new(((self.bits >> 19) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ3")
            .field(
                "d_xfersize3",
                &format_args!("{}", self.d_xfersize3().bits()),
            )
            .field("d_pktcnt3", &format_args!("{}", self.d_pktcnt3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize3(&mut self) -> D_XFERSIZE3_W<0> {
        D_XFERSIZE3_W::new(self)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt3(&mut self) -> D_PKTCNT3_W<19> {
        D_PKTCNT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz3](index.html) module"]
pub struct DIEPTSIZ3_SPEC;
impl crate::RegisterSpec for DIEPTSIZ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptsiz3::R](R) reader structure"]
impl crate::Readable for DIEPTSIZ3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptsiz3::W](W) writer structure"]
impl crate::Writable for DIEPTSIZ3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ3 to value 0"]
impl crate::Resettable for DIEPTSIZ3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
