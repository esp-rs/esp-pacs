#[doc = "Register `OUT1` reader"]
pub struct R(crate::R<OUT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT1` writer"]
pub struct W(crate::W<OUT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT1_SPEC>;
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
impl From<crate::W<OUT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - GPIO32~39 output value"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - GPIO32~39 output value"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, OUT1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 output value"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT1")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 output value"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out1](index.html) module"]
pub struct OUT1_SPEC;
impl crate::RegisterSpec for OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out1::R](R) reader structure"]
impl crate::Readable for OUT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out1::W](W) writer structure"]
impl crate::Writable for OUT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT1 to value 0"]
impl crate::Resettable for OUT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
