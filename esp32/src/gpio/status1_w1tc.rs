#[doc = "Register `STATUS1_W1TC` reader"]
pub struct R(crate::R<STATUS1_W1TC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS1_W1TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS1_W1TC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS1_W1TC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS1_W1TC` writer"]
pub struct W(crate::W<STATUS1_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS1_W1TC_SPEC>;
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
impl From<crate::W<STATUS1_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS1_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS1_INT_W1TC` reader - GPIO32~39 interrupt status write 1 to clear"]
pub type STATUS1_INT_W1TC_R = crate::FieldReader;
#[doc = "Field `STATUS1_INT_W1TC` writer - GPIO32~39 interrupt status write 1 to clear"]
pub type STATUS1_INT_W1TC_W<'a, const O: u8> = crate::FieldWriter<'a, STATUS1_W1TC_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to clear"]
    #[inline(always)]
    pub fn status1_int_w1tc(&self) -> STATUS1_INT_W1TC_R {
        STATUS1_INT_W1TC_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS1_W1TC")
            .field(
                "status1_int_w1tc",
                &format_args!("{}", self.status1_int_w1tc().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS1_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn status1_int_w1tc(&mut self) -> STATUS1_INT_W1TC_W<0> {
        STATUS1_INT_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status1_w1tc](index.html) module"]
pub struct STATUS1_W1TC_SPEC;
impl crate::RegisterSpec for STATUS1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status1_w1tc::R](R) reader structure"]
impl crate::Readable for STATUS1_W1TC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status1_w1tc::W](W) writer structure"]
impl crate::Writable for STATUS1_W1TC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS1_W1TC to value 0"]
impl crate::Resettable for STATUS1_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
