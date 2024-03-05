#[doc = "Register `PHY_ULPS_CTRL` reader"]
pub type R = crate::R<PHY_ULPS_CTRL_SPEC>;
#[doc = "Register `PHY_ULPS_CTRL` writer"]
pub type W = crate::W<PHY_ULPS_CTRL_SPEC>;
#[doc = "Field `PHY_TXREQULPSCLK` reader - NA"]
pub type PHY_TXREQULPSCLK_R = crate::BitReader;
#[doc = "Field `PHY_TXREQULPSCLK` writer - NA"]
pub type PHY_TXREQULPSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TXEXITULPSCLK` reader - NA"]
pub type PHY_TXEXITULPSCLK_R = crate::BitReader;
#[doc = "Field `PHY_TXEXITULPSCLK` writer - NA"]
pub type PHY_TXEXITULPSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TXREQULPSLAN` reader - NA"]
pub type PHY_TXREQULPSLAN_R = crate::BitReader;
#[doc = "Field `PHY_TXREQULPSLAN` writer - NA"]
pub type PHY_TXREQULPSLAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TXEXITULPSLAN` reader - NA"]
pub type PHY_TXEXITULPSLAN_R = crate::BitReader;
#[doc = "Field `PHY_TXEXITULPSLAN` writer - NA"]
pub type PHY_TXEXITULPSLAN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_txrequlpsclk(&self) -> PHY_TXREQULPSCLK_R {
        PHY_TXREQULPSCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_txexitulpsclk(&self) -> PHY_TXEXITULPSCLK_R {
        PHY_TXEXITULPSCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn phy_txrequlpslan(&self) -> PHY_TXREQULPSLAN_R {
        PHY_TXREQULPSLAN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn phy_txexitulpslan(&self) -> PHY_TXEXITULPSLAN_R {
        PHY_TXEXITULPSLAN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_ULPS_CTRL")
            .field(
                "phy_txrequlpsclk",
                &format_args!("{}", self.phy_txrequlpsclk().bit()),
            )
            .field(
                "phy_txexitulpsclk",
                &format_args!("{}", self.phy_txexitulpsclk().bit()),
            )
            .field(
                "phy_txrequlpslan",
                &format_args!("{}", self.phy_txrequlpslan().bit()),
            )
            .field(
                "phy_txexitulpslan",
                &format_args!("{}", self.phy_txexitulpslan().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_ULPS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_txrequlpsclk(&mut self) -> PHY_TXREQULPSCLK_W<PHY_ULPS_CTRL_SPEC> {
        PHY_TXREQULPSCLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_txexitulpsclk(&mut self) -> PHY_TXEXITULPSCLK_W<PHY_ULPS_CTRL_SPEC> {
        PHY_TXEXITULPSCLK_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_txrequlpslan(&mut self) -> PHY_TXREQULPSLAN_W<PHY_ULPS_CTRL_SPEC> {
        PHY_TXREQULPSLAN_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_txexitulpslan(&mut self) -> PHY_TXEXITULPSLAN_W<PHY_ULPS_CTRL_SPEC> {
        PHY_TXEXITULPSLAN_W::new(self, 3)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ulps_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ulps_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_ULPS_CTRL_SPEC;
impl crate::RegisterSpec for PHY_ULPS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_ulps_ctrl::R`](R) reader structure"]
impl crate::Readable for PHY_ULPS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_ulps_ctrl::W`](W) writer structure"]
impl crate::Writable for PHY_ULPS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_ULPS_CTRL to value 0"]
impl crate::Resettable for PHY_ULPS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
