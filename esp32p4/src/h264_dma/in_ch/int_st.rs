#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `IN_DONE` reader - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` reader - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1` reader - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L2` reader - The raw interrupt status bit for the INFIFO_OVF_L2_CH_INT interrupt."]
pub type INFIFO_OVF_L2_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L2` reader - The raw interrupt status bit for the INFIFO_UDF_L2_CH_INT interrupt."]
pub type INFIFO_UDF_L2_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `IN_DSCR_TASK_OVF` reader - The raw interrupt status bit for the IN_DSCR_TASK_OVF_CH_INT interrupt."]
pub type IN_DSCR_TASK_OVF_R = crate::BitReader;
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
    #[doc = "Bit 4 - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> INFIFO_OVF_L1_R {
        INFIFO_OVF_L1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> INFIFO_UDF_L1_R {
        INFIFO_UDF_L1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l2(&self) -> INFIFO_OVF_L2_R {
        INFIFO_OVF_L2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l2(&self) -> INFIFO_UDF_L2_R {
        INFIFO_UDF_L2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the IN_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_task_ovf(&self) -> IN_DSCR_TASK_OVF_R {
        IN_DSCR_TASK_OVF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("infifo_ovf_l1", &self.infifo_ovf_l1())
            .field("infifo_udf_l1", &self.infifo_udf_l1())
            .field("infifo_ovf_l2", &self.infifo_ovf_l2())
            .field("infifo_udf_l2", &self.infifo_udf_l2())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("in_dscr_task_ovf", &self.in_dscr_task_ovf())
            .finish()
    }
}
#[doc = "RX CHx interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
