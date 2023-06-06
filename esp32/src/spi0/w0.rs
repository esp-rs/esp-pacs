#[doc = "Register `W0` reader"]
pub struct R(crate::R<W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W0` writer"]
pub struct W(crate::W<W0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W0_SPEC>;
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
impl From<crate::W<W0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF0` reader - data buffer"]
pub type BUF0_R = crate::FieldReader<u32>;
#[doc = "Field `BUF0` writer - data buffer"]
pub type BUF0_W<'a, const O: u8> = crate::FieldWriter<'a, W0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf0(&self) -> BUF0_R {
        BUF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0")
            .field("buf0", &format_args!("{}", self.buf0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf0(&mut self) -> BUF0_W<0> {
        BUF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0](index.html) module"]
pub struct W0_SPEC;
impl crate::RegisterSpec for W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w0::R](R) reader structure"]
impl crate::Readable for W0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w0::W](W) writer structure"]
impl crate::Writable for W0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W0 to value 0"]
impl crate::Resettable for W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
