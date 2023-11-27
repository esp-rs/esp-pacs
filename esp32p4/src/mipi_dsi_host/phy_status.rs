#[doc = "Register `PHY_STATUS` reader"]
pub type R = crate::R<PHY_STATUS_SPEC>;
#[doc = "Field `PHY_LOCK` reader - NA"]
pub type PHY_LOCK_R = crate::BitReader;
#[doc = "Field `PHY_DIRECTION` reader - NA"]
pub type PHY_DIRECTION_R = crate::BitReader;
#[doc = "Field `PHY_STOPSTATECLKLANE` reader - NA"]
pub type PHY_STOPSTATECLKLANE_R = crate::BitReader;
#[doc = "Field `PHY_ULPSACTIVENOTCLK` reader - NA"]
pub type PHY_ULPSACTIVENOTCLK_R = crate::BitReader;
#[doc = "Field `PHY_STOPSTATE0LANE` reader - NA"]
pub type PHY_STOPSTATE0LANE_R = crate::BitReader;
#[doc = "Field `PHY_ULPSACTIVENOT0LANE` reader - NA"]
pub type PHY_ULPSACTIVENOT0LANE_R = crate::BitReader;
#[doc = "Field `PHY_RXULPSESC0LANE` reader - NA"]
pub type PHY_RXULPSESC0LANE_R = crate::BitReader;
#[doc = "Field `PHY_STOPSTATE1LANE` reader - NA"]
pub type PHY_STOPSTATE1LANE_R = crate::BitReader;
#[doc = "Field `PHY_ULPSACTIVENOT1LANE` reader - NA"]
pub type PHY_ULPSACTIVENOT1LANE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_lock(&self) -> PHY_LOCK_R {
        PHY_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_direction(&self) -> PHY_DIRECTION_R {
        PHY_DIRECTION_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn phy_stopstateclklane(&self) -> PHY_STOPSTATECLKLANE_R {
        PHY_STOPSTATECLKLANE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn phy_ulpsactivenotclk(&self) -> PHY_ULPSACTIVENOTCLK_R {
        PHY_ULPSACTIVENOTCLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn phy_stopstate0lane(&self) -> PHY_STOPSTATE0LANE_R {
        PHY_STOPSTATE0LANE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn phy_ulpsactivenot0lane(&self) -> PHY_ULPSACTIVENOT0LANE_R {
        PHY_ULPSACTIVENOT0LANE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn phy_rxulpsesc0lane(&self) -> PHY_RXULPSESC0LANE_R {
        PHY_RXULPSESC0LANE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn phy_stopstate1lane(&self) -> PHY_STOPSTATE1LANE_R {
        PHY_STOPSTATE1LANE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn phy_ulpsactivenot1lane(&self) -> PHY_ULPSACTIVENOT1LANE_R {
        PHY_ULPSACTIVENOT1LANE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_STATUS")
            .field("phy_lock", &format_args!("{}", self.phy_lock().bit()))
            .field(
                "phy_direction",
                &format_args!("{}", self.phy_direction().bit()),
            )
            .field(
                "phy_stopstateclklane",
                &format_args!("{}", self.phy_stopstateclklane().bit()),
            )
            .field(
                "phy_ulpsactivenotclk",
                &format_args!("{}", self.phy_ulpsactivenotclk().bit()),
            )
            .field(
                "phy_stopstate0lane",
                &format_args!("{}", self.phy_stopstate0lane().bit()),
            )
            .field(
                "phy_ulpsactivenot0lane",
                &format_args!("{}", self.phy_ulpsactivenot0lane().bit()),
            )
            .field(
                "phy_rxulpsesc0lane",
                &format_args!("{}", self.phy_rxulpsesc0lane().bit()),
            )
            .field(
                "phy_stopstate1lane",
                &format_args!("{}", self.phy_stopstate1lane().bit()),
            )
            .field(
                "phy_ulpsactivenot1lane",
                &format_args!("{}", self.phy_ulpsactivenot1lane().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_STATUS_SPEC;
impl crate::RegisterSpec for PHY_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_status::R`](R) reader structure"]
impl crate::Readable for PHY_STATUS_SPEC {}
#[doc = "`reset()` method sets PHY_STATUS to value 0x0140"]
impl crate::Resettable for PHY_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0140;
}
