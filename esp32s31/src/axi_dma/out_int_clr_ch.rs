#[doc = "Register `OUT_INT_CLR_CH%s` writer"]
pub type W = crate::W<OUT_INT_CLR_CH_SPEC>;
#[doc = "Field `OUT_DONE_CH_INT_CLR` writer - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH_INT_CLR` writer - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_CLR` writer - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_CLR` writer - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L1_OVF_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_L1_OVF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L1_UDF_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_L1_UDF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L2_OVF_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_OVF_L2_CH_INT interrupt."]
pub type OUTFIFO_L2_OVF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L2_UDF_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_UDF_L2_CH_INT interrupt."]
pub type OUTFIFO_L2_UDF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L3_OVF_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_L3_OVF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L3_UDF_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_L3_UDF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LINK_SWITCH_CH_INT_CLR` writer - Set this bit to clear the OUT_LINK_SWITCH_CH_INT interrupt."]
pub type OUT_LINK_SWITCH_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_CLR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch_int_clr(&mut self) -> OUT_DONE_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUT_DONE_CH_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch_int_clr(&mut self) -> OUT_EOF_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUT_EOF_CH_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch_int_clr(
        &mut self,
    ) -> OUT_DSCR_ERR_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUT_DSCR_ERR_CH_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch_int_clr(
        &mut self,
    ) -> OUT_TOTAL_EOF_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUT_TOTAL_EOF_CH_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l1_ovf_ch_int_clr(
        &mut self,
    ) -> OUTFIFO_L1_OVF_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUTFIFO_L1_OVF_CH_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l1_udf_ch_int_clr(
        &mut self,
    ) -> OUTFIFO_L1_UDF_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUTFIFO_L1_UDF_CH_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the OUTFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l2_ovf_ch_int_clr(
        &mut self,
    ) -> OUTFIFO_L2_OVF_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUTFIFO_L2_OVF_CH_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the OUTFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l2_udf_ch_int_clr(
        &mut self,
    ) -> OUTFIFO_L2_UDF_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUTFIFO_L2_UDF_CH_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l3_ovf_ch_int_clr(
        &mut self,
    ) -> OUTFIFO_L3_OVF_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUTFIFO_L3_OVF_CH_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l3_udf_ch_int_clr(
        &mut self,
    ) -> OUTFIFO_L3_UDF_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUTFIFO_L3_UDF_CH_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear the OUT_LINK_SWITCH_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_link_switch_ch_int_clr(
        &mut self,
    ) -> OUT_LINK_SWITCH_CH_INT_CLR_W<'_, OUT_INT_CLR_CH_SPEC> {
        OUT_LINK_SWITCH_CH_INT_CLR_W::new(self, 10)
    }
}
#[doc = "Interrupt clear bits of channel%s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_clr_ch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_CLR_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_CLR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_int_clr_ch::W`](W) writer structure"]
impl crate::Writable for OUT_INT_CLR_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_INT_CLR_CH%s to value 0"]
impl crate::Resettable for OUT_INT_CLR_CH_SPEC {}
