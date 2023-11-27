#[doc = "Register `PHY_RX` reader"]
pub type R = crate::R<PHY_RX_SPEC>;
#[doc = "Field `PHY_RXULPSESC_0` reader - NA"]
pub type PHY_RXULPSESC_0_R = crate::BitReader;
#[doc = "Field `PHY_RXULPSESC_1` reader - NA"]
pub type PHY_RXULPSESC_1_R = crate::BitReader;
#[doc = "Field `PHY_RXULPSCLKNOT` reader - NA"]
pub type PHY_RXULPSCLKNOT_R = crate::BitReader;
#[doc = "Field `PHY_RXCLKACTIVEHS` reader - NA"]
pub type PHY_RXCLKACTIVEHS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_rxulpsesc_0(&self) -> PHY_RXULPSESC_0_R {
        PHY_RXULPSESC_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_rxulpsesc_1(&self) -> PHY_RXULPSESC_1_R {
        PHY_RXULPSESC_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn phy_rxulpsclknot(&self) -> PHY_RXULPSCLKNOT_R {
        PHY_RXULPSCLKNOT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn phy_rxclkactivehs(&self) -> PHY_RXCLKACTIVEHS_R {
        PHY_RXCLKACTIVEHS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_RX")
            .field(
                "phy_rxulpsesc_0",
                &format_args!("{}", self.phy_rxulpsesc_0().bit()),
            )
            .field(
                "phy_rxulpsesc_1",
                &format_args!("{}", self.phy_rxulpsesc_1().bit()),
            )
            .field(
                "phy_rxulpsclknot",
                &format_args!("{}", self.phy_rxulpsclknot().bit()),
            )
            .field(
                "phy_rxclkactivehs",
                &format_args!("{}", self.phy_rxclkactivehs().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_RX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_rx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_RX_SPEC;
impl crate::RegisterSpec for PHY_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_rx::R`](R) reader structure"]
impl crate::Readable for PHY_RX_SPEC {}
#[doc = "`reset()` method sets PHY_RX to value 0x0001_0000"]
impl crate::Resettable for PHY_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
