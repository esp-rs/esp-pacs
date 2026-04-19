#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `PREP_DONE` reader - The interrupt enable bit for the ecdsa_prep_done_int interrupt"]
pub type PREP_DONE_R = crate::BitReader;
#[doc = "Field `PREP_DONE` writer - The interrupt enable bit for the ecdsa_prep_done_int interrupt"]
pub type PREP_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC_DONE` reader - The interrupt enable bit for the ecdsa_proc_done_int interrupt"]
pub type PROC_DONE_R = crate::BitReader;
#[doc = "Field `PROC_DONE` writer - The interrupt enable bit for the ecdsa_proc_done_int interrupt"]
pub type PROC_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POST_DONE` reader - The interrupt enable bit for the ecdsa_post_done_int interrupt"]
pub type POST_DONE_R = crate::BitReader;
#[doc = "Field `POST_DONE` writer - The interrupt enable bit for the ecdsa_post_done_int interrupt"]
pub type POST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA_RELEASE` reader - The interrupt enable bit for the ecdsa_sha_release_int interrupt"]
pub type SHA_RELEASE_R = crate::BitReader;
#[doc = "Field `SHA_RELEASE` writer - The interrupt enable bit for the ecdsa_sha_release_int interrupt"]
pub type SHA_RELEASE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the ecdsa_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done(&self) -> PREP_DONE_R {
        PREP_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the ecdsa_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done(&self) -> PROC_DONE_R {
        PROC_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the ecdsa_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done(&self) -> POST_DONE_R {
        POST_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the ecdsa_sha_release_int interrupt"]
    #[inline(always)]
    pub fn sha_release(&self) -> SHA_RELEASE_R {
        SHA_RELEASE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("prep_done", &self.prep_done())
            .field("proc_done", &self.proc_done())
            .field("post_done", &self.post_done())
            .field("sha_release", &self.sha_release())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the ecdsa_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done(&mut self) -> PREP_DONE_W<'_, INT_ENA_SPEC> {
        PREP_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the ecdsa_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done(&mut self) -> PROC_DONE_W<'_, INT_ENA_SPEC> {
        PROC_DONE_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the ecdsa_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done(&mut self) -> POST_DONE_W<'_, INT_ENA_SPEC> {
        POST_DONE_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the ecdsa_sha_release_int interrupt"]
    #[inline(always)]
    pub fn sha_release(&mut self) -> SHA_RELEASE_W<'_, INT_ENA_SPEC> {
        SHA_RELEASE_W::new(self, 3)
    }
}
#[doc = "ECDSA interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
