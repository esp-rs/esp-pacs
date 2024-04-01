#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TIMER_OVF(0-3)` reader - This is the masked interrupt status bit for the TIMER%s_OVF interrupt when LEDC.INT_ENA.TIMERx_OVF is set to 1."]
pub type TIMER_OVF_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH(0-7)` reader - This is the masked interrupt status bit for the DUTY_CHNG_END_CH%s interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
pub type DUTY_CHNG_END_CH_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH(0-7)` reader - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH%s interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
pub type OVF_CNT_CH_R = crate::BitReader;
impl R {
    #[doc = "This is the masked interrupt status bit for the TIMER(0-3)_OVF interrupt when LEDC.INT_ENA.TIMERx_OVF is set to 1."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `TIMER0_OVF` field"]
    #[inline(always)]
    pub fn timer_ovf(&self, n: u8) -> TIMER_OVF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TIMER_OVF_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "This is the masked interrupt status bit for the TIMER(0-3)_OVF interrupt when LEDC.INT_ENA.TIMERx_OVF is set to 1."]
    #[inline(always)]
    pub fn timer_ovf_iter(&self) -> impl Iterator<Item = TIMER_OVF_R> + '_ {
        (0..4).map(move |n| TIMER_OVF_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - This is the masked interrupt status bit for the TIMER0_OVF interrupt when LEDC.INT_ENA.TIMERx_OVF is set to 1."]
    #[inline(always)]
    pub fn timer0_ovf(&self) -> TIMER_OVF_R {
        TIMER_OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the masked interrupt status bit for the TIMER1_OVF interrupt when LEDC.INT_ENA.TIMERx_OVF is set to 1."]
    #[inline(always)]
    pub fn timer1_ovf(&self) -> TIMER_OVF_R {
        TIMER_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the masked interrupt status bit for the TIMER2_OVF interrupt when LEDC.INT_ENA.TIMERx_OVF is set to 1."]
    #[inline(always)]
    pub fn timer2_ovf(&self) -> TIMER_OVF_R {
        TIMER_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the masked interrupt status bit for the TIMER3_OVF interrupt when LEDC.INT_ENA.TIMERx_OVF is set to 1."]
    #[inline(always)]
    pub fn timer3_ovf(&self) -> TIMER_OVF_R {
        TIMER_OVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "This is the masked interrupt status bit for the DUTY_CHNG_END_CH(0-7) interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_CH0` field"]
    #[inline(always)]
    pub fn duty_chng_end_ch(&self, n: u8) -> DUTY_CHNG_END_CH_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DUTY_CHNG_END_CH_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "This is the masked interrupt status bit for the DUTY_CHNG_END_CH(0-7) interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch_iter(&self) -> impl Iterator<Item = DUTY_CHNG_END_CH_R> + '_ {
        (0..8).map(move |n| DUTY_CHNG_END_CH_R::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - This is the masked interrupt status bit for the DUTY_CHNG_END_CH0 interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch0(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the masked interrupt status bit for the DUTY_CHNG_END_CH1 interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch1(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the masked interrupt status bit for the DUTY_CHNG_END_CH2 interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch2(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the masked interrupt status bit for the DUTY_CHNG_END_CH3 interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch3(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the masked interrupt status bit for the DUTY_CHNG_END_CH4 interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch4(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the masked interrupt status bit for the DUTY_CHNG_END_CH5 interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch5(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the masked interrupt status bit for the DUTY_CHNG_END_CH6 interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch6(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the masked interrupt status bit for the DUTY_CHNG_END_CH7 interrupt when LEDC.INT_ENA.DUTY_CHNG_END_CHx is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch7(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH(0-7) interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OVF_CNT_CH0` field"]
    #[inline(always)]
    pub fn ovf_cnt_ch(&self, n: u8) -> OVF_CNT_CH_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        OVF_CNT_CH_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH(0-7) interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch_iter(&self) -> impl Iterator<Item = OVF_CNT_CH_R> + '_ {
        (0..8).map(move |n| OVF_CNT_CH_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH0 interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch0(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH1 interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch1(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH2 interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch2(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH3 interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch3(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH4 interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch4(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH5 interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch5(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH6 interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch6(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This is the masked interrupt status bit for the LEDC.INT_RAW.OVF_CNT_CH7 interrupt when LEDC.INT_ENA.OVF_CNT_CHx is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch7(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("timer0_ovf", &format_args!("{}", self.timer0_ovf().bit()))
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
            .field(
                "duty_chng_end_ch6",
                &format_args!("{}", self.duty_chng_end_ch6().bit()),
            )
            .field(
                "duty_chng_end_ch7",
                &format_args!("{}", self.duty_chng_end_ch7().bit()),
            )
            .field("ovf_cnt_ch0", &format_args!("{}", self.ovf_cnt_ch0().bit()))
            .field("ovf_cnt_ch1", &format_args!("{}", self.ovf_cnt_ch1().bit()))
            .field("ovf_cnt_ch2", &format_args!("{}", self.ovf_cnt_ch2().bit()))
            .field("ovf_cnt_ch3", &format_args!("{}", self.ovf_cnt_ch3().bit()))
            .field("ovf_cnt_ch4", &format_args!("{}", self.ovf_cnt_ch4().bit()))
            .field("ovf_cnt_ch5", &format_args!("{}", self.ovf_cnt_ch5().bit()))
            .field("ovf_cnt_ch6", &format_args!("{}", self.ovf_cnt_ch6().bit()))
            .field("ovf_cnt_ch7", &format_args!("{}", self.ovf_cnt_ch7().bit()))
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
