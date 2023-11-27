#[doc = "Register `IN_INT_CLR_CH0` writer"]
pub type W = crate::W<IN_INT_CLR_CH0_SPEC>;
#[doc = "Field `IN_DONE_CH0_INT_CLR` writer - Set this bit to clear the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH0_INT_CLR` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_CH0_INT_CLR` writer - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_CLR` writer - Set this bit to clear the INDSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1_CH0_INT_CLR` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1_CH0_INT_CLR` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L2_CH0_INT_CLR` writer - Set this bit to clear the INFIFO_OVF_L2_CH_INT interrupt."]
pub type INFIFO_OVF_L2_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L2_CH0_INT_CLR` writer - Set this bit to clear the INFIFO_UDF_L2_CH_INT interrupt."]
pub type INFIFO_UDF_L2_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_CLR` writer - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_TASK_OVF_CH0_INT_CLR` writer - Set this bit to clear the IN_DSCR_TASK_OVF_CH_INT interrupt."]
pub type IN_DSCR_TASK_OVF_CH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_CLR_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_ch0_int_clr(&mut self) -> IN_DONE_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        IN_DONE_CH0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_ch0_int_clr(&mut self) -> IN_SUC_EOF_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        IN_SUC_EOF_CH0_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_ch0_int_clr(&mut self) -> IN_ERR_EOF_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        IN_ERR_EOF_CH0_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the INDSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err_ch0_int_clr(&mut self) -> IN_DSCR_ERR_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        IN_DSCR_ERR_CH0_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1_ch0_int_clr(
        &mut self,
    ) -> INFIFO_OVF_L1_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        INFIFO_OVF_L1_CH0_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1_ch0_int_clr(
        &mut self,
    ) -> INFIFO_UDF_L1_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        INFIFO_UDF_L1_CH0_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l2_ch0_int_clr(
        &mut self,
    ) -> INFIFO_OVF_L2_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        INFIFO_OVF_L2_CH0_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l2_ch0_int_clr(
        &mut self,
    ) -> INFIFO_UDF_L2_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        INFIFO_UDF_L2_CH0_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty_ch0_int_clr(
        &mut self,
    ) -> IN_DSCR_EMPTY_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        IN_DSCR_EMPTY_CH0_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the IN_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_task_ovf_ch0_int_clr(
        &mut self,
    ) -> IN_DSCR_TASK_OVF_CH0_INT_CLR_W<IN_INT_CLR_CH0_SPEC> {
        IN_DSCR_TASK_OVF_CH0_INT_CLR_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RX CH0 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_CLR_CH0_SPEC;
impl crate::RegisterSpec for IN_INT_CLR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`in_int_clr_ch0::W`](W) writer structure"]
impl crate::Writable for IN_INT_CLR_CH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_INT_CLR_CH0 to value 0"]
impl crate::Resettable for IN_INT_CLR_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
