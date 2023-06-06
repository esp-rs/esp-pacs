#[doc = "Register `HPTXFSIZ` reader"]
pub struct R(crate::R<HPTXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPTXFSIZ` writer"]
pub struct W(crate::W<HPTXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPTXFSIZ_SPEC>;
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
impl From<crate::W<HPTXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPTXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTXFSTADDR` reader - "]
pub type PTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSTADDR` writer - "]
pub type PTXFSTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, HPTXFSIZ_SPEC, 16, O, u16>;
#[doc = "Field `PTXFSIZE` reader - "]
pub type PTXFSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZE` writer - "]
pub type PTXFSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, HPTXFSIZ_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ptxfstaddr(&self) -> PTXFSTADDR_R {
        PTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ptxfsize(&self) -> PTXFSIZE_R {
        PTXFSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXFSIZ")
            .field("ptxfstaddr", &format_args!("{}", self.ptxfstaddr().bits()))
            .field("ptxfsize", &format_args!("{}", self.ptxfsize().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HPTXFSIZ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfstaddr(&mut self) -> PTXFSTADDR_W<0> {
        PTXFSTADDR_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfsize(&mut self) -> PTXFSIZE_W<16> {
        PTXFSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxfsiz](index.html) module"]
pub struct HPTXFSIZ_SPEC;
impl crate::RegisterSpec for HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hptxfsiz::R](R) reader structure"]
impl crate::Readable for HPTXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hptxfsiz::W](W) writer structure"]
impl crate::Writable for HPTXFSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x1000_0200"]
impl crate::Resettable for HPTXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0200;
}
