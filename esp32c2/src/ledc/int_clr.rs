#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `OVF` writer - Set this bit to clear the LEDC_TIMER0_OVF_INT interrupt."]
pub type OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER1_OVF` writer - Set this bit to clear the LEDC_TIMER1_OVF_INT interrupt."]
pub type TIMER1_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER2_OVF` writer - Set this bit to clear the LEDC_TIMER2_OVF_INT interrupt."]
pub type TIMER2_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER3_OVF` writer - Set this bit to clear the LEDC_TIMER3_OVF_INT interrupt."]
pub type TIMER3_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH0` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
pub type DUTY_CHNG_END_CH0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH1` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
pub type DUTY_CHNG_END_CH1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH2` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
pub type DUTY_CHNG_END_CH2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH3` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
pub type DUTY_CHNG_END_CH3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH4` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
pub type DUTY_CHNG_END_CH4_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH5` writer - Set this bit to clear the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
pub type DUTY_CHNG_END_CH5_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF_CNT_CH0` writer - Set this bit to clear the LEDC_OVF_CNT_CH0_INT interrupt."]
pub type OVF_CNT_CH0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF_CNT_CH1` writer - Set this bit to clear the LEDC_OVF_CNT_CH1_INT interrupt."]
pub type OVF_CNT_CH1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF_CNT_CH2` writer - Set this bit to clear the LEDC_OVF_CNT_CH2_INT interrupt."]
pub type OVF_CNT_CH2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF_CNT_CH3` writer - Set this bit to clear the LEDC_OVF_CNT_CH3_INT interrupt."]
pub type OVF_CNT_CH3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF_CNT_CH4` writer - Set this bit to clear the LEDC_OVF_CNT_CH4_INT interrupt."]
pub type OVF_CNT_CH4_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF_CNT_CH5` writer - Set this bit to clear the LEDC_OVF_CNT_CH5_INT interrupt."]
pub type OVF_CNT_CH5_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the LEDC_TIMER0_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<INT_CLR_SPEC> {
        OVF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the LEDC_TIMER1_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ovf(&mut self) -> TIMER1_OVF_W<INT_CLR_SPEC> {
        TIMER1_OVF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the LEDC_TIMER2_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_ovf(&mut self) -> TIMER2_OVF_W<INT_CLR_SPEC> {
        TIMER2_OVF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the LEDC_TIMER3_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn timer3_ovf(&mut self) -> TIMER3_OVF_W<INT_CLR_SPEC> {
        TIMER3_OVF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch0(&mut self) -> DUTY_CHNG_END_CH0_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch1(&mut self) -> DUTY_CHNG_END_CH1_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch2(&mut self) -> DUTY_CHNG_END_CH2_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch3(&mut self) -> DUTY_CHNG_END_CH3_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH3_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch4(&mut self) -> DUTY_CHNG_END_CH4_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH4_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch5(&mut self) -> DUTY_CHNG_END_CH5_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_CH5_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear the LEDC_OVF_CNT_CH0_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch0(&mut self) -> OVF_CNT_CH0_W<INT_CLR_SPEC> {
        OVF_CNT_CH0_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to clear the LEDC_OVF_CNT_CH1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch1(&mut self) -> OVF_CNT_CH1_W<INT_CLR_SPEC> {
        OVF_CNT_CH1_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to clear the LEDC_OVF_CNT_CH2_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch2(&mut self) -> OVF_CNT_CH2_W<INT_CLR_SPEC> {
        OVF_CNT_CH2_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to clear the LEDC_OVF_CNT_CH3_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch3(&mut self) -> OVF_CNT_CH3_W<INT_CLR_SPEC> {
        OVF_CNT_CH3_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to clear the LEDC_OVF_CNT_CH4_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch4(&mut self) -> OVF_CNT_CH4_W<INT_CLR_SPEC> {
        OVF_CNT_CH4_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to clear the LEDC_OVF_CNT_CH5_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch5(&mut self) -> OVF_CNT_CH5_W<INT_CLR_SPEC> {
        OVF_CNT_CH5_W::new(self, 15)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
