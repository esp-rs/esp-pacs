#[doc = "Register `W10` reader"]
pub struct R(crate::R<W10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W10` writer"]
pub struct W(crate::W<W10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W10_SPEC>;
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
impl From<crate::W<W10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF10` reader - data buffer"]
pub type BUF10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUF10` writer - data buffer"]
pub type BUF10_W<'a, const O: u8> = crate::FieldWriter<'a, W10_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf10(&self) -> BUF10_R {
        BUF10_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W10")
            .field("buf10", &format_args!("{}", self.buf10().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf10(&mut self) -> BUF10_W<0> {
        BUF10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w10](index.html) module"]
pub struct W10_SPEC;
impl crate::RegisterSpec for W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w10::R](R) reader structure"]
impl crate::Readable for W10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w10::W](W) writer structure"]
impl crate::Writable for W10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W10 to value 0"]
impl crate::Resettable for W10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
