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
#[doc = "Field `LSTIMER0_OVF_INT_RAW` reader - reg_lstimer0_ovf_int_raw."]
pub type LSTIMER0_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `LSTIMER1_OVF_INT_RAW` reader - reg_lstimer1_ovf_int_raw."]
pub type LSTIMER1_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `LSTIMER2_OVF_INT_RAW` reader - reg_lstimer2_ovf_int_raw."]
pub type LSTIMER2_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `LSTIMER3_OVF_INT_RAW` reader - reg_lstimer3_ovf_int_raw."]
pub type LSTIMER3_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_RAW` reader - reg_duty_chng_end_lsch0_int_raw."]
pub type DUTY_CHNG_END_LSCH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_RAW` reader - reg_duty_chng_end_lsch1_int_raw."]
pub type DUTY_CHNG_END_LSCH1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_RAW` reader - reg_duty_chng_end_lsch2_int_raw."]
pub type DUTY_CHNG_END_LSCH2_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_RAW` reader - reg_duty_chng_end_lsch3_int_raw."]
pub type DUTY_CHNG_END_LSCH3_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_RAW` reader - reg_duty_chng_end_lsch4_int_raw."]
pub type DUTY_CHNG_END_LSCH4_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_RAW` reader - reg_duty_chng_end_lsch5_int_raw."]
pub type DUTY_CHNG_END_LSCH5_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_LSCH0_INT_RAW` reader - reg_ovf_cnt_lsch0_int_raw."]
pub type OVF_CNT_LSCH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_LSCH1_INT_RAW` reader - reg_ovf_cnt_lsch1_int_raw."]
pub type OVF_CNT_LSCH1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_LSCH2_INT_RAW` reader - reg_ovf_cnt_lsch2_int_raw."]
pub type OVF_CNT_LSCH2_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_LSCH3_INT_RAW` reader - reg_ovf_cnt_lsch3_int_raw."]
pub type OVF_CNT_LSCH3_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_LSCH4_INT_RAW` reader - reg_ovf_cnt_lsch4_int_raw."]
pub type OVF_CNT_LSCH4_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_LSCH5_INT_RAW` reader - reg_ovf_cnt_lsch5_int_raw."]
pub type OVF_CNT_LSCH5_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - reg_lstimer0_ovf_int_raw."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_raw(&self) -> LSTIMER0_OVF_INT_RAW_R {
        LSTIMER0_OVF_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_lstimer1_ovf_int_raw."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_raw(&self) -> LSTIMER1_OVF_INT_RAW_R {
        LSTIMER1_OVF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_lstimer2_ovf_int_raw."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_raw(&self) -> LSTIMER2_OVF_INT_RAW_R {
        LSTIMER2_OVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_lstimer3_ovf_int_raw."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_raw(&self) -> LSTIMER3_OVF_INT_RAW_R {
        LSTIMER3_OVF_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_duty_chng_end_lsch0_int_raw."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_raw(&self) -> DUTY_CHNG_END_LSCH0_INT_RAW_R {
        DUTY_CHNG_END_LSCH0_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_duty_chng_end_lsch1_int_raw."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_raw(&self) -> DUTY_CHNG_END_LSCH1_INT_RAW_R {
        DUTY_CHNG_END_LSCH1_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_duty_chng_end_lsch2_int_raw."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_raw(&self) -> DUTY_CHNG_END_LSCH2_INT_RAW_R {
        DUTY_CHNG_END_LSCH2_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_duty_chng_end_lsch3_int_raw."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_raw(&self) -> DUTY_CHNG_END_LSCH3_INT_RAW_R {
        DUTY_CHNG_END_LSCH3_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_duty_chng_end_lsch4_int_raw."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_raw(&self) -> DUTY_CHNG_END_LSCH4_INT_RAW_R {
        DUTY_CHNG_END_LSCH4_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_duty_chng_end_lsch5_int_raw."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_raw(&self) -> DUTY_CHNG_END_LSCH5_INT_RAW_R {
        DUTY_CHNG_END_LSCH5_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_ovf_cnt_lsch0_int_raw."]
    #[inline(always)]
    pub fn ovf_cnt_lsch0_int_raw(&self) -> OVF_CNT_LSCH0_INT_RAW_R {
        OVF_CNT_LSCH0_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_ovf_cnt_lsch1_int_raw."]
    #[inline(always)]
    pub fn ovf_cnt_lsch1_int_raw(&self) -> OVF_CNT_LSCH1_INT_RAW_R {
        OVF_CNT_LSCH1_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_ovf_cnt_lsch2_int_raw."]
    #[inline(always)]
    pub fn ovf_cnt_lsch2_int_raw(&self) -> OVF_CNT_LSCH2_INT_RAW_R {
        OVF_CNT_LSCH2_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_ovf_cnt_lsch3_int_raw."]
    #[inline(always)]
    pub fn ovf_cnt_lsch3_int_raw(&self) -> OVF_CNT_LSCH3_INT_RAW_R {
        OVF_CNT_LSCH3_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_ovf_cnt_lsch4_int_raw."]
    #[inline(always)]
    pub fn ovf_cnt_lsch4_int_raw(&self) -> OVF_CNT_LSCH4_INT_RAW_R {
        OVF_CNT_LSCH4_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_lsch5_int_raw."]
    #[inline(always)]
    pub fn ovf_cnt_lsch5_int_raw(&self) -> OVF_CNT_LSCH5_INT_RAW_R {
        OVF_CNT_LSCH5_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "LEDC_INT_RAW.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
