#[doc = "Register `J0_%s` reader"]
pub struct R(crate::R<J0__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<J0__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<J0__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<J0__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `J0_%s` writer"]
pub struct W(crate::W<J0__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<J0__SPEC>;
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
impl From<crate::W<J0__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<J0__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `J0_0` reader - This register stores the %sth 32-bit piece of 128-bit J0"]
pub type J0_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `J0_0` writer - This register stores the %sth 32-bit piece of 128-bit J0"]
pub type J0_0_W<'a, const O: u8> = crate::FieldWriter<'a, J0__SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit J0"]
    #[inline(always)]
    pub fn j0_0(&self) -> J0_0_R {
        J0_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("J0_")
            .field("j0_0", &format_args!("{}", self.j0_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<J0__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit J0"]
    #[inline(always)]
    #[must_use]
    pub fn j0_0(&mut self) -> J0_0_W<0> {
        J0_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "J0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [j0_](index.html) module"]
pub struct J0__SPEC;
impl crate::RegisterSpec for J0__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [j0_::R](R) reader structure"]
impl crate::Readable for J0__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [j0_::W](W) writer structure"]
impl crate::Writable for J0__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets J0_%s to value 0"]
impl crate::Resettable for J0__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
