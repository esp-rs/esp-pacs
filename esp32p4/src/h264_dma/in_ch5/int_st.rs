///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `IN_DONE` reader - The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
pub type IN_DONE_R = crate::BitReader;
///Field `IN_SUC_EOF` reader - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
pub type IN_SUC_EOF_R = crate::BitReader;
///Field `INFIFO_OVF_L1` reader - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
pub type INFIFO_OVF_L1_R = crate::BitReader;
///Field `INFIFO_UDF_L1` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
pub type INFIFO_UDF_L1_R = crate::BitReader;
///Field `FETCH_MB_COL_CNT_OVF` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
pub type FETCH_MB_COL_CNT_OVF_R = crate::BitReader;
impl R {
    ///Bit 0 - The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> INFIFO_OVF_L1_R {
        INFIFO_OVF_L1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> INFIFO_UDF_L1_R {
        INFIFO_UDF_L1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn fetch_mb_col_cnt_ovf(&self) -> FETCH_MB_COL_CNT_OVF_R {
        FETCH_MB_COL_CNT_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("infifo_ovf_l1", &self.infifo_ovf_l1())
            .field("infifo_udf_l1", &self.infifo_udf_l1())
            .field("fetch_mb_col_cnt_ovf", &self.fetch_mb_col_cnt_ovf())
            .finish()
    }
}
/**RX CH5 interrupt st register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_ST_SPEC {}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
