#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `TIMER0_OVF` reader - Triggered when the timer0 has reached its maximum counter value."]
pub type TIMER0_OVF_R = crate::BitReader;
#[doc = "Field `TIMER1_OVF` reader - Triggered when the timer1 has reached its maximum counter value."]
pub type TIMER1_OVF_R = crate::BitReader;
#[doc = "Field `TIMER2_OVF` reader - Triggered when the timer2 has reached its maximum counter value."]
pub type TIMER2_OVF_R = crate::BitReader;
#[doc = "Field `TIMER3_OVF` reader - Triggered when the timer3 has reached its maximum counter value."]
pub type TIMER3_OVF_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH0` reader - Interrupt raw bit for channel 0. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH0_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH1` reader - Interrupt raw bit for channel 1. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH1_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH2` reader - Interrupt raw bit for channel 2. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH2_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH3` reader - Interrupt raw bit for channel 3. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH3_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH4` reader - Interrupt raw bit for channel 4. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH4_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH5` reader - Interrupt raw bit for channel 5. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH5_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH6` reader - Interrupt raw bit for channel 6. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH6_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH7` reader - Interrupt raw bit for channel 7. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH7_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH0` reader - Interrupt raw bit for channel 0. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
pub type OVF_CNT_CH0_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH1` reader - Interrupt raw bit for channel 1. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
pub type OVF_CNT_CH1_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH2` reader - Interrupt raw bit for channel 2. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
pub type OVF_CNT_CH2_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH3` reader - Interrupt raw bit for channel 3. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
pub type OVF_CNT_CH3_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH4` reader - Interrupt raw bit for channel 4. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
pub type OVF_CNT_CH4_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH5` reader - Interrupt raw bit for channel 5. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
pub type OVF_CNT_CH5_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH6` reader - Interrupt raw bit for channel 6. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6."]
pub type OVF_CNT_CH6_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH7` reader - Interrupt raw bit for channel 7. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7."]
pub type OVF_CNT_CH7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Triggered when the timer0 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer0_ovf(&self) -> TIMER0_OVF_R {
        TIMER0_OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Triggered when the timer1 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer1_ovf(&self) -> TIMER1_OVF_R {
        TIMER1_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Triggered when the timer2 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer2_ovf(&self) -> TIMER2_OVF_R {
        TIMER2_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Triggered when the timer3 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer3_ovf(&self) -> TIMER3_OVF_R {
        TIMER3_OVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt raw bit for channel 0. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch0(&self) -> DUTY_CHNG_END_CH0_R {
        DUTY_CHNG_END_CH0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt raw bit for channel 1. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch1(&self) -> DUTY_CHNG_END_CH1_R {
        DUTY_CHNG_END_CH1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt raw bit for channel 2. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch2(&self) -> DUTY_CHNG_END_CH2_R {
        DUTY_CHNG_END_CH2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt raw bit for channel 3. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch3(&self) -> DUTY_CHNG_END_CH3_R {
        DUTY_CHNG_END_CH3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt raw bit for channel 4. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch4(&self) -> DUTY_CHNG_END_CH4_R {
        DUTY_CHNG_END_CH4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt raw bit for channel 5. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch5(&self) -> DUTY_CHNG_END_CH5_R {
        DUTY_CHNG_END_CH5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt raw bit for channel 6. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch6(&self) -> DUTY_CHNG_END_CH6_R {
        DUTY_CHNG_END_CH6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt raw bit for channel 7. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch7(&self) -> DUTY_CHNG_END_CH7_R {
        DUTY_CHNG_END_CH7_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt raw bit for channel 0. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
    #[inline(always)]
    pub fn ovf_cnt_ch0(&self) -> OVF_CNT_CH0_R {
        OVF_CNT_CH0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt raw bit for channel 1. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
    #[inline(always)]
    pub fn ovf_cnt_ch1(&self) -> OVF_CNT_CH1_R {
        OVF_CNT_CH1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt raw bit for channel 2. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
    #[inline(always)]
    pub fn ovf_cnt_ch2(&self) -> OVF_CNT_CH2_R {
        OVF_CNT_CH2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt raw bit for channel 3. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
    #[inline(always)]
    pub fn ovf_cnt_ch3(&self) -> OVF_CNT_CH3_R {
        OVF_CNT_CH3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt raw bit for channel 4. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
    #[inline(always)]
    pub fn ovf_cnt_ch4(&self) -> OVF_CNT_CH4_R {
        OVF_CNT_CH4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt raw bit for channel 5. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
    #[inline(always)]
    pub fn ovf_cnt_ch5(&self) -> OVF_CNT_CH5_R {
        OVF_CNT_CH5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt raw bit for channel 6. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6."]
    #[inline(always)]
    pub fn ovf_cnt_ch6(&self) -> OVF_CNT_CH6_R {
        OVF_CNT_CH6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt raw bit for channel 7. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7."]
    #[inline(always)]
    pub fn ovf_cnt_ch7(&self) -> OVF_CNT_CH7_R {
        OVF_CNT_CH7_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
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
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
