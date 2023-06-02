#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER0_OVF_INT_ST` reader - This is the masked interrupt status bit for the LEDC_TIMER0_OVF_INT interrupt when LEDC_TIMER0_OVF_INT_ENA is set to 1."]
pub type TIMER0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER1_OVF_INT_ST` reader - This is the masked interrupt status bit for the LEDC_TIMER1_OVF_INT interrupt when LEDC_TIMER1_OVF_INT_ENA is set to 1."]
pub type TIMER1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER2_OVF_INT_ST` reader - This is the masked interrupt status bit for the LEDC_TIMER2_OVF_INT interrupt when LEDC_TIMER2_OVF_INT_ENA is set to 1."]
pub type TIMER2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER3_OVF_INT_ST` reader - This is the masked interrupt status bit for the LEDC_TIMER3_OVF_INT interrupt when LEDC_TIMER3_OVF_INT_ENA is set to 1."]
pub type TIMER3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_ST` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt when LEDC_DUTY_CHNG_END_CH0_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_ST` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt when LEDC_DUTY_CHNG_END_CH1_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_ST` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt when LEDC_DUTY_CHNG_END_CH2_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH2_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_ST` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt when LEDC_DUTY_CHNG_END_CH3_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_ST` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt when LEDC_DUTY_CHNG_END_CH4_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH4_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_ST` reader - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt when LEDC_DUTY_CHNG_END_CH5_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH5_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH0_INT_ST` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH0_INT interrupt when LEDC_OVF_CNT_CH0_INT_ENA is set to 1."]
pub type OVF_CNT_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH1_INT_ST` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH1_INT interrupt when LEDC_OVF_CNT_CH1_INT_ENA is set to 1."]
pub type OVF_CNT_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH2_INT_ST` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH2_INT interrupt when LEDC_OVF_CNT_CH2_INT_ENA is set to 1."]
pub type OVF_CNT_CH2_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH3_INT_ST` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH3_INT interrupt when LEDC_OVF_CNT_CH3_INT_ENA is set to 1."]
pub type OVF_CNT_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH4_INT_ST` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH4_INT interrupt when LEDC_OVF_CNT_CH4_INT_ENA is set to 1."]
pub type OVF_CNT_CH4_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH5_INT_ST` reader - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH5_INT interrupt when LEDC_OVF_CNT_CH5_INT_ENA is set to 1."]
pub type OVF_CNT_CH5_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the masked interrupt status bit for the LEDC_TIMER0_OVF_INT interrupt when LEDC_TIMER0_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer0_ovf_int_st(&self) -> TIMER0_OVF_INT_ST_R {
        TIMER0_OVF_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the masked interrupt status bit for the LEDC_TIMER1_OVF_INT interrupt when LEDC_TIMER1_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer1_ovf_int_st(&self) -> TIMER1_OVF_INT_ST_R {
        TIMER1_OVF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the masked interrupt status bit for the LEDC_TIMER2_OVF_INT interrupt when LEDC_TIMER2_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer2_ovf_int_st(&self) -> TIMER2_OVF_INT_ST_R {
        TIMER2_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the masked interrupt status bit for the LEDC_TIMER3_OVF_INT interrupt when LEDC_TIMER3_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer3_ovf_int_st(&self) -> TIMER3_OVF_INT_ST_R {
        TIMER3_OVF_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt when LEDC_DUTY_CHNG_END_CH0_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_st(&self) -> DUTY_CHNG_END_CH0_INT_ST_R {
        DUTY_CHNG_END_CH0_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt when LEDC_DUTY_CHNG_END_CH1_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_st(&self) -> DUTY_CHNG_END_CH1_INT_ST_R {
        DUTY_CHNG_END_CH1_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt when LEDC_DUTY_CHNG_END_CH2_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_st(&self) -> DUTY_CHNG_END_CH2_INT_ST_R {
        DUTY_CHNG_END_CH2_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt when LEDC_DUTY_CHNG_END_CH3_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_st(&self) -> DUTY_CHNG_END_CH3_INT_ST_R {
        DUTY_CHNG_END_CH3_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt when LEDC_DUTY_CHNG_END_CH4_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_st(&self) -> DUTY_CHNG_END_CH4_INT_ST_R {
        DUTY_CHNG_END_CH4_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the masked interrupt status bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt when LEDC_DUTY_CHNG_END_CH5_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_st(&self) -> DUTY_CHNG_END_CH5_INT_ST_R {
        DUTY_CHNG_END_CH5_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH0_INT interrupt when LEDC_OVF_CNT_CH0_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_st(&self) -> OVF_CNT_CH0_INT_ST_R {
        OVF_CNT_CH0_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH1_INT interrupt when LEDC_OVF_CNT_CH1_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_st(&self) -> OVF_CNT_CH1_INT_ST_R {
        OVF_CNT_CH1_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH2_INT interrupt when LEDC_OVF_CNT_CH2_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_st(&self) -> OVF_CNT_CH2_INT_ST_R {
        OVF_CNT_CH2_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH3_INT interrupt when LEDC_OVF_CNT_CH3_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_st(&self) -> OVF_CNT_CH3_INT_ST_R {
        OVF_CNT_CH3_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH4_INT interrupt when LEDC_OVF_CNT_CH4_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_st(&self) -> OVF_CNT_CH4_INT_ST_R {
        OVF_CNT_CH4_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This is the masked interrupt status bit for the LEDC_OVF_CNT_CH5_INT interrupt when LEDC_OVF_CNT_CH5_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_st(&self) -> OVF_CNT_CH5_INT_ST_R {
        OVF_CNT_CH5_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "timer0_ovf_int_st",
                &format_args!("{}", self.timer0_ovf_int_st().bit()),
            )
            .field(
                "timer1_ovf_int_st",
                &format_args!("{}", self.timer1_ovf_int_st().bit()),
            )
            .field(
                "timer2_ovf_int_st",
                &format_args!("{}", self.timer2_ovf_int_st().bit()),
            )
            .field(
                "timer3_ovf_int_st",
                &format_args!("{}", self.timer3_ovf_int_st().bit()),
            )
            .field(
                "duty_chng_end_ch0_int_st",
                &format_args!("{}", self.duty_chng_end_ch0_int_st().bit()),
            )
            .field(
                "duty_chng_end_ch1_int_st",
                &format_args!("{}", self.duty_chng_end_ch1_int_st().bit()),
            )
            .field(
                "duty_chng_end_ch2_int_st",
                &format_args!("{}", self.duty_chng_end_ch2_int_st().bit()),
            )
            .field(
                "duty_chng_end_ch3_int_st",
                &format_args!("{}", self.duty_chng_end_ch3_int_st().bit()),
            )
            .field(
                "duty_chng_end_ch4_int_st",
                &format_args!("{}", self.duty_chng_end_ch4_int_st().bit()),
            )
            .field(
                "duty_chng_end_ch5_int_st",
                &format_args!("{}", self.duty_chng_end_ch5_int_st().bit()),
            )
            .field(
                "ovf_cnt_ch0_int_st",
                &format_args!("{}", self.ovf_cnt_ch0_int_st().bit()),
            )
            .field(
                "ovf_cnt_ch1_int_st",
                &format_args!("{}", self.ovf_cnt_ch1_int_st().bit()),
            )
            .field(
                "ovf_cnt_ch2_int_st",
                &format_args!("{}", self.ovf_cnt_ch2_int_st().bit()),
            )
            .field(
                "ovf_cnt_ch3_int_st",
                &format_args!("{}", self.ovf_cnt_ch3_int_st().bit()),
            )
            .field(
                "ovf_cnt_ch4_int_st",
                &format_args!("{}", self.ovf_cnt_ch4_int_st().bit()),
            )
            .field(
                "ovf_cnt_ch5_int_st",
                &format_args!("{}", self.ovf_cnt_ch5_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Masked interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
