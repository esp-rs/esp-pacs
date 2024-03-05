#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TIMER0_OVF_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_TIMER0_OVF_INT."]
pub type TIMER0_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_OVF_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_TIMER1_OVF_INT."]
pub type TIMER1_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_OVF_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_TIMER2_OVF_INT."]
pub type TIMER2_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_OVF_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_TIMER3_OVF_INT."]
pub type TIMER3_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH0_INT."]
pub type DUTY_CHNG_END_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH1_INT."]
pub type DUTY_CHNG_END_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH2_INT."]
pub type DUTY_CHNG_END_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH3_INT."]
pub type DUTY_CHNG_END_CH3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH4_INT."]
pub type DUTY_CHNG_END_CH4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH5_INT."]
pub type DUTY_CHNG_END_CH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH6_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH6_INT."]
pub type DUTY_CHNG_END_CH6_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH7_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH7_INT."]
pub type DUTY_CHNG_END_CH7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH0_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH0_INT."]
pub type OVF_CNT_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH1_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH1_INT."]
pub type OVF_CNT_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH2_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH2_INT."]
pub type OVF_CNT_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH3_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH3_INT."]
pub type OVF_CNT_CH3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH4_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH4_INT."]
pub type OVF_CNT_CH4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH5_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH5_INT."]
pub type OVF_CNT_CH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH6_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH6_INT."]
pub type OVF_CNT_CH6_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH7_INT_CLR` writer - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH7_INT."]
pub type OVF_CNT_CH7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear bit: Write 1 to clear LEDC_TIMER0_OVF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ovf_int_clr(&mut self) -> TIMER0_OVF_INT_CLR_W<INT_CLR_SPEC> {
        TIMER0_OVF_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit: Write 1 to clear LEDC_TIMER1_OVF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ovf_int_clr(&mut self) -> TIMER1_OVF_INT_CLR_W<INT_CLR_SPEC> {
        TIMER1_OVF_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit: Write 1 to clear LEDC_TIMER2_OVF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_ovf_int_clr(&mut self) -> TIMER2_OVF_INT_CLR_W<INT_CLR_SPEC> {
        TIMER2_OVF_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit: Write 1 to clear LEDC_TIMER3_OVF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn timer3_ovf_int_clr(&mut self) -> TIMER3_OVF_INT_CLR_W<INT_CLR_SPEC> {
        TIMER3_OVF_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH0_INT."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch0_int_clr(&mut self) -> DUTY_CHNG_END_CH0_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH0_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH1_INT."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch1_int_clr(&mut self) -> DUTY_CHNG_END_CH1_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH1_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH2_INT."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch2_int_clr(&mut self) -> DUTY_CHNG_END_CH2_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH2_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH3_INT."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch3_int_clr(&mut self) -> DUTY_CHNG_END_CH3_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH3_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH4_INT."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch4_int_clr(&mut self) -> DUTY_CHNG_END_CH4_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH4_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH5_INT."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch5_int_clr(&mut self) -> DUTY_CHNG_END_CH5_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH5_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH6_INT."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch6_int_clr(&mut self) -> DUTY_CHNG_END_CH6_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH6_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH7_INT."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch7_int_clr(&mut self) -> DUTY_CHNG_END_CH7_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH7_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH0_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch0_int_clr(&mut self) -> OVF_CNT_CH0_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_CH0_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH1_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch1_int_clr(&mut self) -> OVF_CNT_CH1_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_CH1_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH2_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch2_int_clr(&mut self) -> OVF_CNT_CH2_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_CH2_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH3_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch3_int_clr(&mut self) -> OVF_CNT_CH3_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_CH3_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH4_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch4_int_clr(&mut self) -> OVF_CNT_CH4_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_CH4_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH5_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch5_int_clr(&mut self) -> OVF_CNT_CH5_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_CH5_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH6_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch6_int_clr(&mut self) -> OVF_CNT_CH6_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_CH6_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear bit: Write 1 to clear LEDC_OVF_CNT_CH7_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch7_int_clr(&mut self) -> OVF_CNT_CH7_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_CH7_INT_CLR_W::new(self, 19)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
