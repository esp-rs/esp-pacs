#[doc = "Register `TOUCH_FILTER_CTRL` reader"]
pub type R = crate::R<TOUCH_FILTER_CTRL_SPEC>;
#[doc = "Register `TOUCH_FILTER_CTRL` writer"]
pub type W = crate::W<TOUCH_FILTER_CTRL_SPEC>;
#[doc = "Field `TOUCH_BYPASS_NEG_NOISE_THRES` reader - bypass neg noise thres"]
pub type TOUCH_BYPASS_NEG_NOISE_THRES_R = crate::BitReader;
#[doc = "Field `TOUCH_BYPASS_NEG_NOISE_THRES` writer - bypass neg noise thres"]
pub type TOUCH_BYPASS_NEG_NOISE_THRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_BYPASS_NOISE_THRES` reader - bypaas noise thres"]
pub type TOUCH_BYPASS_NOISE_THRES_R = crate::BitReader;
#[doc = "Field `TOUCH_BYPASS_NOISE_THRES` writer - bypaas noise thres"]
pub type TOUCH_BYPASS_NOISE_THRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SMOOTH_LVL` reader - smooth filter factor"]
pub type TOUCH_SMOOTH_LVL_R = crate::FieldReader;
#[doc = "Field `TOUCH_SMOOTH_LVL` writer - smooth filter factor"]
pub type TOUCH_SMOOTH_LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_JITTER_STEP` reader - touch jitter step"]
pub type TOUCH_JITTER_STEP_R = crate::FieldReader;
#[doc = "Field `TOUCH_JITTER_STEP` writer - touch jitter step"]
pub type TOUCH_JITTER_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` reader - negative threshold counter limit"]
pub type TOUCH_NEG_NOISE_LIMIT_R = crate::FieldReader;
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` writer - negative threshold counter limit"]
pub type TOUCH_NEG_NOISE_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_NEG_NOISE_THRES` reader - neg noise thres"]
pub type TOUCH_NEG_NOISE_THRES_R = crate::FieldReader;
#[doc = "Field `TOUCH_NEG_NOISE_THRES` writer - neg noise thres"]
pub type TOUCH_NEG_NOISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_NOISE_THRES` reader - noise thres"]
pub type TOUCH_NOISE_THRES_R = crate::FieldReader;
#[doc = "Field `TOUCH_NOISE_THRES` writer - noise thres"]
pub type TOUCH_NOISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_HYSTERESIS` reader - hysteresis"]
pub type TOUCH_HYSTERESIS_R = crate::FieldReader;
#[doc = "Field `TOUCH_HYSTERESIS` writer - hysteresis"]
pub type TOUCH_HYSTERESIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DEBOUNCE` reader - debounce counter"]
pub type TOUCH_DEBOUNCE_R = crate::FieldReader;
#[doc = "Field `TOUCH_DEBOUNCE` writer - debounce counter"]
pub type TOUCH_DEBOUNCE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_FILTER_MODE` reader - 0: IIR ? 1: IIR ? 2: IIR 1/8 3: Jitter"]
pub type TOUCH_FILTER_MODE_R = crate::FieldReader;
#[doc = "Field `TOUCH_FILTER_MODE` writer - 0: IIR ? 1: IIR ? 2: IIR 1/8 3: Jitter"]
pub type TOUCH_FILTER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_FILTER_EN` reader - touch filter enable"]
pub type TOUCH_FILTER_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_FILTER_EN` writer - touch filter enable"]
pub type TOUCH_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - bypass neg noise thres"]
    #[inline(always)]
    pub fn touch_bypass_neg_noise_thres(&self) -> TOUCH_BYPASS_NEG_NOISE_THRES_R {
        TOUCH_BYPASS_NEG_NOISE_THRES_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - bypaas noise thres"]
    #[inline(always)]
    pub fn touch_bypass_noise_thres(&self) -> TOUCH_BYPASS_NOISE_THRES_R {
        TOUCH_BYPASS_NOISE_THRES_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - smooth filter factor"]
    #[inline(always)]
    pub fn touch_smooth_lvl(&self) -> TOUCH_SMOOTH_LVL_R {
        TOUCH_SMOOTH_LVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:14 - touch jitter step"]
    #[inline(always)]
    pub fn touch_jitter_step(&self) -> TOUCH_JITTER_STEP_R {
        TOUCH_JITTER_STEP_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - negative threshold counter limit"]
    #[inline(always)]
    pub fn touch_neg_noise_limit(&self) -> TOUCH_NEG_NOISE_LIMIT_R {
        TOUCH_NEG_NOISE_LIMIT_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:20 - neg noise thres"]
    #[inline(always)]
    pub fn touch_neg_noise_thres(&self) -> TOUCH_NEG_NOISE_THRES_R {
        TOUCH_NEG_NOISE_THRES_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22 - noise thres"]
    #[inline(always)]
    pub fn touch_noise_thres(&self) -> TOUCH_NOISE_THRES_R {
        TOUCH_NOISE_THRES_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - hysteresis"]
    #[inline(always)]
    pub fn touch_hysteresis(&self) -> TOUCH_HYSTERESIS_R {
        TOUCH_HYSTERESIS_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27 - debounce counter"]
    #[inline(always)]
    pub fn touch_debounce(&self) -> TOUCH_DEBOUNCE_R {
        TOUCH_DEBOUNCE_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 0: IIR ? 1: IIR ? 2: IIR 1/8 3: Jitter"]
    #[inline(always)]
    pub fn touch_filter_mode(&self) -> TOUCH_FILTER_MODE_R {
        TOUCH_FILTER_MODE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - touch filter enable"]
    #[inline(always)]
    pub fn touch_filter_en(&self) -> TOUCH_FILTER_EN_R {
        TOUCH_FILTER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_FILTER_CTRL")
            .field(
                "touch_bypass_neg_noise_thres",
                &self.touch_bypass_neg_noise_thres().bit(),
            )
            .field(
                "touch_bypass_noise_thres",
                &self.touch_bypass_noise_thres().bit(),
            )
            .field("touch_smooth_lvl", &self.touch_smooth_lvl().bits())
            .field("touch_jitter_step", &self.touch_jitter_step().bits())
            .field(
                "touch_neg_noise_limit",
                &self.touch_neg_noise_limit().bits(),
            )
            .field(
                "touch_neg_noise_thres",
                &self.touch_neg_noise_thres().bits(),
            )
            .field("touch_noise_thres", &self.touch_noise_thres().bits())
            .field("touch_hysteresis", &self.touch_hysteresis().bits())
            .field("touch_debounce", &self.touch_debounce().bits())
            .field("touch_filter_mode", &self.touch_filter_mode().bits())
            .field("touch_filter_en", &self.touch_filter_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_FILTER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 7 - bypass neg noise thres"]
    #[inline(always)]
    #[must_use]
    pub fn touch_bypass_neg_noise_thres(
        &mut self,
    ) -> TOUCH_BYPASS_NEG_NOISE_THRES_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_BYPASS_NEG_NOISE_THRES_W::new(self, 7)
    }
    #[doc = "Bit 8 - bypaas noise thres"]
    #[inline(always)]
    #[must_use]
    pub fn touch_bypass_noise_thres(
        &mut self,
    ) -> TOUCH_BYPASS_NOISE_THRES_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_BYPASS_NOISE_THRES_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - smooth filter factor"]
    #[inline(always)]
    #[must_use]
    pub fn touch_smooth_lvl(&mut self) -> TOUCH_SMOOTH_LVL_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_SMOOTH_LVL_W::new(self, 9)
    }
    #[doc = "Bits 11:14 - touch jitter step"]
    #[inline(always)]
    #[must_use]
    pub fn touch_jitter_step(&mut self) -> TOUCH_JITTER_STEP_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_JITTER_STEP_W::new(self, 11)
    }
    #[doc = "Bits 15:18 - negative threshold counter limit"]
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_limit(&mut self) -> TOUCH_NEG_NOISE_LIMIT_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_NEG_NOISE_LIMIT_W::new(self, 15)
    }
    #[doc = "Bits 19:20 - neg noise thres"]
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_thres(&mut self) -> TOUCH_NEG_NOISE_THRES_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_NEG_NOISE_THRES_W::new(self, 19)
    }
    #[doc = "Bits 21:22 - noise thres"]
    #[inline(always)]
    #[must_use]
    pub fn touch_noise_thres(&mut self) -> TOUCH_NOISE_THRES_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_NOISE_THRES_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn touch_hysteresis(&mut self) -> TOUCH_HYSTERESIS_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_HYSTERESIS_W::new(self, 23)
    }
    #[doc = "Bits 25:27 - debounce counter"]
    #[inline(always)]
    #[must_use]
    pub fn touch_debounce(&mut self) -> TOUCH_DEBOUNCE_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_DEBOUNCE_W::new(self, 25)
    }
    #[doc = "Bits 28:30 - 0: IIR ? 1: IIR ? 2: IIR 1/8 3: Jitter"]
    #[inline(always)]
    #[must_use]
    pub fn touch_filter_mode(&mut self) -> TOUCH_FILTER_MODE_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_FILTER_MODE_W::new(self, 28)
    }
    #[doc = "Bit 31 - touch filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn touch_filter_en(&mut self) -> TOUCH_FILTER_EN_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_FILTER_EN_W::new(self, 31)
    }
}
#[doc = "configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_filter_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_FILTER_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_FILTER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_filter_ctrl::R`](R) reader structure"]
impl crate::Readable for TOUCH_FILTER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_filter_ctrl::W`](W) writer structure"]
impl crate::Writable for TOUCH_FILTER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_FILTER_CTRL to value 0x96aa_8800"]
impl crate::Resettable for TOUCH_FILTER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x96aa_8800;
}
