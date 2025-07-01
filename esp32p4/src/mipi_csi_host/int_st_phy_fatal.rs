#[doc = "Register `INT_ST_PHY_FATAL` reader"]
pub type R = crate::R<INT_ST_PHY_FATAL_SPEC>;
#[doc = "Field `ST_PHY_ERRSOTSYNCHS_0` reader - NA"]
pub type ST_PHY_ERRSOTSYNCHS_0_R = crate::BitReader;
#[doc = "Field `ST_PHY_ERRSOTSYNCHS_1` reader - NA"]
pub type ST_PHY_ERRSOTSYNCHS_1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_phy_errsotsynchs_0(&self) -> ST_PHY_ERRSOTSYNCHS_0_R {
        ST_PHY_ERRSOTSYNCHS_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_phy_errsotsynchs_1(&self) -> ST_PHY_ERRSOTSYNCHS_1_R {
        ST_PHY_ERRSOTSYNCHS_1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_PHY_FATAL")
            .field("st_phy_errsotsynchs_0", &self.st_phy_errsotsynchs_0())
            .field("st_phy_errsotsynchs_1", &self.st_phy_errsotsynchs_1())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_phy_fatal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_PHY_FATAL_SPEC;
impl crate::RegisterSpec for INT_ST_PHY_FATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_phy_fatal::R`](R) reader structure"]
impl crate::Readable for INT_ST_PHY_FATAL_SPEC {}
#[doc = "`reset()` method sets INT_ST_PHY_FATAL to value 0"]
impl crate::Resettable for INT_ST_PHY_FATAL_SPEC {}
