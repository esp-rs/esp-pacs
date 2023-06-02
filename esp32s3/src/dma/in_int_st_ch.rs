#[doc = "Register `IN_INT_ST_CH%s` reader"]
pub struct R(crate::R<IN_INT_ST_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_INT_ST_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_INT_ST_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_INT_ST_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_DONE` reader - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` reader - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `INFIFO_FULL_WM` reader - The raw interrupt status bit for the INFIFO_FULL_WM_CH_INT interrupt."]
pub type INFIFO_FULL_WM_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1` reader - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L3` reader - The raw interrupt status bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub type INFIFO_OVF_L3_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L3` reader - The raw interrupt status bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub type INFIFO_UDF_L3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the INFIFO_FULL_WM_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_full_wm(&self) -> INFIFO_FULL_WM_R {
        INFIFO_FULL_WM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> INFIFO_OVF_L1_R {
        INFIFO_OVF_L1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> INFIFO_UDF_L1_R {
        INFIFO_UDF_L1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l3(&self) -> INFIFO_OVF_L3_R {
        INFIFO_OVF_L3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l3(&self) -> INFIFO_UDF_L3_R {
        INFIFO_UDF_L3_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_INT_ST_CH")
            .field("in_done", &format_args!("{}", self.in_done().bit()))
            .field("in_suc_eof", &format_args!("{}", self.in_suc_eof().bit()))
            .field("in_err_eof", &format_args!("{}", self.in_err_eof().bit()))
            .field("in_dscr_err", &format_args!("{}", self.in_dscr_err().bit()))
            .field(
                "in_dscr_empty",
                &format_args!("{}", self.in_dscr_empty().bit()),
            )
            .field(
                "infifo_full_wm",
                &format_args!("{}", self.infifo_full_wm().bit()),
            )
            .field(
                "infifo_ovf_l1",
                &format_args!("{}", self.infifo_ovf_l1().bit()),
            )
            .field(
                "infifo_udf_l1",
                &format_args!("{}", self.infifo_udf_l1().bit()),
            )
            .field(
                "infifo_ovf_l3",
                &format_args!("{}", self.infifo_ovf_l3().bit()),
            )
            .field(
                "infifo_udf_l3",
                &format_args!("{}", self.infifo_udf_l3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_ST_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Masked interrupt of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_int_st_ch](index.html) module"]
pub struct IN_INT_ST_CH_SPEC;
impl crate::RegisterSpec for IN_INT_ST_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_int_st_ch::R](R) reader structure"]
impl crate::Readable for IN_INT_ST_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_INT_ST_CH%s to value 0"]
impl crate::Resettable for IN_INT_ST_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
