///Register `RND_ECO_CS` reader
pub type R = crate::R<RND_ECO_CS_SPEC>;
///Register `RND_ECO_CS` writer
pub type W = crate::W<RND_ECO_CS_SPEC>;
///Field `REG_RND_ECO_EN` reader - NA
pub type REG_RND_ECO_EN_R = crate::BitReader;
///Field `REG_RND_ECO_EN` writer - NA
pub type REG_RND_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RND_ECO_RESULT` reader - NA
pub type RND_ECO_RESULT_R = crate::BitReader;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn reg_rnd_eco_en(&self) -> REG_RND_ECO_EN_R {
        REG_RND_ECO_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    pub fn rnd_eco_result(&self) -> RND_ECO_RESULT_R {
        RND_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RND_ECO_CS")
            .field("reg_rnd_eco_en", &self.reg_rnd_eco_en())
            .field("rnd_eco_result", &self.rnd_eco_result())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_rnd_eco_en(&mut self) -> REG_RND_ECO_EN_W<RND_ECO_CS_SPEC> {
        REG_RND_ECO_EN_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RND_ECO_CS_SPEC;
impl crate::RegisterSpec for RND_ECO_CS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rnd_eco_cs::R`](R) reader structure
impl crate::Readable for RND_ECO_CS_SPEC {}
///`write(|w| ..)` method takes [`rnd_eco_cs::W`](W) writer structure
impl crate::Writable for RND_ECO_CS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RND_ECO_CS to value 0
impl crate::Resettable for RND_ECO_CS_SPEC {
    const RESET_VALUE: u32 = 0;
}
