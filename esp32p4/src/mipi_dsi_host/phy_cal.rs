///Register `PHY_CAL` reader
pub type R = crate::R<PHY_CAL_SPEC>;
///Register `PHY_CAL` writer
pub type W = crate::W<PHY_CAL_SPEC>;
///Field `TXSKEWCALHS` reader - NA
pub type TXSKEWCALHS_R = crate::BitReader;
///Field `TXSKEWCALHS` writer - NA
pub type TXSKEWCALHS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn txskewcalhs(&self) -> TXSKEWCALHS_R {
        TXSKEWCALHS_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_CAL")
            .field("txskewcalhs", &self.txskewcalhs())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn txskewcalhs(&mut self) -> TXSKEWCALHS_W<PHY_CAL_SPEC> {
        TXSKEWCALHS_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_cal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_cal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PHY_CAL_SPEC;
impl crate::RegisterSpec for PHY_CAL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`phy_cal::R`](R) reader structure
impl crate::Readable for PHY_CAL_SPEC {}
///`write(|w| ..)` method takes [`phy_cal::W`](W) writer structure
impl crate::Writable for PHY_CAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PHY_CAL to value 0
impl crate::Resettable for PHY_CAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
