#[doc = "Register `INT_CLR_CH1` writer"]
pub struct W(crate::W<INT_CLR_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_CH1_SPEC>;
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
impl From<crate::W<INT_CLR_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_DONE` writer - Set this bit to clear the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `IN_SUC_EOF` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `IN_ERR_EOF` writer - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `OUT_DONE` writer - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `OUT_EOF` writer - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `IN_DSCR_ERR` writer - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `OUT_DSCR_ERR` writer - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `IN_DSCR_EMPTY` writer - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `OUT_TOTAL_EOF` writer - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `INFIFO_OVF` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `INFIFO_UDF` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `OUTFIFO_OVF` writer - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[doc = "Field `OUTFIFO_UDF` writer - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_CH1_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<0> {
        IN_DONE_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<1> {
        IN_SUC_EOF_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<2> {
        IN_ERR_EOF_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<3> {
        OUT_DONE_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<4> {
        OUT_EOF_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<5> {
        IN_DSCR_ERR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<6> {
        OUT_DSCR_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<7> {
        IN_DSCR_EMPTY_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<8> {
        OUT_TOTAL_EOF_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf(&mut self) -> INFIFO_OVF_W<9> {
        INFIFO_OVF_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf(&mut self) -> INFIFO_UDF_W<10> {
        INFIFO_UDF_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf(&mut self) -> OUTFIFO_OVF_W<11> {
        OUTFIFO_OVF_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf(&mut self) -> OUTFIFO_UDF_W<12> {
        OUTFIFO_UDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_INT_CLR_CH1_REG.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_ch1](index.html) module"]
pub struct INT_CLR_CH1_SPEC;
impl crate::RegisterSpec for INT_CLR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr_ch1::W](W) writer structure"]
impl crate::Writable for INT_CLR_CH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR_CH1 to value 0"]
impl crate::Resettable for INT_CLR_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
