#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `PREP_DONE` writer - Set this bit to clear the ecdsa_prep_done_int interrupt"]
pub type PREP_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PROC_DONE` writer - Set this bit to clear the ecdsa_proc_done_int interrupt"]
pub type PROC_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `POST_DONE` writer - Set this bit to clear the ecdsa_post_done_int interrupt"]
pub type POST_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SHA_RELEASE` writer - Set this bit to clear the ecdsa_sha_release_int interrupt"]
pub type SHA_RELEASE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the ecdsa_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done(&mut self) -> PREP_DONE_W<INT_CLR_SPEC> {
        PREP_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the ecdsa_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done(&mut self) -> PROC_DONE_W<INT_CLR_SPEC> {
        PROC_DONE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the ecdsa_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done(&mut self) -> POST_DONE_W<INT_CLR_SPEC> {
        POST_DONE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the ecdsa_sha_release_int interrupt"]
    #[inline(always)]
    pub fn sha_release(&mut self) -> SHA_RELEASE_W<INT_CLR_SPEC> {
        SHA_RELEASE_W::new(self, 3)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
