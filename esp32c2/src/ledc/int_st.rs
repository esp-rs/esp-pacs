#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `OVF` reader - This is the masked interrupt status bit for the LEDC_TIMER0_OVF_INT interrupt when LEDC_TIMER0_OVF_INT_ENA is set to 1."]
pub type OVF_R = crate::BitReader;
#[doc = "Field `TIMER1_OVF` reader - This is the masked interrupt status bit for the LEDC_TIMER1_OVF_INT interrupt when LEDC_TIMER1_OVF_INT_ENA is set to 1."]
pub type TIMER1_OVF_R = crate::BitReader;
#[doc = "Field `TIMER2_OVF` reader - This is the masked interrupt status bit for the LEDC_TIMER2_OVF_INT interrupt when LEDC_TIMER2_OVF_INT_ENA is set to 1."]
pub type TIMER2_OVF_R = crate::BitReader;
#[doc = "Field `TIMER3_OVF` reader - This is the masked interrupt status bit for the LEDC_TIMER3_OVF_INT interrupt when LEDC_TIMER3_OVF_INT_ENA is set to 1."]
pub type TIMER3_OVF_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH0` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt when LEDC_DUTY_CHNG_END_CH0_INT_ENAIS set to 1."]
pub type DUTY_CHNG_END_CH0_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH1` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt when LEDC_DUTY_CHNG_END_CH1_INT_ENAIS set to 1."]
pub type DUTY_CHNG_END_CH1_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH2` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt when LEDC_DUTY_CHNG_END_CH2_INT_ENAIS set to 1."]
pub type DUTY_CHNG_END_CH2_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH3` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt when LEDC_DUTY_CHNG_END_CH3_INT_ENAIS set to 1."]
pub type DUTY_CHNG_END_CH3_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH4` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt when LEDC_DUTY_CHNG_END_CH4_INT_ENAIS set to 1."]
pub type DUTY_CHNG_END_CH4_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH5` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt when LEDC_DUTY_CHNG_END_CH5_INT_ENAIS set to 1."]
pub type DUTY_CHNG_END_CH5_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH0` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH0_INT interrupt when LEDC_OVF_CNT_CH0_INT_ENA is set to 1."]
pub type OVF_CNT_CH0_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH1` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH1_INT interrupt when LEDC_OVF_CNT_CH1_INT_ENA is set to 1."]
pub type OVF_CNT_CH1_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH2` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH2_INT interrupt when LEDC_OVF_CNT_CH2_INT_ENA is set to 1."]
pub type OVF_CNT_CH2_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH3` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH3_INT interrupt when LEDC_OVF_CNT_CH3_INT_ENA is set to 1."]
pub type OVF_CNT_CH3_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH4` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH4_INT interrupt when LEDC_OVF_CNT_CH4_INT_ENA is set to 1."]
pub type OVF_CNT_CH4_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH5` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH5_INT interrupt when LEDC_OVF_CNT_CH5_INT_ENA is set to 1."]
pub type OVF_CNT_CH5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the masked interrupt status bit for the LEDC_TIMER0_OVF_INT interrupt when LEDC_TIMER0_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the masked interrupt status bit for the LEDC_TIMER1_OVF_INT interrupt when LEDC_TIMER1_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer1_ovf(&self) -> TIMER1_OVF_R {
        TIMER1_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the masked interrupt status bit for the LEDC_TIMER2_OVF_INT interrupt when LEDC_TIMER2_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer2_ovf(&self) -> TIMER2_OVF_R {
        TIMER2_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the masked interrupt status bit for the LEDC_TIMER3_OVF_INT interrupt when LEDC_TIMER3_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer3_ovf(&self) -> TIMER3_OVF_R {
        TIMER3_OVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt when LEDC_DUTY_CHNG_END_CH0_INT_ENAIS set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch0(&self) -> DUTY_CHNG_END_CH0_R {
        DUTY_CHNG_END_CH0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt when LEDC_DUTY_CHNG_END_CH1_INT_ENAIS set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch1(&self) -> DUTY_CHNG_END_CH1_R {
        DUTY_CHNG_END_CH1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt when LEDC_DUTY_CHNG_END_CH2_INT_ENAIS set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch2(&self) -> DUTY_CHNG_END_CH2_R {
        DUTY_CHNG_END_CH2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt when LEDC_DUTY_CHNG_END_CH3_INT_ENAIS set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch3(&self) -> DUTY_CHNG_END_CH3_R {
        DUTY_CHNG_END_CH3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt when LEDC_DUTY_CHNG_END_CH4_INT_ENAIS set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch4(&self) -> DUTY_CHNG_END_CH4_R {
        DUTY_CHNG_END_CH4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt when LEDC_DUTY_CHNG_END_CH5_INT_ENAIS set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch5(&self) -> DUTY_CHNG_END_CH5_R {
        DUTY_CHNG_END_CH5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH0_INT interrupt when LEDC_OVF_CNT_CH0_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch0(&self) -> OVF_CNT_CH0_R {
        OVF_CNT_CH0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH1_INT interrupt when LEDC_OVF_CNT_CH1_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch1(&self) -> OVF_CNT_CH1_R {
        OVF_CNT_CH1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH2_INT interrupt when LEDC_OVF_CNT_CH2_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch2(&self) -> OVF_CNT_CH2_R {
        OVF_CNT_CH2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH3_INT interrupt when LEDC_OVF_CNT_CH3_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch3(&self) -> OVF_CNT_CH3_R {
        OVF_CNT_CH3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH4_INT interrupt when LEDC_OVF_CNT_CH4_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch4(&self) -> OVF_CNT_CH4_R {
        OVF_CNT_CH4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH5_INT interrupt when LEDC_OVF_CNT_CH5_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch5(&self) -> OVF_CNT_CH5_R {
        OVF_CNT_CH5_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("ovf", &format_args!("{}", self.ovf().bit()))
            .field("timer1_ovf", &format_args!("{}", self.timer1_ovf().bit()))
            .field("timer2_ovf", &format_args!("{}", self.timer2_ovf().bit()))
            .field("timer3_ovf", &format_args!("{}", self.timer3_ovf().bit()))
            .field(
                "duty_chng_end_ch0",
                &format_args!("{}", self.duty_chng_end_ch0().bit()),
            )
            .field(
                "duty_chng_end_ch1",
                &format_args!("{}", self.duty_chng_end_ch1().bit()),
            )
            .field(
                "duty_chng_end_ch2",
                &format_args!("{}", self.duty_chng_end_ch2().bit()),
            )
            .field(
                "duty_chng_end_ch3",
                &format_args!("{}", self.duty_chng_end_ch3().bit()),
            )
            .field(
                "duty_chng_end_ch4",
                &format_args!("{}", self.duty_chng_end_ch4().bit()),
            )
            .field(
                "duty_chng_end_ch5",
                &format_args!("{}", self.duty_chng_end_ch5().bit()),
            )
            .field("ovf_cnt_ch0", &format_args!("{}", self.ovf_cnt_ch0().bit()))
            .field("ovf_cnt_ch1", &format_args!("{}", self.ovf_cnt_ch1().bit()))
            .field("ovf_cnt_ch2", &format_args!("{}", self.ovf_cnt_ch2().bit()))
            .field("ovf_cnt_ch3", &format_args!("{}", self.ovf_cnt_ch3().bit()))
            .field("ovf_cnt_ch4", &format_args!("{}", self.ovf_cnt_ch4().bit()))
            .field("ovf_cnt_ch5", &format_args!("{}", self.ovf_cnt_ch5().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
