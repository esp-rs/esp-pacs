#[doc = "Register `DESTROY` writer"]
pub type W = crate::W<DESTROY_SPEC>;
#[doc = "Field `DESTROY` writer - Write 1 to destroy encrypted result."]
pub type DESTROY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DESTROY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to destroy encrypted result."]
    #[inline(always)]
    #[must_use]
    pub fn destroy(&mut self) -> DESTROY_W<DESTROY_SPEC, 0> {
        DESTROY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "XTS-AES destroy control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destroy::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESTROY_SPEC;
impl crate::RegisterSpec for DESTROY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`destroy::W`](W) writer structure"]
impl crate::Writable for DESTROY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTROY to value 0"]
impl crate::Resettable for DESTROY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
