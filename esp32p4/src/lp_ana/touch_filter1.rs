#[doc = "Register `TOUCH_FILTER1` reader"]
pub type R = crate::R<TOUCH_FILTER1_SPEC>;
#[doc = "Register `TOUCH_FILTER1` writer"]
pub type W = crate::W<TOUCH_FILTER1_SPEC>;
#[doc = "Field `TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN` reader - Reserved"]
pub type TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN` writer - Reserved"]
pub type TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_HYSTERESIS` reader - need_des"]
pub type TOUCH_HYSTERESIS_R = crate::FieldReader;
#[doc = "Field `TOUCH_HYSTERESIS` writer - need_des"]
pub type TOUCH_HYSTERESIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_NEG_NOISE_THRES` reader - need_des"]
pub type TOUCH_NEG_NOISE_THRES_R = crate::FieldReader;
#[doc = "Field `TOUCH_NEG_NOISE_THRES` writer - need_des"]
pub type TOUCH_NEG_NOISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_NOISE_THRES` reader - need_des"]
pub type TOUCH_NOISE_THRES_R = crate::FieldReader;
#[doc = "Field `TOUCH_NOISE_THRES` writer - need_des"]
pub type TOUCH_NOISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_SMOOTH_LVL` reader - need_des"]
pub type TOUCH_SMOOTH_LVL_R = crate::FieldReader;
#[doc = "Field `TOUCH_SMOOTH_LVL` writer - need_des"]
pub type TOUCH_SMOOTH_LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_JITTER_STEP` reader - need_des"]
pub type TOUCH_JITTER_STEP_R = crate::FieldReader;
#[doc = "Field `TOUCH_JITTER_STEP` writer - need_des"]
pub type TOUCH_JITTER_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_FILTER_MODE` reader - need_des"]
pub type TOUCH_FILTER_MODE_R = crate::FieldReader;
#[doc = "Field `TOUCH_FILTER_MODE` writer - need_des"]
pub type TOUCH_FILTER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_FILTER_EN` reader - need_des"]
pub type TOUCH_FILTER_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_FILTER_EN` writer - need_des"]
pub type TOUCH_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` reader - need_des"]
pub type TOUCH_NEG_NOISE_LIMIT_R = crate::FieldReader;
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` writer - need_des"]
pub type TOUCH_NEG_NOISE_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_APPROACH_LIMIT` reader - need_des"]
pub type TOUCH_APPROACH_LIMIT_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_LIMIT` writer - need_des"]
pub type TOUCH_APPROACH_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOUCH_DEBOUNCE_LIMIT` reader - need_des"]
pub type TOUCH_DEBOUNCE_LIMIT_R = crate::FieldReader;
#[doc = "Field `TOUCH_DEBOUNCE_LIMIT` writer - need_des"]
pub type TOUCH_DEBOUNCE_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn touch_neg_noise_disupdate_baseline_en(&self) -> TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_R {
        TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    pub fn touch_hysteresis(&self) -> TOUCH_HYSTERESIS_R {
        TOUCH_HYSTERESIS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn touch_neg_noise_thres(&self) -> TOUCH_NEG_NOISE_THRES_R {
        TOUCH_NEG_NOISE_THRES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn touch_noise_thres(&self) -> TOUCH_NOISE_THRES_R {
        TOUCH_NOISE_THRES_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_smooth_lvl(&self) -> TOUCH_SMOOTH_LVL_R {
        TOUCH_SMOOTH_LVL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_jitter_step(&self) -> TOUCH_JITTER_STEP_R {
        TOUCH_JITTER_STEP_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    pub fn touch_filter_mode(&self) -> TOUCH_FILTER_MODE_R {
        TOUCH_FILTER_MODE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn touch_filter_en(&self) -> TOUCH_FILTER_EN_R {
        TOUCH_FILTER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    pub fn touch_neg_noise_limit(&self) -> TOUCH_NEG_NOISE_LIMIT_R {
        TOUCH_NEG_NOISE_LIMIT_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn touch_approach_limit(&self) -> TOUCH_APPROACH_LIMIT_R {
        TOUCH_APPROACH_LIMIT_R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn touch_debounce_limit(&self) -> TOUCH_DEBOUNCE_LIMIT_R {
        TOUCH_DEBOUNCE_LIMIT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_FILTER1")
            .field(
                "touch_neg_noise_disupdate_baseline_en",
                &self.touch_neg_noise_disupdate_baseline_en(),
            )
            .field("touch_hysteresis", &self.touch_hysteresis())
            .field("touch_neg_noise_thres", &self.touch_neg_noise_thres())
            .field("touch_noise_thres", &self.touch_noise_thres())
            .field("touch_smooth_lvl", &self.touch_smooth_lvl())
            .field("touch_jitter_step", &self.touch_jitter_step())
            .field("touch_filter_mode", &self.touch_filter_mode())
            .field("touch_filter_en", &self.touch_filter_en())
            .field("touch_neg_noise_limit", &self.touch_neg_noise_limit())
            .field("touch_approach_limit", &self.touch_approach_limit())
            .field("touch_debounce_limit", &self.touch_debounce_limit())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_disupdate_baseline_en(
        &mut self,
    ) -> TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_W<TOUCH_FILTER1_SPEC> {
        TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_hysteresis(&mut self) -> TOUCH_HYSTERESIS_W<TOUCH_FILTER1_SPEC> {
        TOUCH_HYSTERESIS_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_thres(&mut self) -> TOUCH_NEG_NOISE_THRES_W<TOUCH_FILTER1_SPEC> {
        TOUCH_NEG_NOISE_THRES_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_noise_thres(&mut self) -> TOUCH_NOISE_THRES_W<TOUCH_FILTER1_SPEC> {
        TOUCH_NOISE_THRES_W::new(self, 5)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_smooth_lvl(&mut self) -> TOUCH_SMOOTH_LVL_W<TOUCH_FILTER1_SPEC> {
        TOUCH_SMOOTH_LVL_W::new(self, 7)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_jitter_step(&mut self) -> TOUCH_JITTER_STEP_W<TOUCH_FILTER1_SPEC> {
        TOUCH_JITTER_STEP_W::new(self, 9)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_filter_mode(&mut self) -> TOUCH_FILTER_MODE_W<TOUCH_FILTER1_SPEC> {
        TOUCH_FILTER_MODE_W::new(self, 13)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_filter_en(&mut self) -> TOUCH_FILTER_EN_W<TOUCH_FILTER1_SPEC> {
        TOUCH_FILTER_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_limit(&mut self) -> TOUCH_NEG_NOISE_LIMIT_W<TOUCH_FILTER1_SPEC> {
        TOUCH_NEG_NOISE_LIMIT_W::new(self, 17)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_limit(&mut self) -> TOUCH_APPROACH_LIMIT_W<TOUCH_FILTER1_SPEC> {
        TOUCH_APPROACH_LIMIT_W::new(self, 21)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_debounce_limit(&mut self) -> TOUCH_DEBOUNCE_LIMIT_W<TOUCH_FILTER1_SPEC> {
        TOUCH_DEBOUNCE_LIMIT_W::new(self, 29)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_filter1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_filter1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_FILTER1_SPEC;
impl crate::RegisterSpec for TOUCH_FILTER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_filter1::R`](R) reader structure"]
impl crate::Readable for TOUCH_FILTER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_filter1::W`](W) writer structure"]
impl crate::Writable for TOUCH_FILTER1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_FILTER1 to value 0x6a0a_0200"]
impl crate::Resettable for TOUCH_FILTER1_SPEC {
    const RESET_VALUE: u32 = 0x6a0a_0200;
}
