#[doc = "Register `PHY_CAL` reader"]
pub type R = crate::R<PHY_CAL_SPEC>;
#[doc = "Field `RXSKEWCALHS` reader - NA"]
pub type RXSKEWCALHS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn rxskewcalhs(&self) -> RXSKEWCALHS_R {
        RXSKEWCALHS_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_CAL")
            .field("rxskewcalhs", &format_args!("{}", self.rxskewcalhs().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_CAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_cal::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_CAL_SPEC;
impl crate::RegisterSpec for PHY_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cal::R`](R) reader structure"]
impl crate::Readable for PHY_CAL_SPEC {}
#[doc = "`reset()` method sets PHY_CAL to value 0"]
impl crate::Resettable for PHY_CAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
