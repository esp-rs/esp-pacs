#[doc = "Register `W6` reader"]
pub struct R(crate::R<W6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W6` writer"]
pub struct W(crate::W<W6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W6_SPEC>;
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
impl From<crate::W<W6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF6` reader - data buffer"]
pub type BUF6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUF6` writer - data buffer"]
pub type BUF6_W<'a, const O: u8> = crate::FieldWriter<'a, W6_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf6(&self) -> BUF6_R {
        BUF6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W6")
            .field("buf6", &format_args!("{}", self.buf6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf6(&mut self) -> BUF6_W<0> {
        BUF6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI CPU-controlled buffer6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w6](index.html) module"]
pub struct W6_SPEC;
impl crate::RegisterSpec for W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w6::R](R) reader structure"]
impl crate::Readable for W6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w6::W](W) writer structure"]
impl crate::Writable for W6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W6 to value 0"]
impl crate::Resettable for W6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
