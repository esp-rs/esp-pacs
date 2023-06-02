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
#[doc = "Field `OUT_DONE` writer - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, OUT_INT_CLR_CH_SPEC, O>;
#[doc = "Field `OUT_EOF` writer - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_W<'a, const O: u8> = crate::BitWriter<'a, OUT_INT_CLR_CH_SPEC, O>;
#[doc = "Field `OUT_DSCR_ERR` writer - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, OUT_INT_CLR_CH_SPEC, O>;
#[doc = "Field `OUT_TOTAL_EOF` writer - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_W<'a, const O: u8> = crate::BitWriter<'a, OUT_INT_CLR_CH_SPEC, O>;
#[doc = "Field `OUTFIFO_OVF_L1` writer - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_W<'a, const O: u8> = crate::BitWriter<'a, OUT_INT_CLR_CH_SPEC, O>;
#[doc = "Field `OUTFIFO_UDF_L1` writer - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_W<'a, const O: u8> = crate::BitWriter<'a, OUT_INT_CLR_CH_SPEC, O>;
#[doc = "Field `OUTFIFO_OVF_L3` writer - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_OVF_L3_W<'a, const O: u8> = crate::BitWriter<'a, OUT_INT_CLR_CH_SPEC, O>;
#[doc = "Field `OUTFIFO_UDF_L3` writer - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_UDF_L3_W<'a, const O: u8> = crate::BitWriter<'a, OUT_INT_CLR_CH_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_CLR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<0> {
        OUT_DONE_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<1> {
        OUT_EOF_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<2> {
        OUT_DSCR_ERR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<3> {
        OUT_TOTAL_EOF_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_l1(&mut self) -> OUTFIFO_OVF_L1_W<4> {
        OUTFIFO_OVF_L1_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_l1(&mut self) -> OUTFIFO_UDF_L1_W<5> {
        OUTFIFO_UDF_L1_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_l3(&mut self) -> OUTFIFO_OVF_L3_W<6> {
        OUTFIFO_OVF_L3_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_l3(&mut self) -> OUTFIFO_UDF_L3_W<7> {
        OUTFIFO_UDF_L3_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_INT_CLR_CH%s to value 0"]
impl crate::Resettable for OUT_INT_CLR_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
