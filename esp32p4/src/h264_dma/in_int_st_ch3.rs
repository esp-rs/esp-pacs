#[doc = "Register `IN_INT_ST_CH3` reader"]
pub type R = crate::R<IN_INT_ST_CH3_SPEC>;
#[doc = "Field `IN_DONE_CH3_INT_ST` reader - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_CH3_INT_ST` reader - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_CH3_INT_ST` reader - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_CH3_INT_ST` reader - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1_CH3_INT_ST` reader - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1_CH3_INT_ST` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L2_CH3_INT_ST` reader - The raw interrupt status bit for the INFIFO_OVF_L2_CH_INT interrupt."]
pub type INFIFO_OVF_L2_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L2_CH3_INT_ST` reader - The raw interrupt status bit for the INFIFO_UDF_L2_CH_INT interrupt."]
pub type INFIFO_UDF_L2_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_CH3_INT_ST` reader - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_DSCR_TASK_OVF_CH3_INT_ST` reader - The raw interrupt status bit for the IN_DSCR_TASK_OVF_CH_INT interrupt."]
pub type IN_DSCR_TASK_OVF_CH3_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch3_int_st(&self) -> IN_DONE_CH3_INT_ST_R {
        IN_DONE_CH3_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch3_int_st(&self) -> IN_SUC_EOF_CH3_INT_ST_R {
        IN_SUC_EOF_CH3_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch3_int_st(&self) -> IN_ERR_EOF_CH3_INT_ST_R {
        IN_ERR_EOF_CH3_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch3_int_st(&self) -> IN_DSCR_ERR_CH3_INT_ST_R {
        IN_DSCR_ERR_CH3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1_ch3_int_st(&self) -> INFIFO_OVF_L1_CH3_INT_ST_R {
        INFIFO_OVF_L1_CH3_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1_ch3_int_st(&self) -> INFIFO_UDF_L1_CH3_INT_ST_R {
        INFIFO_UDF_L1_CH3_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l2_ch3_int_st(&self) -> INFIFO_OVF_L2_CH3_INT_ST_R {
        INFIFO_OVF_L2_CH3_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l2_ch3_int_st(&self) -> INFIFO_UDF_L2_CH3_INT_ST_R {
        INFIFO_UDF_L2_CH3_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch3_int_st(&self) -> IN_DSCR_EMPTY_CH3_INT_ST_R {
        IN_DSCR_EMPTY_CH3_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the IN_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_task_ovf_ch3_int_st(&self) -> IN_DSCR_TASK_OVF_CH3_INT_ST_R {
        IN_DSCR_TASK_OVF_CH3_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_INT_ST_CH3")
            .field(
                "in_done_ch3_int_st",
                &format_args!("{}", self.in_done_ch3_int_st().bit()),
            )
            .field(
                "in_suc_eof_ch3_int_st",
                &format_args!("{}", self.in_suc_eof_ch3_int_st().bit()),
            )
            .field(
                "in_err_eof_ch3_int_st",
                &format_args!("{}", self.in_err_eof_ch3_int_st().bit()),
            )
            .field(
                "in_dscr_err_ch3_int_st",
                &format_args!("{}", self.in_dscr_err_ch3_int_st().bit()),
            )
            .field(
                "infifo_ovf_l1_ch3_int_st",
                &format_args!("{}", self.infifo_ovf_l1_ch3_int_st().bit()),
            )
            .field(
                "infifo_udf_l1_ch3_int_st",
                &format_args!("{}", self.infifo_udf_l1_ch3_int_st().bit()),
            )
            .field(
                "infifo_ovf_l2_ch3_int_st",
                &format_args!("{}", self.infifo_ovf_l2_ch3_int_st().bit()),
            )
            .field(
                "infifo_udf_l2_ch3_int_st",
                &format_args!("{}", self.infifo_udf_l2_ch3_int_st().bit()),
            )
            .field(
                "in_dscr_empty_ch3_int_st",
                &format_args!("{}", self.in_dscr_empty_ch3_int_st().bit()),
            )
            .field(
                "in_dscr_task_ovf_ch3_int_st",
                &format_args!("{}", self.in_dscr_task_ovf_ch3_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_ST_CH3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RX CH3 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_ST_CH3_SPEC;
impl crate::RegisterSpec for IN_INT_ST_CH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_int_st_ch3::R`](R) reader structure"]
impl crate::Readable for IN_INT_ST_CH3_SPEC {}
#[doc = "`reset()` method sets IN_INT_ST_CH3 to value 0"]
impl crate::Resettable for IN_INT_ST_CH3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
