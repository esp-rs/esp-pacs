#[doc = "Register `IN` reader"]
pub struct R(crate::R<IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN` writer"]
pub struct W(crate::W<IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_SPEC>;
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
impl From<crate::W<IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_NEXT` reader - GPIO0 ~ 31 input value. Each bit represents a pad input value, 1 for high level and 0 for low level."]
pub type DATA_NEXT_R = crate::FieldReader<u32>;
#[doc = "Field `DATA_NEXT` writer - GPIO0 ~ 31 input value. Each bit represents a pad input value, 1 for high level and 0 for low level."]
pub type DATA_NEXT_W<'a, const O: u8> = crate::FieldWriter<'a, IN_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0 ~ 31 input value. Each bit represents a pad input value, 1 for high level and 0 for low level."]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN")
            .field("data_next", &format_args!("{}", self.data_next().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0 ~ 31 input value. Each bit represents a pad input value, 1 for high level and 0 for low level."]
    #[inline(always)]
    #[must_use]
    pub fn data_next(&mut self) -> DATA_NEXT_W<0> {
        DATA_NEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO0 ~ 31 input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_](index.html) module"]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_::R](R) reader structure"]
impl crate::Readable for IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_::W](W) writer structure"]
impl crate::Writable for IN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
