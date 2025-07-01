#[doc = "Register `PHY_TMR_RD_CFG` reader"]
pub type R = crate::R<PHY_TMR_RD_CFG_SPEC>;
#[doc = "Register `PHY_TMR_RD_CFG` writer"]
pub type W = crate::W<PHY_TMR_RD_CFG_SPEC>;
#[doc = "Field `MAX_RD_TIME` reader - NA"]
pub type MAX_RD_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `MAX_RD_TIME` writer - NA"]
pub type MAX_RD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn max_rd_time(&self) -> MAX_RD_TIME_R {
        MAX_RD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_TMR_RD_CFG")
            .field("max_rd_time", &self.max_rd_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn max_rd_time(&mut self) -> MAX_RD_TIME_W<PHY_TMR_RD_CFG_SPEC> {
        MAX_RD_TIME_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_tmr_rd_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_tmr_rd_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_TMR_RD_CFG_SPEC;
impl crate::RegisterSpec for PHY_TMR_RD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tmr_rd_cfg::R`](R) reader structure"]
impl crate::Readable for PHY_TMR_RD_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_tmr_rd_cfg::W`](W) writer structure"]
impl crate::Writable for PHY_TMR_RD_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_TMR_RD_CFG to value 0"]
impl crate::Resettable for PHY_TMR_RD_CFG_SPEC {}
