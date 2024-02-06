#[doc = "Register `INT_ST_PHY` reader"]
pub type R = crate::R<INT_ST_PHY_SPEC>;
#[doc = "Field `ST_PHY_ERRSOTHS_0` reader - NA"]
pub type ST_PHY_ERRSOTHS_0_R = crate::BitReader;
#[doc = "Field `ST_PHY_ERRSOTHS_1` reader - NA"]
pub type ST_PHY_ERRSOTHS_1_R = crate::BitReader;
#[doc = "Field `ST_PHY_ERRESC_0` reader - NA"]
pub type ST_PHY_ERRESC_0_R = crate::BitReader;
#[doc = "Field `ST_PHY_ERRESC_1` reader - NA"]
pub type ST_PHY_ERRESC_1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_phy_errsoths_0(&self) -> ST_PHY_ERRSOTHS_0_R {
        ST_PHY_ERRSOTHS_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_phy_errsoths_1(&self) -> ST_PHY_ERRSOTHS_1_R {
        ST_PHY_ERRSOTHS_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn st_phy_erresc_0(&self) -> ST_PHY_ERRESC_0_R {
        ST_PHY_ERRESC_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn st_phy_erresc_1(&self) -> ST_PHY_ERRESC_1_R {
        ST_PHY_ERRESC_1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_PHY")
            .field(
                "st_phy_errsoths_0",
                &format_args!("{}", self.st_phy_errsoths_0().bit()),
            )
            .field(
                "st_phy_errsoths_1",
                &format_args!("{}", self.st_phy_errsoths_1().bit()),
            )
            .field(
                "st_phy_erresc_0",
                &format_args!("{}", self.st_phy_erresc_0().bit()),
            )
            .field(
                "st_phy_erresc_1",
                &format_args!("{}", self.st_phy_erresc_1().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_PHY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_phy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_PHY_SPEC;
impl crate::RegisterSpec for INT_ST_PHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_phy::R`](R) reader structure"]
impl crate::Readable for INT_ST_PHY_SPEC {}
#[doc = "`reset()` method sets INT_ST_PHY to value 0"]
impl crate::Resettable for INT_ST_PHY_SPEC {
    const RESET_VALUE: u32 = 0;
}
