#[doc = "Register `LPCLK_CTRL` reader"]
pub type R = crate::R<LPCLK_CTRL_SPEC>;
#[doc = "Register `LPCLK_CTRL` writer"]
pub type W = crate::W<LPCLK_CTRL_SPEC>;
#[doc = "Field `PHY_TXREQUESTCLKHS` reader - NA"]
pub type PHY_TXREQUESTCLKHS_R = crate::BitReader;
#[doc = "Field `PHY_TXREQUESTCLKHS` writer - NA"]
pub type PHY_TXREQUESTCLKHS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CLKLANE_CTRL` reader - NA"]
pub type AUTO_CLKLANE_CTRL_R = crate::BitReader;
#[doc = "Field `AUTO_CLKLANE_CTRL` writer - NA"]
pub type AUTO_CLKLANE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_txrequestclkhs(&self) -> PHY_TXREQUESTCLKHS_R {
        PHY_TXREQUESTCLKHS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn auto_clklane_ctrl(&self) -> AUTO_CLKLANE_CTRL_R {
        AUTO_CLKLANE_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCLK_CTRL")
            .field("phy_txrequestclkhs", &self.phy_txrequestclkhs().bit())
            .field("auto_clklane_ctrl", &self.auto_clklane_ctrl().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPCLK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_txrequestclkhs(&mut self) -> PHY_TXREQUESTCLKHS_W<LPCLK_CTRL_SPEC> {
        PHY_TXREQUESTCLKHS_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn auto_clklane_ctrl(&mut self) -> AUTO_CLKLANE_CTRL_W<LPCLK_CTRL_SPEC> {
        AUTO_CLKLANE_CTRL_W::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpclk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpclk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCLK_CTRL_SPEC;
impl crate::RegisterSpec for LPCLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpclk_ctrl::R`](R) reader structure"]
impl crate::Readable for LPCLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpclk_ctrl::W`](W) writer structure"]
impl crate::Writable for LPCLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPCLK_CTRL to value 0"]
impl crate::Resettable for LPCLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
