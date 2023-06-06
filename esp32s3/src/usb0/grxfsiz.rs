#[doc = "Register `GRXFSIZ` reader"]
pub struct R(crate::R<GRXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRXFSIZ` writer"]
pub struct W(crate::W<GRXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRXFSIZ_SPEC>;
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
impl From<crate::W<GRXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFDEP` reader - "]
pub type RXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `RXFDEP` writer - "]
pub type RXFDEP_W<'a, const O: u8> = crate::FieldWriter<'a, GRXFSIZ_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rxfdep(&self) -> RXFDEP_R {
        RXFDEP_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXFSIZ")
            .field("rxfdep", &format_args!("{}", self.rxfdep().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GRXFSIZ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rxfdep(&mut self) -> RXFDEP_W<0> {
        RXFDEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxfsiz](index.html) module"]
pub struct GRXFSIZ_SPEC;
impl crate::RegisterSpec for GRXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grxfsiz::R](R) reader structure"]
impl crate::Readable for GRXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grxfsiz::W](W) writer structure"]
impl crate::Writable for GRXFSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRXFSIZ to value 0x0100"]
impl crate::Resettable for GRXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
