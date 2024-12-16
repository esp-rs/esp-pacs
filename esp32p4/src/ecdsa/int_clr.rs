#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `CALC_DONE` writer - Set this bit to clear the ecdsa_calc_done_int interrupt"]
pub type CALC_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SHA_RELEASE` writer - Set this bit to clear the ecdsa_sha_release_int interrupt"]
pub type SHA_RELEASE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the ecdsa_calc_done_int interrupt"]
    #[inline(always)]
    pub fn calc_done(&mut self) -> CALC_DONE_W<INT_CLR_SPEC> {
        CALC_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the ecdsa_sha_release_int interrupt"]
    #[inline(always)]
    pub fn sha_release(&mut self) -> SHA_RELEASE_W<INT_CLR_SPEC> {
        SHA_RELEASE_W::new(self, 1)
    }
}
#[doc = "ECDSA interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
