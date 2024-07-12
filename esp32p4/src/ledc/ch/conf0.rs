#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `TIMER_SEL` reader - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
pub type TIMER_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER_SEL` writer - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
pub type TIMER_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIG_OUT_EN` reader - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
pub type SIG_OUT_EN_R = crate::BitReader;
#[doc = "Field `SIG_OUT_EN` writer - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
pub type SIG_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_LV` reader - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
pub type IDLE_LV_R = crate::BitReader;
#[doc = "Field `IDLE_LV` writer - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
pub type IDLE_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARA_UP` writer - Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\\\0: Invalid. No effect\\\\1: Update"]
pub type PARA_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_NUM` reader - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `OVF_NUM` writer - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OVF_CNT_EN` reader - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
pub type OVF_CNT_EN_R = crate::BitReader;
#[doc = "Field `OVF_CNT_EN` writer - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
pub type OVF_CNT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_RESET` writer - Configures whether or not to reset the ovf_cnt of channel %s.\\\\0: Invalid. No effect\\\\1: Reset the ovf_cnt"]
pub type OVF_CNT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
    #[inline(always)]
    pub fn sig_out_en(&self) -> SIG_OUT_EN_R {
        SIG_OUT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
    #[inline(always)]
    pub fn idle_lv(&self) -> IDLE_LV_R {
        IDLE_LV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    pub fn ovf_num(&self) -> OVF_NUM_R {
        OVF_NUM_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ovf_cnt_en(&self) -> OVF_CNT_EN_R {
        OVF_CNT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("timer_sel", &self.timer_sel())
            .field("sig_out_en", &self.sig_out_en())
            .field("idle_lv", &self.idle_lv())
            .field("ovf_num", &self.ovf_num())
            .field("ovf_cnt_en", &self.ovf_cnt_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<CONF0_SPEC> {
        TIMER_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sig_out_en(&mut self) -> SIG_OUT_EN_W<CONF0_SPEC> {
        SIG_OUT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
    #[inline(always)]
    #[must_use]
    pub fn idle_lv(&mut self) -> IDLE_LV_W<CONF0_SPEC> {
        IDLE_LV_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\\\0: Invalid. No effect\\\\1: Update"]
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<CONF0_SPEC> {
        PARA_UP_W::new(self, 4)
    }
    #[doc = "Bits 5:14 - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_num(&mut self) -> OVF_NUM_W<CONF0_SPEC> {
        OVF_NUM_W::new(self, 5)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_en(&mut self) -> OVF_CNT_EN_W<CONF0_SPEC> {
        OVF_CNT_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to reset the ovf_cnt of channel %s.\\\\0: Invalid. No effect\\\\1: Reset the ovf_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_reset(&mut self) -> OVF_CNT_RESET_W<CONF0_SPEC> {
        OVF_CNT_RESET_W::new(self, 16)
    }
}
#[doc = "Configuration register 0 for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
