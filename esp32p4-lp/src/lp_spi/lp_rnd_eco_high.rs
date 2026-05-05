#[doc = "Register `LP_RND_ECO_HIGH` reader"]
pub type R = crate::R<LP_RND_ECO_HIGH_SPEC>;
#[doc = "Register `LP_RND_ECO_HIGH` writer"]
pub type W = crate::W<LP_RND_ECO_HIGH_SPEC>;
#[doc = "Field `LP_REG_RND_ECO_HIGH` reader - NA"]
pub type LP_REG_RND_ECO_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `LP_REG_RND_ECO_HIGH` writer - NA"]
pub type LP_REG_RND_ECO_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn lp_reg_rnd_eco_high(&self) -> LP_REG_RND_ECO_HIGH_R {
        LP_REG_RND_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_RND_ECO_HIGH")
            .field("lp_reg_rnd_eco_high", &self.lp_reg_rnd_eco_high())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn lp_reg_rnd_eco_high(&mut self) -> LP_REG_RND_ECO_HIGH_W<'_, LP_RND_ECO_HIGH_SPEC> {
        LP_REG_RND_ECO_HIGH_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rnd_eco_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rnd_eco_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_RND_ECO_HIGH_SPEC;
impl crate::RegisterSpec for LP_RND_ECO_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rnd_eco_high::R`](R) reader structure"]
impl crate::Readable for LP_RND_ECO_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_rnd_eco_high::W`](W) writer structure"]
impl crate::Writable for LP_RND_ECO_HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_RND_ECO_HIGH to value 0"]
impl crate::Resettable for LP_RND_ECO_HIGH_SPEC {}
