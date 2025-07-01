#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `OUT_DONE` reader - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L1` reader - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L1` reader - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L2` reader - The raw interrupt status bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
pub type OUTFIFO_OVF_L2_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L2` reader - The raw interrupt status bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
pub type OUTFIFO_UDF_L2_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_TASK_OVF` reader - The raw interrupt status bit for the OUT_DSCR_TASK_OVF_CH_INT interrupt."]
pub type OUT_DSCR_TASK_OVF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&self) -> OUTFIFO_OVF_L1_R {
        OUTFIFO_OVF_L1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&self) -> OUTFIFO_UDF_L1_R {
        OUTFIFO_UDF_L1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l2(&self) -> OUTFIFO_OVF_L2_R {
        OUTFIFO_OVF_L2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l2(&self) -> OUTFIFO_UDF_L2_R {
        OUTFIFO_UDF_L2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the OUT_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_task_ovf(&self) -> OUT_DSCR_TASK_OVF_R {
        OUT_DSCR_TASK_OVF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("out_total_eof", &self.out_total_eof())
            .field("outfifo_ovf_l1", &self.outfifo_ovf_l1())
            .field("outfifo_udf_l1", &self.outfifo_udf_l1())
            .field("outfifo_ovf_l2", &self.outfifo_ovf_l2())
            .field("outfifo_udf_l2", &self.outfifo_udf_l2())
            .field("out_dscr_task_ovf", &self.out_dscr_task_ovf())
            .finish()
    }
}
#[doc = "TX CHx interrupt st register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
