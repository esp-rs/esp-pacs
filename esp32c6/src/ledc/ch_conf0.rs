#[doc = "Register `CH%s_CONF0` reader"]
pub struct R(crate::R<CH_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_CONF0` writer"]
pub struct W(crate::W<CH_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CONF0_SPEC>;
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
impl From<crate::W<CH_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL` reader - This field is used to select one of timers for channel %s. 0: select timer0, 1: select timer1, 2: select timer2, 3: select timer3"]
pub type TIMER_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER_SEL` writer - This field is used to select one of timers for channel %s. 0: select timer0, 1: select timer1, 2: select timer2, 3: select timer3"]
pub type TIMER_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CH_CONF0_SPEC, 2, O>;
#[doc = "Field `SIG_OUT_EN` reader - Set this bit to enable signal output on channel %s."]
pub type SIG_OUT_EN_R = crate::BitReader;
#[doc = "Field `SIG_OUT_EN` writer - Set this bit to enable signal output on channel %s."]
pub type SIG_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CH_CONF0_SPEC, O>;
#[doc = "Field `IDLE_LV` reader - This bit is used to control the output value when channel %s is inactive (when LEDC_SIG_OUT_EN_CH%s is 0)."]
pub type IDLE_LV_R = crate::BitReader;
#[doc = "Field `IDLE_LV` writer - This bit is used to control the output value when channel %s is inactive (when LEDC_SIG_OUT_EN_CH%s is 0)."]
pub type IDLE_LV_W<'a, const O: u8> = crate::BitWriter<'a, CH_CONF0_SPEC, O>;
#[doc = "Field `PARA_UP` writer - This bit is used to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware."]
pub type PARA_UP_W<'a, const O: u8> = crate::BitWriter<'a, CH_CONF0_SPEC, O>;
#[doc = "Field `OVF_NUM` reader - This register is used to configure the maximum times of overflow minus 1. The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `OVF_NUM` writer - This register is used to configure the maximum times of overflow minus 1. The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CH_CONF0_SPEC, 10, O, u16>;
#[doc = "Field `OVF_CNT_EN` reader - This bit is used to enable the ovf_cnt of channel %s."]
pub type OVF_CNT_EN_R = crate::BitReader;
#[doc = "Field `OVF_CNT_EN` writer - This bit is used to enable the ovf_cnt of channel %s."]
pub type OVF_CNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CH_CONF0_SPEC, O>;
#[doc = "Field `OVF_CNT_RESET` writer - Set this bit to reset the ovf_cnt of channel %s."]
pub type OVF_CNT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CH_CONF0_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - This field is used to select one of timers for channel %s. 0: select timer0, 1: select timer1, 2: select timer2, 3: select timer3"]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to enable signal output on channel %s."]
    #[inline(always)]
    pub fn sig_out_en(&self) -> SIG_OUT_EN_R {
        SIG_OUT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when channel %s is inactive (when LEDC_SIG_OUT_EN_CH%s is 0)."]
    #[inline(always)]
    pub fn idle_lv(&self) -> IDLE_LV_R {
        IDLE_LV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - This register is used to configure the maximum times of overflow minus 1. The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    pub fn ovf_num(&self) -> OVF_NUM_R {
        OVF_NUM_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - This bit is used to enable the ovf_cnt of channel %s."]
    #[inline(always)]
    pub fn ovf_cnt_en(&self) -> OVF_CNT_EN_R {
        OVF_CNT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_CONF0")
            .field("timer_sel", &format_args!("{}", self.timer_sel().bits()))
            .field("sig_out_en", &format_args!("{}", self.sig_out_en().bit()))
            .field("idle_lv", &format_args!("{}", self.idle_lv().bit()))
            .field("ovf_num", &format_args!("{}", self.ovf_num().bits()))
            .field("ovf_cnt_en", &format_args!("{}", self.ovf_cnt_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to select one of timers for channel %s. 0: select timer0, 1: select timer1, 2: select timer2, 3: select timer3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<0> {
        TIMER_SEL_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to enable signal output on channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn sig_out_en(&mut self) -> SIG_OUT_EN_W<2> {
        SIG_OUT_EN_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when channel %s is inactive (when LEDC_SIG_OUT_EN_CH%s is 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idle_lv(&mut self) -> IDLE_LV_W<3> {
        IDLE_LV_W::new(self)
    }
    #[doc = "Bit 4 - This bit is used to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<4> {
        PARA_UP_W::new(self)
    }
    #[doc = "Bits 5:14 - This register is used to configure the maximum times of overflow minus 1. The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_num(&mut self) -> OVF_NUM_W<5> {
        OVF_NUM_W::new(self)
    }
    #[doc = "Bit 15 - This bit is used to enable the ovf_cnt of channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_en(&mut self) -> OVF_CNT_EN_W<15> {
        OVF_CNT_EN_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to reset the ovf_cnt of channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_reset(&mut self) -> OVF_CNT_RESET_W<16> {
        OVF_CNT_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 0 for channel %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_conf0](index.html) module"]
pub struct CH_CONF0_SPEC;
impl crate::RegisterSpec for CH_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_conf0::R](R) reader structure"]
impl crate::Readable for CH_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_conf0::W](W) writer structure"]
impl crate::Writable for CH_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_CONF0 to value 0"]
impl crate::Resettable for CH_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
