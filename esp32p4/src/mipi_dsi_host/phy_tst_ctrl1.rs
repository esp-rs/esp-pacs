///Register `PHY_TST_CTRL1` reader
pub type R = crate::R<PHY_TST_CTRL1_SPEC>;
///Register `PHY_TST_CTRL1` writer
pub type W = crate::W<PHY_TST_CTRL1_SPEC>;
///Field `PHY_TESTDIN` reader - NA
pub type PHY_TESTDIN_R = crate::FieldReader;
///Field `PHY_TESTDIN` writer - NA
pub type PHY_TESTDIN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PHT_TESTDOUT` reader - NA
pub type PHT_TESTDOUT_R = crate::FieldReader;
///Field `PHY_TESTEN` reader - NA
pub type PHY_TESTEN_R = crate::BitReader;
///Field `PHY_TESTEN` writer - NA
pub type PHY_TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - NA
    #[inline(always)]
    pub fn phy_testdin(&self) -> PHY_TESTDIN_R {
        PHY_TESTDIN_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - NA
    #[inline(always)]
    pub fn pht_testdout(&self) -> PHT_TESTDOUT_R {
        PHT_TESTDOUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - NA
    #[inline(always)]
    pub fn phy_testen(&self) -> PHY_TESTEN_R {
        PHY_TESTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_TST_CTRL1")
            .field("phy_testdin", &self.phy_testdin())
            .field("pht_testdout", &self.pht_testdout())
            .field("phy_testen", &self.phy_testen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - NA
    #[inline(always)]
    #[must_use]
    pub fn phy_testdin(&mut self) -> PHY_TESTDIN_W<PHY_TST_CTRL1_SPEC> {
        PHY_TESTDIN_W::new(self, 0)
    }
    ///Bit 16 - NA
    #[inline(always)]
    #[must_use]
    pub fn phy_testen(&mut self) -> PHY_TESTEN_W<PHY_TST_CTRL1_SPEC> {
        PHY_TESTEN_W::new(self, 16)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tst_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PHY_TST_CTRL1_SPEC;
impl crate::RegisterSpec for PHY_TST_CTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`phy_tst_ctrl1::R`](R) reader structure
impl crate::Readable for PHY_TST_CTRL1_SPEC {}
///`write(|w| ..)` method takes [`phy_tst_ctrl1::W`](W) writer structure
impl crate::Writable for PHY_TST_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PHY_TST_CTRL1 to value 0
impl crate::Resettable for PHY_TST_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
