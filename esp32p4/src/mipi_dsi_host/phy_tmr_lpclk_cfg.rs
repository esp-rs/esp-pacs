#[doc = "Register `PHY_TMR_LPCLK_CFG` reader"]
pub type R = crate::R<PHY_TMR_LPCLK_CFG_SPEC>;
#[doc = "Register `PHY_TMR_LPCLK_CFG` writer"]
pub type W = crate::W<PHY_TMR_LPCLK_CFG_SPEC>;
#[doc = "Field `PHY_CLKLP2HS_TIME` reader - NA"]
pub type PHY_CLKLP2HS_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLKLP2HS_TIME` writer - NA"]
pub type PHY_CLKLP2HS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_CLKHS2LP_TIME` reader - NA"]
pub type PHY_CLKHS2LP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLKHS2LP_TIME` writer - NA"]
pub type PHY_CLKHS2LP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn phy_clklp2hs_time(&self) -> PHY_CLKLP2HS_TIME_R {
        PHY_CLKLP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - NA"]
    #[inline(always)]
    pub fn phy_clkhs2lp_time(&self) -> PHY_CLKHS2LP_TIME_R {
        PHY_CLKHS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_TMR_LPCLK_CFG")
            .field(
                "phy_clklp2hs_time",
                &format_args!("{}", self.phy_clklp2hs_time().bits()),
            )
            .field(
                "phy_clkhs2lp_time",
                &format_args!("{}", self.phy_clkhs2lp_time().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_TMR_LPCLK_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_clklp2hs_time(&mut self) -> PHY_CLKLP2HS_TIME_W<PHY_TMR_LPCLK_CFG_SPEC> {
        PHY_CLKLP2HS_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_clkhs2lp_time(&mut self) -> PHY_CLKHS2LP_TIME_W<PHY_TMR_LPCLK_CFG_SPEC> {
        PHY_CLKHS2LP_TIME_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_lpclk_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_lpclk_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_TMR_LPCLK_CFG_SPEC;
impl crate::RegisterSpec for PHY_TMR_LPCLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tmr_lpclk_cfg::R`](R) reader structure"]
impl crate::Readable for PHY_TMR_LPCLK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_tmr_lpclk_cfg::W`](W) writer structure"]
impl crate::Writable for PHY_TMR_LPCLK_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_TMR_LPCLK_CFG to value 0"]
impl crate::Resettable for PHY_TMR_LPCLK_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
