#[doc = "Register `LP_ANA_TOUCH_FILTER1` reader"]
pub type R = crate::R<LP_ANA_TOUCH_FILTER1_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_FILTER1` writer"]
pub type W = crate::W<LP_ANA_TOUCH_FILTER1_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN` reader - Reserved"]
pub type LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN` writer - Reserved"]
pub type LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_HYSTERESIS` reader - need_des"]
pub type LP_ANA_TOUCH_HYSTERESIS_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_HYSTERESIS` writer - need_des"]
pub type LP_ANA_TOUCH_HYSTERESIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_ANA_TOUCH_NEG_NOISE_THRES` reader - need_des"]
pub type LP_ANA_TOUCH_NEG_NOISE_THRES_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_NEG_NOISE_THRES` writer - need_des"]
pub type LP_ANA_TOUCH_NEG_NOISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_ANA_TOUCH_NOISE_THRES` reader - need_des"]
pub type LP_ANA_TOUCH_NOISE_THRES_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_NOISE_THRES` writer - need_des"]
pub type LP_ANA_TOUCH_NOISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_ANA_TOUCH_SMOOTH_LVL` reader - need_des"]
pub type LP_ANA_TOUCH_SMOOTH_LVL_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_SMOOTH_LVL` writer - need_des"]
pub type LP_ANA_TOUCH_SMOOTH_LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_ANA_TOUCH_JITTER_STEP` reader - need_des"]
pub type LP_ANA_TOUCH_JITTER_STEP_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_JITTER_STEP` writer - need_des"]
pub type LP_ANA_TOUCH_JITTER_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_ANA_TOUCH_FILTER_MODE` reader - need_des"]
pub type LP_ANA_TOUCH_FILTER_MODE_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_FILTER_MODE` writer - need_des"]
pub type LP_ANA_TOUCH_FILTER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_ANA_TOUCH_FILTER_EN` reader - need_des"]
pub type LP_ANA_TOUCH_FILTER_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_FILTER_EN` writer - need_des"]
pub type LP_ANA_TOUCH_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_NEG_NOISE_LIMIT` reader - need_des"]
pub type LP_ANA_TOUCH_NEG_NOISE_LIMIT_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_NEG_NOISE_LIMIT` writer - need_des"]
pub type LP_ANA_TOUCH_NEG_NOISE_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_ANA_TOUCH_APPROACH_LIMIT` reader - need_des"]
pub type LP_ANA_TOUCH_APPROACH_LIMIT_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_APPROACH_LIMIT` writer - need_des"]
pub type LP_ANA_TOUCH_APPROACH_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_ANA_TOUCH_DEBOUNCE_LIMIT` reader - need_des"]
pub type LP_ANA_TOUCH_DEBOUNCE_LIMIT_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_DEBOUNCE_LIMIT` writer - need_des"]
pub type LP_ANA_TOUCH_DEBOUNCE_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn lp_ana_touch_neg_noise_disupdate_baseline_en(
        &self,
    ) -> LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_R {
        LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_hysteresis(&self) -> LP_ANA_TOUCH_HYSTERESIS_R {
        LP_ANA_TOUCH_HYSTERESIS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_neg_noise_thres(&self) -> LP_ANA_TOUCH_NEG_NOISE_THRES_R {
        LP_ANA_TOUCH_NEG_NOISE_THRES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_noise_thres(&self) -> LP_ANA_TOUCH_NOISE_THRES_R {
        LP_ANA_TOUCH_NOISE_THRES_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_smooth_lvl(&self) -> LP_ANA_TOUCH_SMOOTH_LVL_R {
        LP_ANA_TOUCH_SMOOTH_LVL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_jitter_step(&self) -> LP_ANA_TOUCH_JITTER_STEP_R {
        LP_ANA_TOUCH_JITTER_STEP_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_filter_mode(&self) -> LP_ANA_TOUCH_FILTER_MODE_R {
        LP_ANA_TOUCH_FILTER_MODE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_filter_en(&self) -> LP_ANA_TOUCH_FILTER_EN_R {
        LP_ANA_TOUCH_FILTER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_neg_noise_limit(&self) -> LP_ANA_TOUCH_NEG_NOISE_LIMIT_R {
        LP_ANA_TOUCH_NEG_NOISE_LIMIT_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_approach_limit(&self) -> LP_ANA_TOUCH_APPROACH_LIMIT_R {
        LP_ANA_TOUCH_APPROACH_LIMIT_R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_debounce_limit(&self) -> LP_ANA_TOUCH_DEBOUNCE_LIMIT_R {
        LP_ANA_TOUCH_DEBOUNCE_LIMIT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_FILTER1")
            .field(
                "lp_ana_touch_neg_noise_disupdate_baseline_en",
                &format_args!(
                    "{}",
                    self.lp_ana_touch_neg_noise_disupdate_baseline_en().bit()
                ),
            )
            .field(
                "lp_ana_touch_hysteresis",
                &format_args!("{}", self.lp_ana_touch_hysteresis().bits()),
            )
            .field(
                "lp_ana_touch_neg_noise_thres",
                &format_args!("{}", self.lp_ana_touch_neg_noise_thres().bits()),
            )
            .field(
                "lp_ana_touch_noise_thres",
                &format_args!("{}", self.lp_ana_touch_noise_thres().bits()),
            )
            .field(
                "lp_ana_touch_smooth_lvl",
                &format_args!("{}", self.lp_ana_touch_smooth_lvl().bits()),
            )
            .field(
                "lp_ana_touch_jitter_step",
                &format_args!("{}", self.lp_ana_touch_jitter_step().bits()),
            )
            .field(
                "lp_ana_touch_filter_mode",
                &format_args!("{}", self.lp_ana_touch_filter_mode().bits()),
            )
            .field(
                "lp_ana_touch_filter_en",
                &format_args!("{}", self.lp_ana_touch_filter_en().bit()),
            )
            .field(
                "lp_ana_touch_neg_noise_limit",
                &format_args!("{}", self.lp_ana_touch_neg_noise_limit().bits()),
            )
            .field(
                "lp_ana_touch_approach_limit",
                &format_args!("{}", self.lp_ana_touch_approach_limit().bits()),
            )
            .field(
                "lp_ana_touch_debounce_limit",
                &format_args!("{}", self.lp_ana_touch_debounce_limit().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_FILTER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_neg_noise_disupdate_baseline_en(
        &mut self,
    ) -> LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_hysteresis(
        &mut self,
    ) -> LP_ANA_TOUCH_HYSTERESIS_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_HYSTERESIS_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_neg_noise_thres(
        &mut self,
    ) -> LP_ANA_TOUCH_NEG_NOISE_THRES_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_NEG_NOISE_THRES_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_noise_thres(
        &mut self,
    ) -> LP_ANA_TOUCH_NOISE_THRES_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_NOISE_THRES_W::new(self, 5)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_smooth_lvl(
        &mut self,
    ) -> LP_ANA_TOUCH_SMOOTH_LVL_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_SMOOTH_LVL_W::new(self, 7)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_jitter_step(
        &mut self,
    ) -> LP_ANA_TOUCH_JITTER_STEP_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_JITTER_STEP_W::new(self, 9)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_filter_mode(
        &mut self,
    ) -> LP_ANA_TOUCH_FILTER_MODE_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_FILTER_MODE_W::new(self, 13)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_filter_en(
        &mut self,
    ) -> LP_ANA_TOUCH_FILTER_EN_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_FILTER_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_neg_noise_limit(
        &mut self,
    ) -> LP_ANA_TOUCH_NEG_NOISE_LIMIT_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_NEG_NOISE_LIMIT_W::new(self, 17)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_approach_limit(
        &mut self,
    ) -> LP_ANA_TOUCH_APPROACH_LIMIT_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_APPROACH_LIMIT_W::new(self, 21)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_debounce_limit(
        &mut self,
    ) -> LP_ANA_TOUCH_DEBOUNCE_LIMIT_W<LP_ANA_TOUCH_FILTER1_SPEC> {
        LP_ANA_TOUCH_DEBOUNCE_LIMIT_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_filter1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_filter1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_FILTER1_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_FILTER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_filter1::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_FILTER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_filter1::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_FILTER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_FILTER1 to value 0x6a0a_0200"]
impl crate::Resettable for LP_ANA_TOUCH_FILTER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x6a0a_0200;
}
