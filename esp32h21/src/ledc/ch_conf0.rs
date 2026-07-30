#[doc = "Register `CH%s_CONF0` reader"]
pub type R = crate::R<CH_CONF0_SPEC>;
#[doc = "Register `CH%s_CONF0` writer"]
pub type W = crate::W<CH_CONF0_SPEC>;
#[doc = "Field `TIMER_SEL_CH` reader - This field is used to select one of timers for channel %s. 0: select timer0, 1: select timer1, 2: select timer2, 3: select timer3"]
pub type TIMER_SEL_CH_R = crate::FieldReader;
#[doc = "Field `TIMER_SEL_CH` writer - This field is used to select one of timers for channel %s. 0: select timer0, 1: select timer1, 2: select timer2, 3: select timer3"]
pub type TIMER_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIG_OUT_EN_CH` reader - Set this bit to enable signal output on channel %s."]
pub type SIG_OUT_EN_CH_R = crate::BitReader;
#[doc = "Field `SIG_OUT_EN_CH` writer - Set this bit to enable signal output on channel %s."]
pub type SIG_OUT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_LV_CH` reader - This bit is used to control the output value when channel %s is inactive (when LEDC_SIG_OUT_EN_CH%s is 0)."]
pub type IDLE_LV_CH_R = crate::BitReader;
#[doc = "Field `IDLE_LV_CH` writer - This bit is used to control the output value when channel %s is inactive (when LEDC_SIG_OUT_EN_CH%s is 0)."]
pub type IDLE_LV_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARA_UP_CH` writer - This bit is used to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware."]
pub type PARA_UP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_NUM_CH` reader - This register is used to configure the maximum times of overflow minus 1. The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_CH_R = crate::FieldReader<u16>;
#[doc = "Field `OVF_NUM_CH` writer - This register is used to configure the maximum times of overflow minus 1. The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OVF_CNT_EN_CH` reader - This bit is used to enable the ovf_cnt of channel %s."]
pub type OVF_CNT_EN_CH_R = crate::BitReader;
#[doc = "Field `OVF_CNT_EN_CH` writer - This bit is used to enable the ovf_cnt of channel %s."]
pub type OVF_CNT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_RESET_CH` writer - Set this bit to reset the ovf_cnt of channel %s."]
pub type OVF_CNT_RESET_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - This field is used to select one of timers for channel %s. 0: select timer0, 1: select timer1, 2: select timer2, 3: select timer3"]
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> TIMER_SEL_CH_R {
        TIMER_SEL_CH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to enable signal output on channel %s."]
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> SIG_OUT_EN_CH_R {
        SIG_OUT_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when channel %s is inactive (when LEDC_SIG_OUT_EN_CH%s is 0)."]
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> IDLE_LV_CH_R {
        IDLE_LV_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - This register is used to configure the maximum times of overflow minus 1. The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> OVF_NUM_CH_R {
        OVF_NUM_CH_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - This bit is used to enable the ovf_cnt of channel %s."]
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> OVF_CNT_EN_CH_R {
        OVF_CNT_EN_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_CONF0")
            .field("timer_sel_ch", &self.timer_sel_ch())
            .field("sig_out_en_ch", &self.sig_out_en_ch())
            .field("idle_lv_ch", &self.idle_lv_ch())
            .field("ovf_num_ch", &self.ovf_num_ch())
            .field("ovf_cnt_en_ch", &self.ovf_cnt_en_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to select one of timers for channel %s. 0: select timer0, 1: select timer1, 2: select timer2, 3: select timer3"]
    #[inline(always)]
    pub fn timer_sel_ch(&mut self) -> TIMER_SEL_CH_W<'_, CH_CONF0_SPEC> {
        TIMER_SEL_CH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to enable signal output on channel %s."]
    #[inline(always)]
    pub fn sig_out_en_ch(&mut self) -> SIG_OUT_EN_CH_W<'_, CH_CONF0_SPEC> {
        SIG_OUT_EN_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when channel %s is inactive (when LEDC_SIG_OUT_EN_CH%s is 0)."]
    #[inline(always)]
    pub fn idle_lv_ch(&mut self) -> IDLE_LV_CH_W<'_, CH_CONF0_SPEC> {
        IDLE_LV_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is used to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware."]
    #[inline(always)]
    pub fn para_up_ch(&mut self) -> PARA_UP_CH_W<'_, CH_CONF0_SPEC> {
        PARA_UP_CH_W::new(self, 4)
    }
    #[doc = "Bits 5:14 - This register is used to configure the maximum times of overflow minus 1. The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    pub fn ovf_num_ch(&mut self) -> OVF_NUM_CH_W<'_, CH_CONF0_SPEC> {
        OVF_NUM_CH_W::new(self, 5)
    }
    #[doc = "Bit 15 - This bit is used to enable the ovf_cnt of channel %s."]
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&mut self) -> OVF_CNT_EN_CH_W<'_, CH_CONF0_SPEC> {
        OVF_CNT_EN_CH_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to reset the ovf_cnt of channel %s."]
    #[inline(always)]
    pub fn ovf_cnt_reset_ch(&mut self) -> OVF_CNT_RESET_CH_W<'_, CH_CONF0_SPEC> {
        OVF_CNT_RESET_CH_W::new(self, 16)
    }
}
#[doc = "Configuration register 0 for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_CONF0_SPEC;
impl crate::RegisterSpec for CH_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_conf0::R`](R) reader structure"]
impl crate::Readable for CH_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_conf0::W`](W) writer structure"]
impl crate::Writable for CH_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%s_CONF0 to value 0"]
impl crate::Resettable for CH_CONF0_SPEC {}
