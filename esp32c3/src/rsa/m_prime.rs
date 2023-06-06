#[doc = "Register `M_PRIME` reader"]
pub struct R(crate::R<M_PRIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M_PRIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M_PRIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M_PRIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M_PRIME` writer"]
pub struct W(crate::W<M_PRIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M_PRIME_SPEC>;
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
impl From<crate::W<M_PRIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M_PRIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_PRIME` reader - Those bits stores m'"]
pub type M_PRIME_R = crate::FieldReader<u32>;
#[doc = "Field `M_PRIME` writer - Those bits stores m'"]
pub type M_PRIME_W<'a, const O: u8> = crate::FieldWriter<'a, M_PRIME_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores m'"]
    #[inline(always)]
    pub fn m_prime(&self) -> M_PRIME_R {
        M_PRIME_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M_PRIME")
            .field("m_prime", &format_args!("{}", self.m_prime().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M_PRIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores m'"]
    #[inline(always)]
    #[must_use]
    pub fn m_prime(&mut self) -> M_PRIME_W<0> {
        M_PRIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSA M_prime register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m_prime](index.html) module"]
pub struct M_PRIME_SPEC;
impl crate::RegisterSpec for M_PRIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m_prime::R](R) reader structure"]
impl crate::Readable for M_PRIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m_prime::W](W) writer structure"]
impl crate::Writable for M_PRIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M_PRIME to value 0"]
impl crate::Resettable for M_PRIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
