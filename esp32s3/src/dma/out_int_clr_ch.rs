#[doc = "Register `OUT_INT_CLR_CH%s` writer"]
pub struct W(crate::W<OUT_INT_CLR_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_INT_CLR_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OUT_INT_CLR_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_INT_CLR_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_DONE_CH_INT_CLR` writer - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `OUT_EOF_CH_INT_CLR` writer - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_CLR` writer - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_CLR` writer - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `OUTFIFO_OVF_L1_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `OUTFIFO_UDF_L1_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `OUTFIFO_OVF_L3_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_OVF_L3_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `OUTFIFO_UDF_L3_CH_INT_CLR` writer - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_UDF_L3_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_CLR_CH_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch_int_clr(&mut self) -> OUT_DONE_CH_INT_CLR_W<0> {
        OUT_DONE_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch_int_clr(&mut self) -> OUT_EOF_CH_INT_CLR_W<1> {
        OUT_EOF_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch_int_clr(&mut self) -> OUT_DSCR_ERR_CH_INT_CLR_W<2> {
        OUT_DSCR_ERR_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch_int_clr(&mut self) -> OUT_TOTAL_EOF_CH_INT_CLR_W<3> {
        OUT_TOTAL_EOF_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1_ch_int_clr(&mut self) -> OUTFIFO_OVF_L1_CH_INT_CLR_W<4> {
        OUTFIFO_OVF_L1_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1_ch_int_clr(&mut self) -> OUTFIFO_UDF_L1_CH_INT_CLR_W<5> {
        OUTFIFO_UDF_L1_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l3_ch_int_clr(&mut self) -> OUTFIFO_OVF_L3_CH_INT_CLR_W<6> {
        OUTFIFO_OVF_L3_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l3_ch_int_clr(&mut self) -> OUTFIFO_UDF_L3_CH_INT_CLR_W<7> {
        OUTFIFO_UDF_L3_CH_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits of Tx channel 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_int_clr_ch](index.html) module"]
pub struct OUT_INT_CLR_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_CLR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [out_int_clr_ch::W](W) writer structure"]
impl crate::Writable for OUT_INT_CLR_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_INT_CLR_CH%s to value 0"]
impl crate::Resettable for OUT_INT_CLR_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
