#[doc = "Register `CH%s_CONF0` reader"]
pub type R = crate::R<CH_CONF0_SPEC>;
#[doc = "Register `CH%s_CONF0` writer"]
pub type W = crate::W<CH_CONF0_SPEC>;
#[doc = "Field `TIMER_SEL_CH` reader - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
pub type TIMER_SEL_CH_R = crate::FieldReader;
#[doc = "Field `TIMER_SEL_CH` writer - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
pub type TIMER_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIG_OUT_EN_CH` reader - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
pub type SIG_OUT_EN_CH_R = crate::BitReader;
#[doc = "Field `SIG_OUT_EN_CH` writer - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
pub type SIG_OUT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_LV_CH` reader - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
pub type IDLE_LV_CH_R = crate::BitReader;
#[doc = "Field `IDLE_LV_CH` writer - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
pub type IDLE_LV_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARA_UP_CH` writer - Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\\\0: Invalid. No effect\\\\1: Update"]
pub type PARA_UP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_NUM_CH` reader - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_CH_R = crate::FieldReader<u16>;
#[doc = "Field `OVF_NUM_CH` writer - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OVF_CNT_EN_CH` reader - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
pub type OVF_CNT_EN_CH_R = crate::BitReader;
#[doc = "Field `OVF_CNT_EN_CH` writer - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
pub type OVF_CNT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_RESET_CH` writer - Configures whether or not to reset the ovf_cnt of channel %s.\\\\0: Invalid. No effect\\\\1: Reset the ovf_cnt"]
pub type OVF_CNT_RESET_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> TIMER_SEL_CH_R {
        TIMER_SEL_CH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> SIG_OUT_EN_CH_R {
        SIG_OUT_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> IDLE_LV_CH_R {
        IDLE_LV_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> OVF_NUM_CH_R {
        OVF_NUM_CH_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> OVF_CNT_EN_CH_R {
        OVF_CNT_EN_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_CONF0")
            .field(
                "timer_sel_ch",
                &format_args!("{}", self.timer_sel_ch().bits()),
            )
            .field(
                "sig_out_en_ch",
                &format_args!("{}", self.sig_out_en_ch().bit()),
            )
            .field("idle_lv_ch", &format_args!("{}", self.idle_lv_ch().bit()))
            .field("ovf_num_ch", &format_args!("{}", self.ovf_num_ch().bits()))
            .field(
                "ovf_cnt_en_ch",
                &format_args!("{}", self.ovf_cnt_en_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel_ch(&mut self) -> TIMER_SEL_CH_W<CH_CONF0_SPEC> {
        TIMER_SEL_CH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sig_out_en_ch(&mut self) -> SIG_OUT_EN_CH_W<CH_CONF0_SPEC> {
        SIG_OUT_EN_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
    #[inline(always)]
    #[must_use]
    pub fn idle_lv_ch(&mut self) -> IDLE_LV_CH_W<CH_CONF0_SPEC> {
        IDLE_LV_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\\\0: Invalid. No effect\\\\1: Update"]
    #[inline(always)]
    #[must_use]
    pub fn para_up_ch(&mut self) -> PARA_UP_CH_W<CH_CONF0_SPEC> {
        PARA_UP_CH_W::new(self, 4)
    }
    #[doc = "Bits 5:14 - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_num_ch(&mut self) -> OVF_NUM_CH_W<CH_CONF0_SPEC> {
        OVF_NUM_CH_W::new(self, 5)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_en_ch(&mut self) -> OVF_CNT_EN_CH_W<CH_CONF0_SPEC> {
        OVF_CNT_EN_CH_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to reset the ovf_cnt of channel %s.\\\\0: Invalid. No effect\\\\1: Reset the ovf_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_reset_ch(&mut self) -> OVF_CNT_RESET_CH_W<CH_CONF0_SPEC> {
        OVF_CNT_RESET_CH_W::new(self, 16)
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
#[doc = "Configuration register 0 for channel %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_CONF0_SPEC;
impl crate::RegisterSpec for CH_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_conf0::R`](R) reader structure"]
impl crate::Readable for CH_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_conf0::W`](W) writer structure"]
impl crate::Writable for CH_CONF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_CONF0 to value 0"]
impl crate::Resettable for CH_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
