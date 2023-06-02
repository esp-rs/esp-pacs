#[doc = "Register `IN_INT_CLR_CH%s` writer"]
pub struct W(crate::W<IN_INT_CLR_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_INT_CLR_CH_SPEC>;
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
impl From<crate::W<IN_INT_CLR_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_INT_CLR_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_DONE` writer - Set this bit to clear the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `IN_SUC_EOF` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `IN_ERR_EOF` writer - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `IN_DSCR_ERR` writer - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `IN_DSCR_EMPTY` writer - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `DMA_INFIFO_FULL_WM` writer - Set this bit to clear the INFIFO_FULL_WM_CH_INT interrupt."]
pub type DMA_INFIFO_FULL_WM_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `INFIFO_OVF_L1` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `INFIFO_UDF_L1` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `INFIFO_OVF_L3` writer - Set this bit to clear the INFIFO_OVF_L3_CH_INT interrupt."]
pub type INFIFO_OVF_L3_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[doc = "Field `INFIFO_UDF_L3` writer - Set this bit to clear the INFIFO_UDF_L3_CH_INT interrupt."]
pub type INFIFO_UDF_L3_W<'a, const O: u8> = crate::BitWriter<'a, IN_INT_CLR_CH_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_CLR_CH_SPEC> {
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
    #[doc = "Bit 3 - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<3> {
        IN_DSCR_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<4> {
        IN_DSCR_EMPTY_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the INFIFO_FULL_WM_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_wm(&mut self) -> DMA_INFIFO_FULL_WM_W<5> {
        DMA_INFIFO_FULL_WM_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1(&mut self) -> INFIFO_OVF_L1_W<6> {
        INFIFO_OVF_L1_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1(&mut self) -> INFIFO_UDF_L1_W<7> {
        INFIFO_UDF_L1_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l3(&mut self) -> INFIFO_OVF_L3_W<8> {
        INFIFO_OVF_L3_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l3(&mut self) -> INFIFO_UDF_L3_W<9> {
        INFIFO_UDF_L3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits of Rx channel 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_int_clr_ch](index.html) module"]
pub struct IN_INT_CLR_CH_SPEC;
impl crate::RegisterSpec for IN_INT_CLR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [in_int_clr_ch::W](W) writer structure"]
impl crate::Writable for IN_INT_CLR_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_INT_CLR_CH%s to value 0"]
impl crate::Resettable for IN_INT_CLR_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
