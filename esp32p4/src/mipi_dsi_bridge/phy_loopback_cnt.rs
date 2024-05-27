///Register `PHY_LOOPBACK_CNT` reader
pub type R = crate::R<PHY_LOOPBACK_CNT_SPEC>;
///Register `PHY_LOOPBACK_CNT` writer
pub type W = crate::W<PHY_LOOPBACK_CNT_SPEC>;
///Field `PHY_HS_CHECK_CNT_TH` reader - hs_loopback test check cnt
pub type PHY_HS_CHECK_CNT_TH_R = crate::FieldReader;
///Field `PHY_HS_CHECK_CNT_TH` writer - hs_loopback test check cnt
pub type PHY_HS_CHECK_CNT_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PHY_LP_CHECK_CNT_TH` reader - lp_loopback test check cnt
pub type PHY_LP_CHECK_CNT_TH_R = crate::FieldReader;
///Field `PHY_LP_CHECK_CNT_TH` writer - lp_loopback test check cnt
pub type PHY_LP_CHECK_CNT_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - hs_loopback test check cnt
    #[inline(always)]
    pub fn phy_hs_check_cnt_th(&self) -> PHY_HS_CHECK_CNT_TH_R {
        PHY_HS_CHECK_CNT_TH_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - lp_loopback test check cnt
    #[inline(always)]
    pub fn phy_lp_check_cnt_th(&self) -> PHY_LP_CHECK_CNT_TH_R {
        PHY_LP_CHECK_CNT_TH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_LOOPBACK_CNT")
            .field("phy_hs_check_cnt_th", &self.phy_hs_check_cnt_th())
            .field("phy_lp_check_cnt_th", &self.phy_lp_check_cnt_th())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - hs_loopback test check cnt
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_check_cnt_th(&mut self) -> PHY_HS_CHECK_CNT_TH_W<PHY_LOOPBACK_CNT_SPEC> {
        PHY_HS_CHECK_CNT_TH_W::new(self, 0)
    }
    ///Bits 16:23 - lp_loopback test check cnt
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_check_cnt_th(&mut self) -> PHY_LP_CHECK_CNT_TH_W<PHY_LOOPBACK_CNT_SPEC> {
        PHY_LP_CHECK_CNT_TH_W::new(self, 16)
    }
}
/**loopback test cnt

You can [`read`](crate::generic::Reg::read) this register and get [`phy_loopback_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_loopback_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PHY_LOOPBACK_CNT_SPEC;
impl crate::RegisterSpec for PHY_LOOPBACK_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`phy_loopback_cnt::R`](R) reader structure
impl crate::Readable for PHY_LOOPBACK_CNT_SPEC {}
///`write(|w| ..)` method takes [`phy_loopback_cnt::W`](W) writer structure
impl crate::Writable for PHY_LOOPBACK_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PHY_LOOPBACK_CNT to value 0x0040_0040
impl crate::Resettable for PHY_LOOPBACK_CNT_SPEC {
    const RESET_VALUE: u32 = 0x0040_0040;
}
