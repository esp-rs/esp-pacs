#[doc = "Register `PHY_LOOPBACK_CNT` reader"]
pub type R = crate::R<PHY_LOOPBACK_CNT_SPEC>;
#[doc = "Register `PHY_LOOPBACK_CNT` writer"]
pub type W = crate::W<PHY_LOOPBACK_CNT_SPEC>;
#[doc = "Field `PHY_HS_CHECK_CNT_TH` reader - hs_loopback test check cnt"]
pub type PHY_HS_CHECK_CNT_TH_R = crate::FieldReader;
#[doc = "Field `PHY_HS_CHECK_CNT_TH` writer - hs_loopback test check cnt"]
pub type PHY_HS_CHECK_CNT_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_LP_CHECK_CNT_TH` reader - lp_loopback test check cnt"]
pub type PHY_LP_CHECK_CNT_TH_R = crate::FieldReader;
#[doc = "Field `PHY_LP_CHECK_CNT_TH` writer - lp_loopback test check cnt"]
pub type PHY_LP_CHECK_CNT_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - hs_loopback test check cnt"]
    #[inline(always)]
    pub fn phy_hs_check_cnt_th(&self) -> PHY_HS_CHECK_CNT_TH_R {
        PHY_HS_CHECK_CNT_TH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - lp_loopback test check cnt"]
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
    #[doc = "Bits 0:7 - hs_loopback test check cnt"]
    #[inline(always)]
    pub fn phy_hs_check_cnt_th(&mut self) -> PHY_HS_CHECK_CNT_TH_W<PHY_LOOPBACK_CNT_SPEC> {
        PHY_HS_CHECK_CNT_TH_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - lp_loopback test check cnt"]
    #[inline(always)]
    pub fn phy_lp_check_cnt_th(&mut self) -> PHY_LP_CHECK_CNT_TH_W<PHY_LOOPBACK_CNT_SPEC> {
        PHY_LP_CHECK_CNT_TH_W::new(self, 16)
    }
}
#[doc = "loopback test cnt\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_loopback_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_loopback_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_LOOPBACK_CNT_SPEC;
impl crate::RegisterSpec for PHY_LOOPBACK_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_loopback_cnt::R`](R) reader structure"]
impl crate::Readable for PHY_LOOPBACK_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_loopback_cnt::W`](W) writer structure"]
impl crate::Writable for PHY_LOOPBACK_CNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_LOOPBACK_CNT to value 0x0040_0040"]
impl crate::Resettable for PHY_LOOPBACK_CNT_SPEC {
    const RESET_VALUE: u32 = 0x0040_0040;
}
