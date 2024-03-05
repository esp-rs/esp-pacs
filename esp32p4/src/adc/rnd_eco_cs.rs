#[doc = "Register `RND_ECO_CS` reader"]
pub type R = crate::R<RND_ECO_CS_SPEC>;
#[doc = "Register `RND_ECO_CS` writer"]
pub type W = crate::W<RND_ECO_CS_SPEC>;
#[doc = "Field `RND_ECO_EN` reader - need_des"]
pub type RND_ECO_EN_R = crate::BitReader;
#[doc = "Field `RND_ECO_EN` writer - need_des"]
pub type RND_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RND_ECO_RESULT` reader - need_des"]
pub type RND_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn rnd_eco_en(&self) -> RND_ECO_EN_R {
        RND_ECO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn rnd_eco_result(&self) -> RND_ECO_RESULT_R {
        RND_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RND_ECO_CS")
            .field("rnd_eco_en", &format_args!("{}", self.rnd_eco_en().bit()))
            .field(
                "rnd_eco_result",
                &format_args!("{}", self.rnd_eco_result().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RND_ECO_CS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_eco_en(&mut self) -> RND_ECO_EN_W<RND_ECO_CS_SPEC> {
        RND_ECO_EN_W::new(self, 0)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RND_ECO_CS_SPEC;
impl crate::RegisterSpec for RND_ECO_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnd_eco_cs::R`](R) reader structure"]
impl crate::Readable for RND_ECO_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rnd_eco_cs::W`](W) writer structure"]
impl crate::Writable for RND_ECO_CS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RND_ECO_CS to value 0"]
impl crate::Resettable for RND_ECO_CS_SPEC {
    const RESET_VALUE: u32 = 0;
}
