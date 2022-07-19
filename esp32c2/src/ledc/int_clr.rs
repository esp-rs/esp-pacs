#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_OVF_INT_CLR` writer - Set this bit to clear the LEDC_TIMER0_OVF_INT interrupt."]
pub type TIMER0_OVF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 0>;
#[doc = "Field `TIMER1_OVF_INT_CLR` writer - Set this bit to clear the LEDC_TIMER1_OVF_INT interrupt."]
pub type TIMER1_OVF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 1>;
#[doc = "Field `TIMER2_OVF_INT_CLR` writer - Set this bit to clear the LEDC_TIMER2_OVF_INT interrupt."]
pub type TIMER2_OVF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 2>;
#[doc = "Field `TIMER3_OVF_INT_CLR` writer - Set this bit to clear the LEDC_TIMER3_OVF_INT interrupt."]
pub type TIMER3_OVF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 3>;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_CLR` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
pub type DUTY_CHNG_END_CH0_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 4>;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_CLR` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
pub type DUTY_CHNG_END_CH1_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 5>;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_CLR` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
pub type DUTY_CHNG_END_CH2_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 6>;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_CLR` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
pub type DUTY_CHNG_END_CH3_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 7>;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_CLR` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
pub type DUTY_CHNG_END_CH4_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 8>;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_CLR` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
pub type DUTY_CHNG_END_CH5_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 9>;
#[doc = "Field `OVF_CNT_CH0_INT_CLR` writer - Set this bit to clear the LEDC_OVF_CNT_CH0_INT interrupt."]
pub type OVF_CNT_CH0_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 10>;
#[doc = "Field `OVF_CNT_CH1_INT_CLR` writer - Set this bit to clear the LEDC_OVF_CNT_CH1_INT interrupt."]
pub type OVF_CNT_CH1_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 11>;
#[doc = "Field `OVF_CNT_CH2_INT_CLR` writer - Set this bit to clear the LEDC_OVF_CNT_CH2_INT interrupt."]
pub type OVF_CNT_CH2_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 12>;
#[doc = "Field `OVF_CNT_CH3_INT_CLR` writer - Set this bit to clear the LEDC_OVF_CNT_CH3_INT interrupt."]
pub type OVF_CNT_CH3_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 13>;
#[doc = "Field `OVF_CNT_CH4_INT_CLR` writer - Set this bit to clear the LEDC_OVF_CNT_CH4_INT interrupt."]
pub type OVF_CNT_CH4_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 14>;
#[doc = "Field `OVF_CNT_CH5_INT_CLR` writer - Set this bit to clear the LEDC_OVF_CNT_CH5_INT interrupt."]
pub type OVF_CNT_CH5_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 15>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the LEDC_TIMER0_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer0_ovf_int_clr(&mut self) -> TIMER0_OVF_INT_CLR_W {
        TIMER0_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the LEDC_TIMER1_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer1_ovf_int_clr(&mut self) -> TIMER1_OVF_INT_CLR_W {
        TIMER1_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the LEDC_TIMER2_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer2_ovf_int_clr(&mut self) -> TIMER2_OVF_INT_CLR_W {
        TIMER2_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the LEDC_TIMER3_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer3_ovf_int_clr(&mut self) -> TIMER3_OVF_INT_CLR_W {
        TIMER3_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_clr(&mut self) -> DUTY_CHNG_END_CH0_INT_CLR_W {
        DUTY_CHNG_END_CH0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_clr(&mut self) -> DUTY_CHNG_END_CH1_INT_CLR_W {
        DUTY_CHNG_END_CH1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_clr(&mut self) -> DUTY_CHNG_END_CH2_INT_CLR_W {
        DUTY_CHNG_END_CH2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_clr(&mut self) -> DUTY_CHNG_END_CH3_INT_CLR_W {
        DUTY_CHNG_END_CH3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_clr(&mut self) -> DUTY_CHNG_END_CH4_INT_CLR_W {
        DUTY_CHNG_END_CH4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_clr(&mut self) -> DUTY_CHNG_END_CH5_INT_CLR_W {
        DUTY_CHNG_END_CH5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear the LEDC_OVF_CNT_CH0_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_clr(&mut self) -> OVF_CNT_CH0_INT_CLR_W {
        OVF_CNT_CH0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the LEDC_OVF_CNT_CH1_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_clr(&mut self) -> OVF_CNT_CH1_INT_CLR_W {
        OVF_CNT_CH1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear the LEDC_OVF_CNT_CH2_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_clr(&mut self) -> OVF_CNT_CH2_INT_CLR_W {
        OVF_CNT_CH2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear the LEDC_OVF_CNT_CH3_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_clr(&mut self) -> OVF_CNT_CH3_INT_CLR_W {
        OVF_CNT_CH3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear the LEDC_OVF_CNT_CH4_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_clr(&mut self) -> OVF_CNT_CH4_INT_CLR_W {
        OVF_CNT_CH4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear the LEDC_OVF_CNT_CH5_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_clr(&mut self) -> OVF_CNT_CH5_INT_CLR_W {
        OVF_CNT_CH5_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
