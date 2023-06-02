#[doc = "Register `TIMER_LOAD` reader"]
pub struct R(crate::R<TIMER_LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_LOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_LOAD` writer"]
pub struct W(crate::W<TIMER_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_LOAD_SPEC>;
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
impl From<crate::W<TIMER_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - "]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `VALUE` writer - "]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER_LOAD_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_LOAD")
            .field("value", &format_args!("{}", self.value().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_load](index.html) module"]
pub struct TIMER_LOAD_SPEC;
impl crate::RegisterSpec for TIMER_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_load::R](R) reader structure"]
impl crate::Readable for TIMER_LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_load::W](W) writer structure"]
impl crate::Writable for TIMER_LOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_LOAD to value 0"]
impl crate::Resettable for TIMER_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
