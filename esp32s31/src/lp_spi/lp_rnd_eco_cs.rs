#[doc = "Register `LP_RND_ECO_CS` reader"]
pub type R = crate::R<LP_RND_ECO_CS_SPEC>;
#[doc = "Register `LP_RND_ECO_CS` writer"]
pub type W = crate::W<LP_RND_ECO_CS_SPEC>;
#[doc = "Field `RND_ECO_EN` reader - "]
pub type RND_ECO_EN_R = crate::BitReader;
#[doc = "Field `RND_ECO_EN` writer - "]
pub type RND_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_RND_ECO_RESULT` reader - "]
pub type LP_RND_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rnd_eco_en(&self) -> RND_ECO_EN_R {
        RND_ECO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lp_rnd_eco_result(&self) -> LP_RND_ECO_RESULT_R {
        LP_RND_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_RND_ECO_CS")
            .field("rnd_eco_en", &self.rnd_eco_en())
            .field("lp_rnd_eco_result", &self.lp_rnd_eco_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rnd_eco_en(&mut self) -> RND_ECO_EN_W<'_, LP_RND_ECO_CS_SPEC> {
        RND_ECO_EN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rnd_eco_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rnd_eco_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_RND_ECO_CS_SPEC;
impl crate::RegisterSpec for LP_RND_ECO_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rnd_eco_cs::R`](R) reader structure"]
impl crate::Readable for LP_RND_ECO_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_rnd_eco_cs::W`](W) writer structure"]
impl crate::Writable for LP_RND_ECO_CS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_RND_ECO_CS to value 0"]
impl crate::Resettable for LP_RND_ECO_CS_SPEC {}
