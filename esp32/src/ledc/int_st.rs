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
#[doc = "Field `HSTIMER0_OVF_INT_ST` reader - The interrupt status bit for high speed channel0 counter overflow event."]
pub type HSTIMER0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `HSTIMER1_OVF_INT_ST` reader - The interrupt status bit for high speed channel1 counter overflow event."]
pub type HSTIMER1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `HSTIMER2_OVF_INT_ST` reader - The interrupt status bit for high speed channel2 counter overflow event."]
pub type HSTIMER2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `HSTIMER3_OVF_INT_ST` reader - The interrupt status bit for high speed channel3 counter overflow event."]
pub type HSTIMER3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `LSTIMER0_OVF_INT_ST` reader - The interrupt status bit for low speed channel0 counter overflow event."]
pub type LSTIMER0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `LSTIMER1_OVF_INT_ST` reader - The interrupt status bit for low speed channel1 counter overflow event."]
pub type LSTIMER1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `LSTIMER2_OVF_INT_ST` reader - The interrupt status bit for low speed channel2 counter overflow event."]
pub type LSTIMER2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `LSTIMER3_OVF_INT_ST` reader - The interrupt status bit for low speed channel3 counter overflow event."]
pub type LSTIMER3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_ST` reader - The interrupt status bit for high speed channel 0 duty change done event."]
pub type DUTY_CHNG_END_HSCH0_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_ST` reader - The interrupt status bit for high speed channel 1 duty change done event."]
pub type DUTY_CHNG_END_HSCH1_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_ST` reader - The interrupt status bit for high speed channel 2 duty change done event."]
pub type DUTY_CHNG_END_HSCH2_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_ST` reader - The interrupt status bit for high speed channel 3 duty change done event."]
pub type DUTY_CHNG_END_HSCH3_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_ST` reader - The interrupt status bit for high speed channel 4 duty change done event."]
pub type DUTY_CHNG_END_HSCH4_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_ST` reader - The interrupt status bit for high speed channel 5 duty change done event."]
pub type DUTY_CHNG_END_HSCH5_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_ST` reader - The interrupt status bit for high speed channel 6 duty change done event."]
pub type DUTY_CHNG_END_HSCH6_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_ST` reader - The interrupt status bit for high speed channel 7 duty change done event."]
pub type DUTY_CHNG_END_HSCH7_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ST` reader - The interrupt status bit for low speed channel 0 duty change done event."]
pub type DUTY_CHNG_END_LSCH0_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ST` reader - The interrupt status bit for low speed channel 1 duty change done event."]
pub type DUTY_CHNG_END_LSCH1_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ST` reader - The interrupt status bit for low speed channel 2 duty change done event."]
pub type DUTY_CHNG_END_LSCH2_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ST` reader - The interrupt status bit for low speed channel 3 duty change done event."]
pub type DUTY_CHNG_END_LSCH3_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ST` reader - The interrupt status bit for low speed channel 4 duty change done event."]
pub type DUTY_CHNG_END_LSCH4_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ST` reader - The interrupt status bit for low speed channel 5 duty change done event."]
pub type DUTY_CHNG_END_LSCH5_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_ST` reader - The interrupt status bit for low speed channel 6 duty change done event."]
pub type DUTY_CHNG_END_LSCH6_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_ST` reader - The interrupt status bit for low speed channel 7 duty change done event"]
pub type DUTY_CHNG_END_LSCH7_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The interrupt status bit for high speed channel0 counter overflow event."]
    #[inline(always)]
    pub fn hstimer0_ovf_int_st(&self) -> HSTIMER0_OVF_INT_ST_R {
        HSTIMER0_OVF_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt status bit for high speed channel1 counter overflow event."]
    #[inline(always)]
    pub fn hstimer1_ovf_int_st(&self) -> HSTIMER1_OVF_INT_ST_R {
        HSTIMER1_OVF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt status bit for high speed channel2 counter overflow event."]
    #[inline(always)]
    pub fn hstimer2_ovf_int_st(&self) -> HSTIMER2_OVF_INT_ST_R {
        HSTIMER2_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt status bit for high speed channel3 counter overflow event."]
    #[inline(always)]
    pub fn hstimer3_ovf_int_st(&self) -> HSTIMER3_OVF_INT_ST_R {
        HSTIMER3_OVF_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt status bit for low speed channel0 counter overflow event."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_st(&self) -> LSTIMER0_OVF_INT_ST_R {
        LSTIMER0_OVF_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt status bit for low speed channel1 counter overflow event."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_st(&self) -> LSTIMER1_OVF_INT_ST_R {
        LSTIMER1_OVF_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt status bit for low speed channel2 counter overflow event."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_st(&self) -> LSTIMER2_OVF_INT_ST_R {
        LSTIMER2_OVF_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt status bit for low speed channel3 counter overflow event."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_st(&self) -> LSTIMER3_OVF_INT_ST_R {
        LSTIMER3_OVF_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt status bit for high speed channel 0 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0_int_st(&self) -> DUTY_CHNG_END_HSCH0_INT_ST_R {
        DUTY_CHNG_END_HSCH0_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt status bit for high speed channel 1 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1_int_st(&self) -> DUTY_CHNG_END_HSCH1_INT_ST_R {
        DUTY_CHNG_END_HSCH1_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt status bit for high speed channel 2 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2_int_st(&self) -> DUTY_CHNG_END_HSCH2_INT_ST_R {
        DUTY_CHNG_END_HSCH2_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt status bit for high speed channel 3 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3_int_st(&self) -> DUTY_CHNG_END_HSCH3_INT_ST_R {
        DUTY_CHNG_END_HSCH3_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt status bit for high speed channel 4 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4_int_st(&self) -> DUTY_CHNG_END_HSCH4_INT_ST_R {
        DUTY_CHNG_END_HSCH4_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt status bit for high speed channel 5 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5_int_st(&self) -> DUTY_CHNG_END_HSCH5_INT_ST_R {
        DUTY_CHNG_END_HSCH5_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt status bit for high speed channel 6 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6_int_st(&self) -> DUTY_CHNG_END_HSCH6_INT_ST_R {
        DUTY_CHNG_END_HSCH6_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt status bit for high speed channel 7 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7_int_st(&self) -> DUTY_CHNG_END_HSCH7_INT_ST_R {
        DUTY_CHNG_END_HSCH7_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt status bit for low speed channel 0 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_st(&self) -> DUTY_CHNG_END_LSCH0_INT_ST_R {
        DUTY_CHNG_END_LSCH0_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt status bit for low speed channel 1 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_st(&self) -> DUTY_CHNG_END_LSCH1_INT_ST_R {
        DUTY_CHNG_END_LSCH1_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt status bit for low speed channel 2 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_st(&self) -> DUTY_CHNG_END_LSCH2_INT_ST_R {
        DUTY_CHNG_END_LSCH2_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt status bit for low speed channel 3 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_st(&self) -> DUTY_CHNG_END_LSCH3_INT_ST_R {
        DUTY_CHNG_END_LSCH3_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt status bit for low speed channel 4 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_st(&self) -> DUTY_CHNG_END_LSCH4_INT_ST_R {
        DUTY_CHNG_END_LSCH4_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt status bit for low speed channel 5 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_st(&self) -> DUTY_CHNG_END_LSCH5_INT_ST_R {
        DUTY_CHNG_END_LSCH5_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt status bit for low speed channel 6 duty change done event."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6_int_st(&self) -> DUTY_CHNG_END_LSCH6_INT_ST_R {
        DUTY_CHNG_END_LSCH6_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt status bit for low speed channel 7 duty change done event"]
    #[inline(always)]
    pub fn duty_chng_end_lsch7_int_st(&self) -> DUTY_CHNG_END_LSCH7_INT_ST_R {
        DUTY_CHNG_END_LSCH7_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "hstimer0_ovf_int_st",
                &format_args!("{}", self.hstimer0_ovf_int_st().bit()),
            )
            .field(
                "hstimer1_ovf_int_st",
                &format_args!("{}", self.hstimer1_ovf_int_st().bit()),
            )
            .field(
                "hstimer2_ovf_int_st",
                &format_args!("{}", self.hstimer2_ovf_int_st().bit()),
            )
            .field(
                "hstimer3_ovf_int_st",
                &format_args!("{}", self.hstimer3_ovf_int_st().bit()),
            )
            .field(
                "lstimer0_ovf_int_st",
                &format_args!("{}", self.lstimer0_ovf_int_st().bit()),
            )
            .field(
                "lstimer1_ovf_int_st",
                &format_args!("{}", self.lstimer1_ovf_int_st().bit()),
            )
            .field(
                "lstimer2_ovf_int_st",
                &format_args!("{}", self.lstimer2_ovf_int_st().bit()),
            )
            .field(
                "lstimer3_ovf_int_st",
                &format_args!("{}", self.lstimer3_ovf_int_st().bit()),
            )
            .field(
                "duty_chng_end_hsch0_int_st",
                &format_args!("{}", self.duty_chng_end_hsch0_int_st().bit()),
            )
            .field(
                "duty_chng_end_hsch1_int_st",
                &format_args!("{}", self.duty_chng_end_hsch1_int_st().bit()),
            )
            .field(
                "duty_chng_end_hsch2_int_st",
                &format_args!("{}", self.duty_chng_end_hsch2_int_st().bit()),
            )
            .field(
                "duty_chng_end_hsch3_int_st",
                &format_args!("{}", self.duty_chng_end_hsch3_int_st().bit()),
            )
            .field(
                "duty_chng_end_hsch4_int_st",
                &format_args!("{}", self.duty_chng_end_hsch4_int_st().bit()),
            )
            .field(
                "duty_chng_end_hsch5_int_st",
                &format_args!("{}", self.duty_chng_end_hsch5_int_st().bit()),
            )
            .field(
                "duty_chng_end_hsch6_int_st",
                &format_args!("{}", self.duty_chng_end_hsch6_int_st().bit()),
            )
            .field(
                "duty_chng_end_hsch7_int_st",
                &format_args!("{}", self.duty_chng_end_hsch7_int_st().bit()),
            )
            .field(
                "duty_chng_end_lsch0_int_st",
                &format_args!("{}", self.duty_chng_end_lsch0_int_st().bit()),
            )
            .field(
                "duty_chng_end_lsch1_int_st",
                &format_args!("{}", self.duty_chng_end_lsch1_int_st().bit()),
            )
            .field(
                "duty_chng_end_lsch2_int_st",
                &format_args!("{}", self.duty_chng_end_lsch2_int_st().bit()),
            )
            .field(
                "duty_chng_end_lsch3_int_st",
                &format_args!("{}", self.duty_chng_end_lsch3_int_st().bit()),
            )
            .field(
                "duty_chng_end_lsch4_int_st",
                &format_args!("{}", self.duty_chng_end_lsch4_int_st().bit()),
            )
            .field(
                "duty_chng_end_lsch5_int_st",
                &format_args!("{}", self.duty_chng_end_lsch5_int_st().bit()),
            )
            .field(
                "duty_chng_end_lsch6_int_st",
                &format_args!("{}", self.duty_chng_end_lsch6_int_st().bit()),
            )
            .field(
                "duty_chng_end_lsch7_int_st",
                &format_args!("{}", self.duty_chng_end_lsch7_int_st().bit()),
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
