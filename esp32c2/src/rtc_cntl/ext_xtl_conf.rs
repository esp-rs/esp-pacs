///Register `EXT_XTL_CONF` reader
pub type R = crate::R<EXT_XTL_CONF_SPEC>;
///Register `EXT_XTL_CONF` writer
pub type W = crate::W<EXT_XTL_CONF_SPEC>;
///Field `XTL_EXT_CTR_LV` reader - 0: power down XTAL at high level
pub type XTL_EXT_CTR_LV_R = crate::BitReader;
///Field `XTL_EXT_CTR_LV` writer - 0: power down XTAL at high level
pub type XTL_EXT_CTR_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XTL_EXT_CTR_EN` reader - Need add desc
pub type XTL_EXT_CTR_EN_R = crate::BitReader;
///Field `XTL_EXT_CTR_EN` writer - Need add desc
pub type XTL_EXT_CTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - 0: power down XTAL at high level
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&self) -> XTL_EXT_CTR_LV_R {
        XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Need add desc
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&self) -> XTL_EXT_CTR_EN_R {
        XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_XTL_CONF")
            .field("xtl_ext_ctr_lv", &self.xtl_ext_ctr_lv())
            .field("xtl_ext_ctr_en", &self.xtl_ext_ctr_en())
            .finish()
    }
}
impl W {
    ///Bit 30 - 0: power down XTAL at high level
    #[inline(always)]
    #[must_use]
    pub fn xtl_ext_ctr_lv(&mut self) -> XTL_EXT_CTR_LV_W<EXT_XTL_CONF_SPEC> {
        XTL_EXT_CTR_LV_W::new(self, 30)
    }
    ///Bit 31 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn xtl_ext_ctr_en(&mut self) -> XTL_EXT_CTR_EN_W<EXT_XTL_CONF_SPEC> {
        XTL_EXT_CTR_EN_W::new(self, 31)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXT_XTL_CONF_SPEC;
impl crate::RegisterSpec for EXT_XTL_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ext_xtl_conf::R`](R) reader structure
impl crate::Readable for EXT_XTL_CONF_SPEC {}
///`write(|w| ..)` method takes [`ext_xtl_conf::W`](W) writer structure
impl crate::Writable for EXT_XTL_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXT_XTL_CONF to value 0
impl crate::Resettable for EXT_XTL_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
