#[doc = "Register `DIEPTSIZ1` reader"]
pub struct R(crate::R<DIEPTSIZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTSIZ1` writer"]
pub struct W(crate::W<DIEPTSIZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ1_SPEC>;
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
impl From<crate::W<DIEPTSIZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERSIZE1` reader - "]
pub type D_XFERSIZE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D_XFERSIZE1` writer - "]
pub type D_XFERSIZE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTSIZ1_SPEC, u8, u8, 7, O>;
#[doc = "Field `D_PKTCNT1` reader - "]
pub type D_PKTCNT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D_PKTCNT1` writer - "]
pub type D_PKTCNT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTSIZ1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize1(&self) -> D_XFERSIZE1_R {
        D_XFERSIZE1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt1(&self) -> D_PKTCNT1_R {
        D_PKTCNT1_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize1(&mut self) -> D_XFERSIZE1_W<0> {
        D_XFERSIZE1_W::new(self)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt1(&mut self) -> D_PKTCNT1_W<19> {
        D_PKTCNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz1](index.html) module"]
pub struct DIEPTSIZ1_SPEC;
impl crate::RegisterSpec for DIEPTSIZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptsiz1::R](R) reader structure"]
impl crate::Readable for DIEPTSIZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptsiz1::W](W) writer structure"]
impl crate::Writable for DIEPTSIZ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ1 to value 0"]
impl crate::Resettable for DIEPTSIZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
