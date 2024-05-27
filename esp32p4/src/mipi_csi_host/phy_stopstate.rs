#[doc = "Register `PHY_STOPSTATE` reader"]
pub type R = crate::R<PHY_STOPSTATE_SPEC>;
#[doc = "Field `PHY_STOPSTATEDATA_0` reader - NA"]
pub type PHY_STOPSTATEDATA_0_R = crate::BitReader;
#[doc = "Field `PHY_STOPSTATEDATA_1` reader - NA"]
pub type PHY_STOPSTATEDATA_1_R = crate::BitReader;
#[doc = "Field `PHY_STOPSTATECLK` reader - NA"]
pub type PHY_STOPSTATECLK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_stopstatedata_0(&self) -> PHY_STOPSTATEDATA_0_R {
        PHY_STOPSTATEDATA_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_stopstatedata_1(&self) -> PHY_STOPSTATEDATA_1_R {
        PHY_STOPSTATEDATA_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn phy_stopstateclk(&self) -> PHY_STOPSTATECLK_R {
        PHY_STOPSTATECLK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_STOPSTATE")
            .field("phy_stopstatedata_0", &self.phy_stopstatedata_0())
            .field("phy_stopstatedata_1", &self.phy_stopstatedata_1())
            .field("phy_stopstateclk", &self.phy_stopstateclk())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_stopstate::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_STOPSTATE_SPEC;
impl crate::RegisterSpec for PHY_STOPSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_stopstate::R`](R) reader structure"]
impl crate::Readable for PHY_STOPSTATE_SPEC {}
#[doc = "`reset()` method sets PHY_STOPSTATE to value 0"]
impl crate::Resettable for PHY_STOPSTATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
