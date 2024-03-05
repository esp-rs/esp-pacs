#[doc = "Register `LP_ANA_TOUCH_FILTER2` reader"]
pub type R = crate::R<LP_ANA_TOUCH_FILTER2_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_FILTER2` writer"]
pub type W = crate::W<LP_ANA_TOUCH_FILTER2_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_OUTEN` reader - need_des"]
pub type LP_ANA_TOUCH_OUTEN_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_OUTEN` writer - need_des"]
pub type LP_ANA_TOUCH_OUTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LP_ANA_TOUCH_BYPASS_NOISE_THRES` reader - need_des"]
pub type LP_ANA_TOUCH_BYPASS_NOISE_THRES_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_BYPASS_NOISE_THRES` writer - need_des"]
pub type LP_ANA_TOUCH_BYPASS_NOISE_THRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES` reader - need_des"]
pub type LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES` writer - need_des"]
pub type LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 15:29 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_outen(&self) -> LP_ANA_TOUCH_OUTEN_R {
        LP_ANA_TOUCH_OUTEN_R::new(((self.bits >> 15) & 0x7fff) as u16)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_bypass_noise_thres(&self) -> LP_ANA_TOUCH_BYPASS_NOISE_THRES_R {
        LP_ANA_TOUCH_BYPASS_NOISE_THRES_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_bypass_neg_noise_thres(&self) -> LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_R {
        LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_FILTER2")
            .field(
                "lp_ana_touch_outen",
                &format_args!("{}", self.lp_ana_touch_outen().bits()),
            )
            .field(
                "lp_ana_touch_bypass_noise_thres",
                &format_args!("{}", self.lp_ana_touch_bypass_noise_thres().bit()),
            )
            .field(
                "lp_ana_touch_bypass_neg_noise_thres",
                &format_args!("{}", self.lp_ana_touch_bypass_neg_noise_thres().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_FILTER2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 15:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_outen(&mut self) -> LP_ANA_TOUCH_OUTEN_W<LP_ANA_TOUCH_FILTER2_SPEC> {
        LP_ANA_TOUCH_OUTEN_W::new(self, 15)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_bypass_noise_thres(
        &mut self,
    ) -> LP_ANA_TOUCH_BYPASS_NOISE_THRES_W<LP_ANA_TOUCH_FILTER2_SPEC> {
        LP_ANA_TOUCH_BYPASS_NOISE_THRES_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_bypass_neg_noise_thres(
        &mut self,
    ) -> LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_W<LP_ANA_TOUCH_FILTER2_SPEC> {
        LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_filter2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_filter2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_FILTER2_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_FILTER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_filter2::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_FILTER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_filter2::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_FILTER2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_FILTER2 to value 0x1fff_8000"]
impl crate::Resettable for LP_ANA_TOUCH_FILTER2_SPEC {
    const RESET_VALUE: u32 = 0x1fff_8000;
}
