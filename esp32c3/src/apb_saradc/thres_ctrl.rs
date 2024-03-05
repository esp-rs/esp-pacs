#[doc = "Register `THRES_CTRL` reader"]
pub type R = crate::R<THRES_CTRL_SPEC>;
#[doc = "Register `THRES_CTRL` writer"]
pub type W = crate::W<THRES_CTRL_SPEC>;
#[doc = "Field `APB_SARADC_THRES_ALL_EN` reader - enable thres to all channel"]
pub type APB_SARADC_THRES_ALL_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES_ALL_EN` writer - enable thres to all channel"]
pub type APB_SARADC_THRES_ALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_THRES1_EN` reader - enable thres1"]
pub type APB_SARADC_THRES1_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_EN` writer - enable thres1"]
pub type APB_SARADC_THRES1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_THRES0_EN` reader - enable thres0"]
pub type APB_SARADC_THRES0_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_EN` writer - enable thres0"]
pub type APB_SARADC_THRES0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - enable thres to all channel"]
    #[inline(always)]
    pub fn apb_saradc_thres_all_en(&self) -> APB_SARADC_THRES_ALL_EN_R {
        APB_SARADC_THRES_ALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - enable thres1"]
    #[inline(always)]
    pub fn apb_saradc_thres1_en(&self) -> APB_SARADC_THRES1_EN_R {
        APB_SARADC_THRES1_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable thres0"]
    #[inline(always)]
    pub fn apb_saradc_thres0_en(&self) -> APB_SARADC_THRES0_EN_R {
        APB_SARADC_THRES0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES_CTRL")
            .field(
                "apb_saradc_thres_all_en",
                &format_args!("{}", self.apb_saradc_thres_all_en().bit()),
            )
            .field(
                "apb_saradc_thres1_en",
                &format_args!("{}", self.apb_saradc_thres1_en().bit()),
            )
            .field(
                "apb_saradc_thres0_en",
                &format_args!("{}", self.apb_saradc_thres0_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<THRES_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 27 - enable thres to all channel"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres_all_en(&mut self) -> APB_SARADC_THRES_ALL_EN_W<THRES_CTRL_SPEC> {
        APB_SARADC_THRES_ALL_EN_W::new(self, 27)
    }
    #[doc = "Bit 30 - enable thres1"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_en(&mut self) -> APB_SARADC_THRES1_EN_W<THRES_CTRL_SPEC> {
        APB_SARADC_THRES1_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable thres0"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_en(&mut self) -> APB_SARADC_THRES0_EN_W<THRES_CTRL_SPEC> {
        APB_SARADC_THRES0_EN_W::new(self, 31)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres_ctrl::R`](R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres_ctrl::W`](W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
