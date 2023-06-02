#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_OVF_INT_RAW` reader - Triggered when the timer0 has reached its maximum counter value."]
pub type TIMER0_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER0_OVF_INT_RAW` writer - Triggered when the timer0 has reached its maximum counter value."]
pub type TIMER0_OVF_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER1_OVF_INT_RAW` reader - Triggered when the timer1 has reached its maximum counter value."]
pub type TIMER1_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER1_OVF_INT_RAW` writer - Triggered when the timer1 has reached its maximum counter value."]
pub type TIMER1_OVF_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER2_OVF_INT_RAW` reader - Triggered when the timer2 has reached its maximum counter value."]
pub type TIMER2_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER2_OVF_INT_RAW` writer - Triggered when the timer2 has reached its maximum counter value."]
pub type TIMER2_OVF_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER3_OVF_INT_RAW` reader - Triggered when the timer3 has reached its maximum counter value."]
pub type TIMER3_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER3_OVF_INT_RAW` writer - Triggered when the timer3 has reached its maximum counter value."]
pub type TIMER3_OVF_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_RAW` reader - Interrupt raw bit for channel 0. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_RAW` writer - Interrupt raw bit for channel 0. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH0_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_RAW` reader - Interrupt raw bit for channel 1. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH1_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_RAW` writer - Interrupt raw bit for channel 1. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH1_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_RAW` reader - Interrupt raw bit for channel 2. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH2_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_RAW` writer - Interrupt raw bit for channel 2. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH2_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_RAW` reader - Interrupt raw bit for channel 3. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_RAW` writer - Interrupt raw bit for channel 3. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH3_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_RAW` reader - Interrupt raw bit for channel 4. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_RAW` writer - Interrupt raw bit for channel 4. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH4_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_RAW` reader - Interrupt raw bit for channel 5. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH5_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_RAW` writer - Interrupt raw bit for channel 5. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH5_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_CH6_INT_RAW` reader - Interrupt raw bit for channel 6. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH6_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH6_INT_RAW` writer - Interrupt raw bit for channel 6. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH6_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_CH7_INT_RAW` reader - Interrupt raw bit for channel 7. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH7_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH7_INT_RAW` writer - Interrupt raw bit for channel 7. Triggered when the gradual change of duty has finished."]
pub type DUTY_CHNG_END_CH7_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OVF_CNT_CH0_INT_RAW` reader - Interrupt raw bit for channel 0. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
pub type OVF_CNT_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH0_INT_RAW` writer - Interrupt raw bit for channel 0. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
pub type OVF_CNT_CH0_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OVF_CNT_CH1_INT_RAW` reader - Interrupt raw bit for channel 1. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
pub type OVF_CNT_CH1_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH1_INT_RAW` writer - Interrupt raw bit for channel 1. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
pub type OVF_CNT_CH1_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OVF_CNT_CH2_INT_RAW` reader - Interrupt raw bit for channel 2. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
pub type OVF_CNT_CH2_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH2_INT_RAW` writer - Interrupt raw bit for channel 2. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
pub type OVF_CNT_CH2_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OVF_CNT_CH3_INT_RAW` reader - Interrupt raw bit for channel 3. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
pub type OVF_CNT_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH3_INT_RAW` writer - Interrupt raw bit for channel 3. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
pub type OVF_CNT_CH3_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OVF_CNT_CH4_INT_RAW` reader - Interrupt raw bit for channel 4. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
pub type OVF_CNT_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH4_INT_RAW` writer - Interrupt raw bit for channel 4. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
pub type OVF_CNT_CH4_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OVF_CNT_CH5_INT_RAW` reader - Interrupt raw bit for channel 5. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
pub type OVF_CNT_CH5_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH5_INT_RAW` writer - Interrupt raw bit for channel 5. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
pub type OVF_CNT_CH5_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OVF_CNT_CH6_INT_RAW` reader - Interrupt raw bit for channel 6. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6."]
pub type OVF_CNT_CH6_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH6_INT_RAW` writer - Interrupt raw bit for channel 6. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6."]
pub type OVF_CNT_CH6_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OVF_CNT_CH7_INT_RAW` reader - Interrupt raw bit for channel 7. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7."]
pub type OVF_CNT_CH7_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH7_INT_RAW` writer - Interrupt raw bit for channel 7. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7."]
pub type OVF_CNT_CH7_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Triggered when the timer0 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer0_ovf_int_raw(&self) -> TIMER0_OVF_INT_RAW_R {
        TIMER0_OVF_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Triggered when the timer1 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer1_ovf_int_raw(&self) -> TIMER1_OVF_INT_RAW_R {
        TIMER1_OVF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Triggered when the timer2 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer2_ovf_int_raw(&self) -> TIMER2_OVF_INT_RAW_R {
        TIMER2_OVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Triggered when the timer3 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer3_ovf_int_raw(&self) -> TIMER3_OVF_INT_RAW_R {
        TIMER3_OVF_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt raw bit for channel 0. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_raw(&self) -> DUTY_CHNG_END_CH0_INT_RAW_R {
        DUTY_CHNG_END_CH0_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt raw bit for channel 1. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_raw(&self) -> DUTY_CHNG_END_CH1_INT_RAW_R {
        DUTY_CHNG_END_CH1_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt raw bit for channel 2. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_raw(&self) -> DUTY_CHNG_END_CH2_INT_RAW_R {
        DUTY_CHNG_END_CH2_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt raw bit for channel 3. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_raw(&self) -> DUTY_CHNG_END_CH3_INT_RAW_R {
        DUTY_CHNG_END_CH3_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt raw bit for channel 4. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_raw(&self) -> DUTY_CHNG_END_CH4_INT_RAW_R {
        DUTY_CHNG_END_CH4_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt raw bit for channel 5. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_raw(&self) -> DUTY_CHNG_END_CH5_INT_RAW_R {
        DUTY_CHNG_END_CH5_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt raw bit for channel 6. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch6_int_raw(&self) -> DUTY_CHNG_END_CH6_INT_RAW_R {
        DUTY_CHNG_END_CH6_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt raw bit for channel 7. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch7_int_raw(&self) -> DUTY_CHNG_END_CH7_INT_RAW_R {
        DUTY_CHNG_END_CH7_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt raw bit for channel 0. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_raw(&self) -> OVF_CNT_CH0_INT_RAW_R {
        OVF_CNT_CH0_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt raw bit for channel 1. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_raw(&self) -> OVF_CNT_CH1_INT_RAW_R {
        OVF_CNT_CH1_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt raw bit for channel 2. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_raw(&self) -> OVF_CNT_CH2_INT_RAW_R {
        OVF_CNT_CH2_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt raw bit for channel 3. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_raw(&self) -> OVF_CNT_CH3_INT_RAW_R {
        OVF_CNT_CH3_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt raw bit for channel 4. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_raw(&self) -> OVF_CNT_CH4_INT_RAW_R {
        OVF_CNT_CH4_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt raw bit for channel 5. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_raw(&self) -> OVF_CNT_CH5_INT_RAW_R {
        OVF_CNT_CH5_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt raw bit for channel 6. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6."]
    #[inline(always)]
    pub fn ovf_cnt_ch6_int_raw(&self) -> OVF_CNT_CH6_INT_RAW_R {
        OVF_CNT_CH6_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt raw bit for channel 7. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7."]
    #[inline(always)]
    pub fn ovf_cnt_ch7_int_raw(&self) -> OVF_CNT_CH7_INT_RAW_R {
        OVF_CNT_CH7_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "timer0_ovf_int_raw",
                &format_args!("{}", self.timer0_ovf_int_raw().bit()),
            )
            .field(
                "timer1_ovf_int_raw",
                &format_args!("{}", self.timer1_ovf_int_raw().bit()),
            )
            .field(
                "timer2_ovf_int_raw",
                &format_args!("{}", self.timer2_ovf_int_raw().bit()),
            )
            .field(
                "timer3_ovf_int_raw",
                &format_args!("{}", self.timer3_ovf_int_raw().bit()),
            )
            .field(
                "duty_chng_end_ch0_int_raw",
                &format_args!("{}", self.duty_chng_end_ch0_int_raw().bit()),
            )
            .field(
                "duty_chng_end_ch1_int_raw",
                &format_args!("{}", self.duty_chng_end_ch1_int_raw().bit()),
            )
            .field(
                "duty_chng_end_ch2_int_raw",
                &format_args!("{}", self.duty_chng_end_ch2_int_raw().bit()),
            )
            .field(
                "duty_chng_end_ch3_int_raw",
                &format_args!("{}", self.duty_chng_end_ch3_int_raw().bit()),
            )
            .field(
                "duty_chng_end_ch4_int_raw",
                &format_args!("{}", self.duty_chng_end_ch4_int_raw().bit()),
            )
            .field(
                "duty_chng_end_ch5_int_raw",
                &format_args!("{}", self.duty_chng_end_ch5_int_raw().bit()),
            )
            .field(
                "duty_chng_end_ch6_int_raw",
                &format_args!("{}", self.duty_chng_end_ch6_int_raw().bit()),
            )
            .field(
                "duty_chng_end_ch7_int_raw",
                &format_args!("{}", self.duty_chng_end_ch7_int_raw().bit()),
            )
            .field(
                "ovf_cnt_ch0_int_raw",
                &format_args!("{}", self.ovf_cnt_ch0_int_raw().bit()),
            )
            .field(
                "ovf_cnt_ch1_int_raw",
                &format_args!("{}", self.ovf_cnt_ch1_int_raw().bit()),
            )
            .field(
                "ovf_cnt_ch2_int_raw",
                &format_args!("{}", self.ovf_cnt_ch2_int_raw().bit()),
            )
            .field(
                "ovf_cnt_ch3_int_raw",
                &format_args!("{}", self.ovf_cnt_ch3_int_raw().bit()),
            )
            .field(
                "ovf_cnt_ch4_int_raw",
                &format_args!("{}", self.ovf_cnt_ch4_int_raw().bit()),
            )
            .field(
                "ovf_cnt_ch5_int_raw",
                &format_args!("{}", self.ovf_cnt_ch5_int_raw().bit()),
            )
            .field(
                "ovf_cnt_ch6_int_raw",
                &format_args!("{}", self.ovf_cnt_ch6_int_raw().bit()),
            )
            .field(
                "ovf_cnt_ch7_int_raw",
                &format_args!("{}", self.ovf_cnt_ch7_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Triggered when the timer0 has reached its maximum counter value."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ovf_int_raw(&mut self) -> TIMER0_OVF_INT_RAW_W<0> {
        TIMER0_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - Triggered when the timer1 has reached its maximum counter value."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ovf_int_raw(&mut self) -> TIMER1_OVF_INT_RAW_W<1> {
        TIMER1_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - Triggered when the timer2 has reached its maximum counter value."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_ovf_int_raw(&mut self) -> TIMER2_OVF_INT_RAW_W<2> {
        TIMER2_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - Triggered when the timer3 has reached its maximum counter value."]
    #[inline(always)]
    #[must_use]
    pub fn timer3_ovf_int_raw(&mut self) -> TIMER3_OVF_INT_RAW_W<3> {
        TIMER3_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt raw bit for channel 0. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch0_int_raw(&mut self) -> DUTY_CHNG_END_CH0_INT_RAW_W<4> {
        DUTY_CHNG_END_CH0_INT_RAW_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt raw bit for channel 1. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch1_int_raw(&mut self) -> DUTY_CHNG_END_CH1_INT_RAW_W<5> {
        DUTY_CHNG_END_CH1_INT_RAW_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt raw bit for channel 2. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch2_int_raw(&mut self) -> DUTY_CHNG_END_CH2_INT_RAW_W<6> {
        DUTY_CHNG_END_CH2_INT_RAW_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt raw bit for channel 3. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch3_int_raw(&mut self) -> DUTY_CHNG_END_CH3_INT_RAW_W<7> {
        DUTY_CHNG_END_CH3_INT_RAW_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt raw bit for channel 4. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch4_int_raw(&mut self) -> DUTY_CHNG_END_CH4_INT_RAW_W<8> {
        DUTY_CHNG_END_CH4_INT_RAW_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt raw bit for channel 5. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch5_int_raw(&mut self) -> DUTY_CHNG_END_CH5_INT_RAW_W<9> {
        DUTY_CHNG_END_CH5_INT_RAW_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt raw bit for channel 6. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch6_int_raw(&mut self) -> DUTY_CHNG_END_CH6_INT_RAW_W<10> {
        DUTY_CHNG_END_CH6_INT_RAW_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt raw bit for channel 7. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch7_int_raw(&mut self) -> DUTY_CHNG_END_CH7_INT_RAW_W<11> {
        DUTY_CHNG_END_CH7_INT_RAW_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt raw bit for channel 0. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch0_int_raw(&mut self) -> OVF_CNT_CH0_INT_RAW_W<12> {
        OVF_CNT_CH0_INT_RAW_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt raw bit for channel 1. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch1_int_raw(&mut self) -> OVF_CNT_CH1_INT_RAW_W<13> {
        OVF_CNT_CH1_INT_RAW_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt raw bit for channel 2. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch2_int_raw(&mut self) -> OVF_CNT_CH2_INT_RAW_W<14> {
        OVF_CNT_CH2_INT_RAW_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt raw bit for channel 3. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch3_int_raw(&mut self) -> OVF_CNT_CH3_INT_RAW_W<15> {
        OVF_CNT_CH3_INT_RAW_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt raw bit for channel 4. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch4_int_raw(&mut self) -> OVF_CNT_CH4_INT_RAW_W<16> {
        OVF_CNT_CH4_INT_RAW_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt raw bit for channel 5. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch5_int_raw(&mut self) -> OVF_CNT_CH5_INT_RAW_W<17> {
        OVF_CNT_CH5_INT_RAW_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt raw bit for channel 6. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch6_int_raw(&mut self) -> OVF_CNT_CH6_INT_RAW_W<18> {
        OVF_CNT_CH6_INT_RAW_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt raw bit for channel 7. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch7_int_raw(&mut self) -> OVF_CNT_CH7_INT_RAW_W<19> {
        OVF_CNT_CH7_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
