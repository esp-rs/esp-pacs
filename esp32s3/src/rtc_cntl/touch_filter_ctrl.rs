#[doc = "Register `TOUCH_FILTER_CTRL` reader"]
pub struct R(crate::R<TOUCH_FILTER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_FILTER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_FILTER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_FILTER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_FILTER_CTRL` writer"]
pub struct W(crate::W<TOUCH_FILTER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_FILTER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TOUCH_FILTER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_FILTER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_BYPASS_NEG_NOISE_THRES` reader - bypass neg noise thres"]
pub type TOUCH_BYPASS_NEG_NOISE_THRES_R = crate::BitReader;
#[doc = "Field `TOUCH_BYPASS_NEG_NOISE_THRES` writer - bypass neg noise thres"]
pub type TOUCH_BYPASS_NEG_NOISE_THRES_W<'a, const O: u8> =
    crate::BitWriter<'a, TOUCH_FILTER_CTRL_SPEC, O>;
#[doc = "Field `TOUCH_BYPASS_NOISE_THRES` reader - bypaas noise thres"]
pub type TOUCH_BYPASS_NOISE_THRES_R = crate::BitReader;
#[doc = "Field `TOUCH_BYPASS_NOISE_THRES` writer - bypaas noise thres"]
pub type TOUCH_BYPASS_NOISE_THRES_W<'a, const O: u8> =
    crate::BitWriter<'a, TOUCH_FILTER_CTRL_SPEC, O>;
#[doc = "Field `TOUCH_SMOOTH_LVL` reader - smooth filter factor"]
pub type TOUCH_SMOOTH_LVL_R = crate::FieldReader;
#[doc = "Field `TOUCH_SMOOTH_LVL` writer - smooth filter factor"]
pub type TOUCH_SMOOTH_LVL_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_FILTER_CTRL_SPEC, 2, O>;
#[doc = "Field `TOUCH_JITTER_STEP` reader - touch jitter step"]
pub type TOUCH_JITTER_STEP_R = crate::FieldReader;
#[doc = "Field `TOUCH_JITTER_STEP` writer - touch jitter step"]
pub type TOUCH_JITTER_STEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_FILTER_CTRL_SPEC, 4, O>;
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` reader - negative threshold counter limit"]
pub type TOUCH_NEG_NOISE_LIMIT_R = crate::FieldReader;
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` writer - negative threshold counter limit"]
pub type TOUCH_NEG_NOISE_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_FILTER_CTRL_SPEC, 4, O>;
#[doc = "Field `TOUCH_NEG_NOISE_THRES` reader - neg noise thres"]
pub type TOUCH_NEG_NOISE_THRES_R = crate::FieldReader;
#[doc = "Field `TOUCH_NEG_NOISE_THRES` writer - neg noise thres"]
pub type TOUCH_NEG_NOISE_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_FILTER_CTRL_SPEC, 2, O>;
#[doc = "Field `TOUCH_NOISE_THRES` reader - noise thres"]
pub type TOUCH_NOISE_THRES_R = crate::FieldReader;
#[doc = "Field `TOUCH_NOISE_THRES` writer - noise thres"]
pub type TOUCH_NOISE_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_FILTER_CTRL_SPEC, 2, O>;
#[doc = "Field `TOUCH_HYSTERESIS` reader - hysteresis"]
pub type TOUCH_HYSTERESIS_R = crate::FieldReader;
#[doc = "Field `TOUCH_HYSTERESIS` writer - hysteresis"]
pub type TOUCH_HYSTERESIS_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_FILTER_CTRL_SPEC, 2, O>;
#[doc = "Field `TOUCH_DEBOUNCE` reader - debounce counter"]
pub type TOUCH_DEBOUNCE_R = crate::FieldReader;
#[doc = "Field `TOUCH_DEBOUNCE` writer - debounce counter"]
pub type TOUCH_DEBOUNCE_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_FILTER_CTRL_SPEC, 3, O>;
#[doc = "Field `TOUCH_FILTER_MODE` reader - 0: IIR ? 1: IIR ? 2: IIR 1/8 3: Jitter"]
pub type TOUCH_FILTER_MODE_R = crate::FieldReader;
#[doc = "Field `TOUCH_FILTER_MODE` writer - 0: IIR ? 1: IIR ? 2: IIR 1/8 3: Jitter"]
pub type TOUCH_FILTER_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_FILTER_CTRL_SPEC, 3, O>;
#[doc = "Field `TOUCH_FILTER_EN` reader - touch filter enable"]
pub type TOUCH_FILTER_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_FILTER_EN` writer - touch filter enable"]
pub type TOUCH_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_FILTER_CTRL_SPEC, O>;
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
                &format_args!("{}", self.touch_bypass_neg_noise_thres().bit()),
            )
            .field(
                "touch_bypass_noise_thres",
                &format_args!("{}", self.touch_bypass_noise_thres().bit()),
            )
            .field(
                "touch_smooth_lvl",
                &format_args!("{}", self.touch_smooth_lvl().bits()),
            )
            .field(
                "touch_jitter_step",
                &format_args!("{}", self.touch_jitter_step().bits()),
            )
            .field(
                "touch_neg_noise_limit",
                &format_args!("{}", self.touch_neg_noise_limit().bits()),
            )
            .field(
                "touch_neg_noise_thres",
                &format_args!("{}", self.touch_neg_noise_thres().bits()),
            )
            .field(
                "touch_noise_thres",
                &format_args!("{}", self.touch_noise_thres().bits()),
            )
            .field(
                "touch_hysteresis",
                &format_args!("{}", self.touch_hysteresis().bits()),
            )
            .field(
                "touch_debounce",
                &format_args!("{}", self.touch_debounce().bits()),
            )
            .field(
                "touch_filter_mode",
                &format_args!("{}", self.touch_filter_mode().bits()),
            )
            .field(
                "touch_filter_en",
                &format_args!("{}", self.touch_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_FILTER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 7 - bypass neg noise thres"]
    #[inline(always)]
    #[must_use]
    pub fn touch_bypass_neg_noise_thres(&mut self) -> TOUCH_BYPASS_NEG_NOISE_THRES_W<7> {
        TOUCH_BYPASS_NEG_NOISE_THRES_W::new(self)
    }
    #[doc = "Bit 8 - bypaas noise thres"]
    #[inline(always)]
    #[must_use]
    pub fn touch_bypass_noise_thres(&mut self) -> TOUCH_BYPASS_NOISE_THRES_W<8> {
        TOUCH_BYPASS_NOISE_THRES_W::new(self)
    }
    #[doc = "Bits 9:10 - smooth filter factor"]
    #[inline(always)]
    #[must_use]
    pub fn touch_smooth_lvl(&mut self) -> TOUCH_SMOOTH_LVL_W<9> {
        TOUCH_SMOOTH_LVL_W::new(self)
    }
    #[doc = "Bits 11:14 - touch jitter step"]
    #[inline(always)]
    #[must_use]
    pub fn touch_jitter_step(&mut self) -> TOUCH_JITTER_STEP_W<11> {
        TOUCH_JITTER_STEP_W::new(self)
    }
    #[doc = "Bits 15:18 - negative threshold counter limit"]
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_limit(&mut self) -> TOUCH_NEG_NOISE_LIMIT_W<15> {
        TOUCH_NEG_NOISE_LIMIT_W::new(self)
    }
    #[doc = "Bits 19:20 - neg noise thres"]
    #[inline(always)]
    #[must_use]
    pub fn touch_neg_noise_thres(&mut self) -> TOUCH_NEG_NOISE_THRES_W<19> {
        TOUCH_NEG_NOISE_THRES_W::new(self)
    }
    #[doc = "Bits 21:22 - noise thres"]
    #[inline(always)]
    #[must_use]
    pub fn touch_noise_thres(&mut self) -> TOUCH_NOISE_THRES_W<21> {
        TOUCH_NOISE_THRES_W::new(self)
    }
    #[doc = "Bits 23:24 - hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn touch_hysteresis(&mut self) -> TOUCH_HYSTERESIS_W<23> {
        TOUCH_HYSTERESIS_W::new(self)
    }
    #[doc = "Bits 25:27 - debounce counter"]
    #[inline(always)]
    #[must_use]
    pub fn touch_debounce(&mut self) -> TOUCH_DEBOUNCE_W<25> {
        TOUCH_DEBOUNCE_W::new(self)
    }
    #[doc = "Bits 28:30 - 0: IIR ? 1: IIR ? 2: IIR 1/8 3: Jitter"]
    #[inline(always)]
    #[must_use]
    pub fn touch_filter_mode(&mut self) -> TOUCH_FILTER_MODE_W<28> {
        TOUCH_FILTER_MODE_W::new(self)
    }
    #[doc = "Bit 31 - touch filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn touch_filter_en(&mut self) -> TOUCH_FILTER_EN_W<31> {
        TOUCH_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_filter_ctrl](index.html) module"]
pub struct TOUCH_FILTER_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_FILTER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_filter_ctrl::R](R) reader structure"]
impl crate::Readable for TOUCH_FILTER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_filter_ctrl::W](W) writer structure"]
impl crate::Writable for TOUCH_FILTER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_FILTER_CTRL to value 0x96aa_8800"]
impl crate::Resettable for TOUCH_FILTER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x96aa_8800;
}
