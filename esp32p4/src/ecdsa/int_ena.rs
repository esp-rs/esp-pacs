#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CALC_DONE` reader - The interrupt enable bit for the ecdsa_calc_done_int interrupt"]
pub type CALC_DONE_R = crate::BitReader;
#[doc = "Field `CALC_DONE` writer - The interrupt enable bit for the ecdsa_calc_done_int interrupt"]
pub type CALC_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA_RELEASE` reader - The interrupt enable bit for the ecdsa_sha_release_int interrupt"]
pub type SHA_RELEASE_R = crate::BitReader;
#[doc = "Field `SHA_RELEASE` writer - The interrupt enable bit for the ecdsa_sha_release_int interrupt"]
pub type SHA_RELEASE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the ecdsa_calc_done_int interrupt"]
    #[inline(always)]
    pub fn calc_done(&self) -> CALC_DONE_R {
        CALC_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the ecdsa_sha_release_int interrupt"]
    #[inline(always)]
    pub fn sha_release(&self) -> SHA_RELEASE_R {
        SHA_RELEASE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("calc_done", &format_args!("{}", self.calc_done().bit()))
            .field("sha_release", &format_args!("{}", self.sha_release().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the ecdsa_calc_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn calc_done(&mut self) -> CALC_DONE_W<INT_ENA_SPEC> {
        CALC_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the ecdsa_sha_release_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sha_release(&mut self) -> SHA_RELEASE_W<INT_ENA_SPEC> {
        SHA_RELEASE_W::new(self, 1)
    }
}
#[doc = "ECDSA interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
