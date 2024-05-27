///Register `PHY_TMR_CFG` reader
pub type R = crate::R<PHY_TMR_CFG_SPEC>;
///Register `PHY_TMR_CFG` writer
pub type W = crate::W<PHY_TMR_CFG_SPEC>;
///Field `PHY_LP2HS_TIME` reader - NA
pub type PHY_LP2HS_TIME_R = crate::FieldReader<u16>;
///Field `PHY_LP2HS_TIME` writer - NA
pub type PHY_LP2HS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `PHY_HS2LP_TIME` reader - NA
pub type PHY_HS2LP_TIME_R = crate::FieldReader<u16>;
///Field `PHY_HS2LP_TIME` writer - NA
pub type PHY_HS2LP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - NA
    #[inline(always)]
    pub fn phy_lp2hs_time(&self) -> PHY_LP2HS_TIME_R {
        PHY_LP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - NA
    #[inline(always)]
    pub fn phy_hs2lp_time(&self) -> PHY_HS2LP_TIME_R {
        PHY_HS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_TMR_CFG")
            .field("phy_lp2hs_time", &self.phy_lp2hs_time())
            .field("phy_hs2lp_time", &self.phy_hs2lp_time())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - NA
    #[inline(always)]
    #[must_use]
    pub fn phy_lp2hs_time(&mut self) -> PHY_LP2HS_TIME_W<PHY_TMR_CFG_SPEC> {
        PHY_LP2HS_TIME_W::new(self, 0)
    }
    ///Bits 16:25 - NA
    #[inline(always)]
    #[must_use]
    pub fn phy_hs2lp_time(&mut self) -> PHY_HS2LP_TIME_W<PHY_TMR_CFG_SPEC> {
        PHY_HS2LP_TIME_W::new(self, 16)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PHY_TMR_CFG_SPEC;
impl crate::RegisterSpec for PHY_TMR_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`phy_tmr_cfg::R`](R) reader structure
impl crate::Readable for PHY_TMR_CFG_SPEC {}
///`write(|w| ..)` method takes [`phy_tmr_cfg::W`](W) writer structure
impl crate::Writable for PHY_TMR_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PHY_TMR_CFG to value 0
impl crate::Resettable for PHY_TMR_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
