#[doc = "Register `DIEPTSIZ6` reader"]
pub struct R(crate::R<DIEPTSIZ6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTSIZ6` writer"]
pub struct W(crate::W<DIEPTSIZ6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ6_SPEC>;
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
impl From<crate::W<DIEPTSIZ6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERSIZE6` reader - "]
pub type D_XFERSIZE6_R = crate::FieldReader;
#[doc = "Field `D_XFERSIZE6` writer - "]
pub type D_XFERSIZE6_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTSIZ6_SPEC, 7, O>;
#[doc = "Field `D_PKTCNT6` reader - "]
pub type D_PKTCNT6_R = crate::FieldReader;
#[doc = "Field `D_PKTCNT6` writer - "]
pub type D_PKTCNT6_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTSIZ6_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize6(&self) -> D_XFERSIZE6_R {
        D_XFERSIZE6_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt6(&self) -> D_PKTCNT6_R {
        D_PKTCNT6_R::new(((self.bits >> 19) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ6")
            .field(
                "d_xfersize6",
                &format_args!("{}", self.d_xfersize6().bits()),
            )
            .field("d_pktcnt6", &format_args!("{}", self.d_pktcnt6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize6(&mut self) -> D_XFERSIZE6_W<0> {
        D_XFERSIZE6_W::new(self)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt6(&mut self) -> D_PKTCNT6_W<19> {
        D_PKTCNT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz6](index.html) module"]
pub struct DIEPTSIZ6_SPEC;
impl crate::RegisterSpec for DIEPTSIZ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptsiz6::R](R) reader structure"]
impl crate::Readable for DIEPTSIZ6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptsiz6::W](W) writer structure"]
impl crate::Writable for DIEPTSIZ6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ6 to value 0"]
impl crate::Resettable for DIEPTSIZ6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
