#[doc = "Register `DIEPTSIZ2` reader"]
pub struct R(crate::R<DIEPTSIZ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTSIZ2` writer"]
pub struct W(crate::W<DIEPTSIZ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ2_SPEC>;
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
impl From<crate::W<DIEPTSIZ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERSIZE2` reader - "]
pub type D_XFERSIZE2_R = crate::FieldReader;
#[doc = "Field `D_XFERSIZE2` writer - "]
pub type D_XFERSIZE2_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTSIZ2_SPEC, 7, O>;
#[doc = "Field `D_PKTCNT2` reader - "]
pub type D_PKTCNT2_R = crate::FieldReader;
#[doc = "Field `D_PKTCNT2` writer - "]
pub type D_PKTCNT2_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTSIZ2_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize2(&self) -> D_XFERSIZE2_R {
        D_XFERSIZE2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt2(&self) -> D_PKTCNT2_R {
        D_PKTCNT2_R::new(((self.bits >> 19) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ2")
            .field(
                "d_xfersize2",
                &format_args!("{}", self.d_xfersize2().bits()),
            )
            .field("d_pktcnt2", &format_args!("{}", self.d_pktcnt2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize2(&mut self) -> D_XFERSIZE2_W<0> {
        D_XFERSIZE2_W::new(self)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt2(&mut self) -> D_PKTCNT2_W<19> {
        D_PKTCNT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz2](index.html) module"]
pub struct DIEPTSIZ2_SPEC;
impl crate::RegisterSpec for DIEPTSIZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptsiz2::R](R) reader structure"]
impl crate::Readable for DIEPTSIZ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptsiz2::W](W) writer structure"]
impl crate::Writable for DIEPTSIZ2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ2 to value 0"]
impl crate::Resettable for DIEPTSIZ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
