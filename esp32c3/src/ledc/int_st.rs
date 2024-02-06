#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `LSTIMER0_OVF_INT_ST` reader - reg_lstimer0_ovf_int_st."]
pub type LSTIMER0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `LSTIMER1_OVF_INT_ST` reader - reg_lstimer1_ovf_int_st."]
pub type LSTIMER1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `LSTIMER2_OVF_INT_ST` reader - reg_lstimer2_ovf_int_st."]
pub type LSTIMER2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `LSTIMER3_OVF_INT_ST` reader - reg_lstimer3_ovf_int_st."]
pub type LSTIMER3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ST` reader - reg_duty_chng_end_lsch0_int_st."]
pub type DUTY_CHNG_END_LSCH0_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ST` reader - reg_duty_chng_end_lsch1_int_st."]
pub type DUTY_CHNG_END_LSCH1_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ST` reader - reg_duty_chng_end_lsch2_int_st."]
pub type DUTY_CHNG_END_LSCH2_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ST` reader - reg_duty_chng_end_lsch3_int_st."]
pub type DUTY_CHNG_END_LSCH3_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ST` reader - reg_duty_chng_end_lsch4_int_st."]
pub type DUTY_CHNG_END_LSCH4_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ST` reader - reg_duty_chng_end_lsch5_int_st."]
pub type DUTY_CHNG_END_LSCH5_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_LSCH0_INT_ST` reader - reg_ovf_cnt_lsch0_int_st."]
pub type OVF_CNT_LSCH0_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_LSCH1_INT_ST` reader - reg_ovf_cnt_lsch1_int_st."]
pub type OVF_CNT_LSCH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_LSCH2_INT_ST` reader - reg_ovf_cnt_lsch2_int_st."]
pub type OVF_CNT_LSCH2_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_LSCH3_INT_ST` reader - reg_ovf_cnt_lsch3_int_st."]
pub type OVF_CNT_LSCH3_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_LSCH4_INT_ST` reader - reg_ovf_cnt_lsch4_int_st."]
pub type OVF_CNT_LSCH4_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_LSCH5_INT_ST` reader - reg_ovf_cnt_lsch5_int_st."]
pub type OVF_CNT_LSCH5_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reg_lstimer0_ovf_int_st."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_st(&self) -> LSTIMER0_OVF_INT_ST_R {
        LSTIMER0_OVF_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_lstimer1_ovf_int_st."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_st(&self) -> LSTIMER1_OVF_INT_ST_R {
        LSTIMER1_OVF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_lstimer2_ovf_int_st."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_st(&self) -> LSTIMER2_OVF_INT_ST_R {
        LSTIMER2_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_lstimer3_ovf_int_st."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_st(&self) -> LSTIMER3_OVF_INT_ST_R {
        LSTIMER3_OVF_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_duty_chng_end_lsch0_int_st."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_st(&self) -> DUTY_CHNG_END_LSCH0_INT_ST_R {
        DUTY_CHNG_END_LSCH0_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_duty_chng_end_lsch1_int_st."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_st(&self) -> DUTY_CHNG_END_LSCH1_INT_ST_R {
        DUTY_CHNG_END_LSCH1_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_duty_chng_end_lsch2_int_st."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_st(&self) -> DUTY_CHNG_END_LSCH2_INT_ST_R {
        DUTY_CHNG_END_LSCH2_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_duty_chng_end_lsch3_int_st."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_st(&self) -> DUTY_CHNG_END_LSCH3_INT_ST_R {
        DUTY_CHNG_END_LSCH3_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_duty_chng_end_lsch4_int_st."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_st(&self) -> DUTY_CHNG_END_LSCH4_INT_ST_R {
        DUTY_CHNG_END_LSCH4_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_duty_chng_end_lsch5_int_st."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_st(&self) -> DUTY_CHNG_END_LSCH5_INT_ST_R {
        DUTY_CHNG_END_LSCH5_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_ovf_cnt_lsch0_int_st."]
    #[inline(always)]
    pub fn ovf_cnt_lsch0_int_st(&self) -> OVF_CNT_LSCH0_INT_ST_R {
        OVF_CNT_LSCH0_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_ovf_cnt_lsch1_int_st."]
    #[inline(always)]
    pub fn ovf_cnt_lsch1_int_st(&self) -> OVF_CNT_LSCH1_INT_ST_R {
        OVF_CNT_LSCH1_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_ovf_cnt_lsch2_int_st."]
    #[inline(always)]
    pub fn ovf_cnt_lsch2_int_st(&self) -> OVF_CNT_LSCH2_INT_ST_R {
        OVF_CNT_LSCH2_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_ovf_cnt_lsch3_int_st."]
    #[inline(always)]
    pub fn ovf_cnt_lsch3_int_st(&self) -> OVF_CNT_LSCH3_INT_ST_R {
        OVF_CNT_LSCH3_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_ovf_cnt_lsch4_int_st."]
    #[inline(always)]
    pub fn ovf_cnt_lsch4_int_st(&self) -> OVF_CNT_LSCH4_INT_ST_R {
        OVF_CNT_LSCH4_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_lsch5_int_st."]
    #[inline(always)]
    pub fn ovf_cnt_lsch5_int_st(&self) -> OVF_CNT_LSCH5_INT_ST_R {
        OVF_CNT_LSCH5_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
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
                "ovf_cnt_lsch0_int_st",
                &format_args!("{}", self.ovf_cnt_lsch0_int_st().bit()),
            )
            .field(
                "ovf_cnt_lsch1_int_st",
                &format_args!("{}", self.ovf_cnt_lsch1_int_st().bit()),
            )
            .field(
                "ovf_cnt_lsch2_int_st",
                &format_args!("{}", self.ovf_cnt_lsch2_int_st().bit()),
            )
            .field(
                "ovf_cnt_lsch3_int_st",
                &format_args!("{}", self.ovf_cnt_lsch3_int_st().bit()),
            )
            .field(
                "ovf_cnt_lsch4_int_st",
                &format_args!("{}", self.ovf_cnt_lsch4_int_st().bit()),
            )
            .field(
                "ovf_cnt_lsch5_int_st",
                &format_args!("{}", self.ovf_cnt_lsch5_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "LEDC_INT_ST.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
