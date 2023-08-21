#[doc = "Register `M_PRIME` reader"]
pub type R = crate::R<M_PRIME_SPEC>;
#[doc = "Register `M_PRIME` writer"]
pub type W = crate::W<M_PRIME_SPEC>;
#[doc = "Field `M_PRIME` reader - Stores M'"]
pub type M_PRIME_R = crate::FieldReader<u32>;
#[doc = "Field `M_PRIME` writer - Stores M'"]
pub type M_PRIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores M'"]
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
    #[doc = "Bits 0:31 - Stores M'"]
    #[inline(always)]
    #[must_use]
    pub fn m_prime(&mut self) -> M_PRIME_W<M_PRIME_SPEC, 0> {
        M_PRIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RSA M' register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_prime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_prime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M_PRIME_SPEC;
impl crate::RegisterSpec for M_PRIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_prime::R`](R) reader structure"]
impl crate::Readable for M_PRIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m_prime::W`](W) writer structure"]
impl crate::Writable for M_PRIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M_PRIME to value 0"]
impl crate::Resettable for M_PRIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
