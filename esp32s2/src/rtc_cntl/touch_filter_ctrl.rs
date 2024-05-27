///Register `TOUCH_FILTER_CTRL` reader
pub type R = crate::R<TOUCH_FILTER_CTRL_SPEC>;
///Register `TOUCH_FILTER_CTRL` writer
pub type W = crate::W<TOUCH_FILTER_CTRL_SPEC>;
///Field `TOUCH_SMOOTH_LVL` reader - 0: Raw data. 1: IIR1/2. 2: IIR1/4. 3: IIR1/8.
pub type TOUCH_SMOOTH_LVL_R = crate::FieldReader;
///Field `TOUCH_SMOOTH_LVL` writer - 0: Raw data. 1: IIR1/2. 2: IIR1/4. 3: IIR1/8.
pub type TOUCH_SMOOTH_LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TOUCH_JITTER_STEP` reader - Touch jitter step. Range: 0 – 15.
pub type TOUCH_JITTER_STEP_R = crate::FieldReader;
///Field `TOUCH_JITTER_STEP` writer - Touch jitter step. Range: 0 – 15.
pub type TOUCH_JITTER_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TOUCH_NEG_NOISE_LIMIT` reader - Negative threshold counter limit.
pub type TOUCH_NEG_NOISE_LIMIT_R = crate::FieldReader;
///Field `TOUCH_NEG_NOISE_LIMIT` writer - Negative threshold counter limit.
pub type TOUCH_NEG_NOISE_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TOUCH_NEG_NOISE_THRES` reader - Negative noise threshold.
pub type TOUCH_NEG_NOISE_THRES_R = crate::FieldReader;
///Field `TOUCH_NEG_NOISE_THRES` writer - Negative noise threshold.
pub type TOUCH_NEG_NOISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TOUCH_NOISE_THRES` reader - Active noise threshold.
pub type TOUCH_NOISE_THRES_R = crate::FieldReader;
///Field `TOUCH_NOISE_THRES` writer - Active noise threshold.
pub type TOUCH_NOISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TOUCH_HYSTERESIS` reader - Touch hysteresis.
pub type TOUCH_HYSTERESIS_R = crate::FieldReader;
///Field `TOUCH_HYSTERESIS` writer - Touch hysteresis.
pub type TOUCH_HYSTERESIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TOUCH_DEBOUNCE` reader - Debounce counter.
pub type TOUCH_DEBOUNCE_R = crate::FieldReader;
///Field `TOUCH_DEBOUNCE` writer - Debounce counter.
pub type TOUCH_DEBOUNCE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TOUCH_FILTER_MODE` reader - Set filter mode. 0: IIR 1/2; 1: IIR 1/4; 2: IIR 1/8; 3: IIR 1/16; 4: IIR 1/32; 5: IIR 1/64; 6: IIR 1/128; 7: Jitter.
pub type TOUCH_FILTER_MODE_R = crate::FieldReader;
///Field `TOUCH_FILTER_MODE` writer - Set filter mode. 0: IIR 1/2; 1: IIR 1/4; 2: IIR 1/8; 3: IIR 1/16; 4: IIR 1/32; 5: IIR 1/64; 6: IIR 1/128; 7: Jitter.
pub type TOUCH_FILTER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TOUCH_FILTER_EN` reader - Enable touch filter.
pub type TOUCH_FILTER_EN_R = crate::BitReader;
///Field `TOUCH_FILTER_EN` writer - Enable touch filter.
pub type TOUCH_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 9:10 - 0: Raw data. 1: IIR1/2. 2: IIR1/4. 3: IIR1/8.
    #[inline(always)]
    pub fn touch_smooth_lvl(&self) -> TOUCH_SMOOTH_LVL_R {
        TOUCH_SMOOTH_LVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:14 - Touch jitter step. Range: 0 – 15.
    #[inline(always)]
    pub fn touch_jitter_step(&self) -> TOUCH_JITTER_STEP_R {
        TOUCH_JITTER_STEP_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bits 15:18 - Negative threshold counter limit.
    #[inline(always)]
    pub fn touch_neg_noise_limit(&self) -> TOUCH_NEG_NOISE_LIMIT_R {
        TOUCH_NEG_NOISE_LIMIT_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    ///Bits 19:20 - Negative noise threshold.
    #[inline(always)]
    pub fn touch_neg_noise_thres(&self) -> TOUCH_NEG_NOISE_THRES_R {
        TOUCH_NEG_NOISE_THRES_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bits 21:22 - Active noise threshold.
    #[inline(always)]
    pub fn touch_noise_thres(&self) -> TOUCH_NOISE_THRES_R {
        TOUCH_NOISE_THRES_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 23:24 - Touch hysteresis.
    #[inline(always)]
    pub fn touch_hysteresis(&self) -> TOUCH_HYSTERESIS_R {
        TOUCH_HYSTERESIS_R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bits 25:27 - Debounce counter.
    #[inline(always)]
    pub fn touch_debounce(&self) -> TOUCH_DEBOUNCE_R {
        TOUCH_DEBOUNCE_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bits 28:30 - Set filter mode. 0: IIR 1/2; 1: IIR 1/4; 2: IIR 1/8; 3: IIR 1/16; 4: IIR 1/32; 5: IIR 1/64; 6: IIR 1/128; 7: Jitter.
    #[inline(always)]
    pub fn touch_filter_mode(&self) -> TOUCH_FILTER_MODE_R {
        TOUCH_FILTER_MODE_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - Enable touch filter.
    #[inline(always)]
    pub fn touch_filter_en(&self) -> TOUCH_FILTER_EN_R {
        TOUCH_FILTER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_FILTER_CTRL")
            .field("touch_smooth_lvl", &self.touch_smooth_lvl())
            .field("touch_jitter_step", &self.touch_jitter_step())
            .field("touch_neg_noise_limit", &self.touch_neg_noise_limit())
            .field("touch_neg_noise_thres", &self.touch_neg_noise_thres())
            .field("touch_noise_thres", &self.touch_noise_thres())
            .field("touch_hysteresis", &self.touch_hysteresis())
            .field("touch_debounce", &self.touch_debounce())
            .field("touch_filter_mode", &self.touch_filter_mode())
            .field("touch_filter_en", &self.touch_filter_en())
            .finish()
    }
}
impl W {
    ///Bits 9:10 - 0: Raw data. 1: IIR1/2. 2: IIR1/4. 3: IIR1/8.
    #[inline(always)]
    #[must_use]
    pub fn touch_smooth_lvl(&mut self) -> TOUCH_SMOOTH_LVL_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_SMOOTH_LVL_W::new(self, 9)
    }
    ///Bits 11:14 - Touch jitter step. Range: 0 – 15.
    #[inline(always)]
    #[must_use]
    pub fn touch_jitter_step(&mut self) -> TOUCH_JITTER_STEP_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_JITTER_STEP_W::new(self, 11)
    }
    ///Bits 15:18 - Negative threshold counter limit.
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_limit(&mut self) -> TOUCH_NEG_NOISE_LIMIT_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_NEG_NOISE_LIMIT_W::new(self, 15)
    }
    ///Bits 19:20 - Negative noise threshold.
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_thres(&mut self) -> TOUCH_NEG_NOISE_THRES_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_NEG_NOISE_THRES_W::new(self, 19)
    }
    ///Bits 21:22 - Active noise threshold.
    #[inline(always)]
    #[must_use]
    pub fn touch_noise_thres(&mut self) -> TOUCH_NOISE_THRES_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_NOISE_THRES_W::new(self, 21)
    }
    ///Bits 23:24 - Touch hysteresis.
    #[inline(always)]
    #[must_use]
    pub fn touch_hysteresis(&mut self) -> TOUCH_HYSTERESIS_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_HYSTERESIS_W::new(self, 23)
    }
    ///Bits 25:27 - Debounce counter.
    #[inline(always)]
    #[must_use]
    pub fn touch_debounce(&mut self) -> TOUCH_DEBOUNCE_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_DEBOUNCE_W::new(self, 25)
    }
    ///Bits 28:30 - Set filter mode. 0: IIR 1/2; 1: IIR 1/4; 2: IIR 1/8; 3: IIR 1/16; 4: IIR 1/32; 5: IIR 1/64; 6: IIR 1/128; 7: Jitter.
    #[inline(always)]
    #[must_use]
    pub fn touch_filter_mode(&mut self) -> TOUCH_FILTER_MODE_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_FILTER_MODE_W::new(self, 28)
    }
    ///Bit 31 - Enable touch filter.
    #[inline(always)]
    #[must_use]
    pub fn touch_filter_en(&mut self) -> TOUCH_FILTER_EN_W<TOUCH_FILTER_CTRL_SPEC> {
        TOUCH_FILTER_EN_W::new(self, 31)
    }
}
/**Configure touch filter settings

You can [`read`](crate::generic::Reg::read) this register and get [`touch_filter_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_FILTER_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_FILTER_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_filter_ctrl::R`](R) reader structure
impl crate::Readable for TOUCH_FILTER_CTRL_SPEC {}
///`write(|w| ..)` method takes [`touch_filter_ctrl::W`](W) writer structure
impl crate::Writable for TOUCH_FILTER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_FILTER_CTRL to value 0x96aa_8800
impl crate::Resettable for TOUCH_FILTER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x96aa_8800;
}
