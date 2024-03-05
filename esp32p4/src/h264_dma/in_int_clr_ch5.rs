#[doc = "Register `IN_INT_CLR_CH5` writer"]
pub type W = crate::W<IN_INT_CLR_CH5_SPEC>;
#[doc = "Field `IN_DONE_CH5_INT_CLR` writer - Set this bit to clear the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH5_INT_CLR` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1_CH5_INT_CLR` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_CH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1_CH5_INT_CLR` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_CH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCH_MB_COL_CNT_OVF_CH5_INT_CLR` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type FETCH_MB_COL_CNT_OVF_CH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_CLR_CH5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_ch5_int_clr(&mut self) -> IN_DONE_CH5_INT_CLR_W<IN_INT_CLR_CH5_SPEC> {
        IN_DONE_CH5_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_ch5_int_clr(&mut self) -> IN_SUC_EOF_CH5_INT_CLR_W<IN_INT_CLR_CH5_SPEC> {
        IN_SUC_EOF_CH5_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1_ch5_int_clr(
        &mut self,
    ) -> INFIFO_OVF_L1_CH5_INT_CLR_W<IN_INT_CLR_CH5_SPEC> {
        INFIFO_OVF_L1_CH5_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1_ch5_int_clr(
        &mut self,
    ) -> INFIFO_UDF_L1_CH5_INT_CLR_W<IN_INT_CLR_CH5_SPEC> {
        INFIFO_UDF_L1_CH5_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fetch_mb_col_cnt_ovf_ch5_int_clr(
        &mut self,
    ) -> FETCH_MB_COL_CNT_OVF_CH5_INT_CLR_W<IN_INT_CLR_CH5_SPEC> {
        FETCH_MB_COL_CNT_OVF_CH5_INT_CLR_W::new(self, 4)
    }
}
#[doc = "RX CH5 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_CLR_CH5_SPEC;
impl crate::RegisterSpec for IN_INT_CLR_CH5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`in_int_clr_ch5::W`](W) writer structure"]
impl crate::Writable for IN_INT_CLR_CH5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_INT_CLR_CH5 to value 0"]
impl crate::Resettable for IN_INT_CLR_CH5_SPEC {
    const RESET_VALUE: u32 = 0;
}
