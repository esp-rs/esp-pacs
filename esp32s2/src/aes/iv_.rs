#[doc = "Register `IV_%s` reader"]
pub struct R(crate::R<IV__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV_%s` writer"]
pub struct W(crate::W<IV__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV__SPEC>;
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
impl From<crate::W<IV__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV_0` reader - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
pub type IV_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV_0` writer - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
pub type IV_0_W<'a, const O: u8> = crate::FieldWriter<'a, IV__SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
    #[inline(always)]
    pub fn iv_0(&self) -> IV_0_R {
        IV_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_")
            .field("iv_0", &format_args!("{}", self.iv_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IV__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
    #[inline(always)]
    #[must_use]
    pub fn iv_0(&mut self) -> IV_0_W<0> {
        IV_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "initialization vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_](index.html) module"]
pub struct IV__SPEC;
impl crate::RegisterSpec for IV__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv_::R](R) reader structure"]
impl crate::Readable for IV__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv_::W](W) writer structure"]
impl crate::Writable for IV__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IV_%s to value 0"]
impl crate::Resettable for IV__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
