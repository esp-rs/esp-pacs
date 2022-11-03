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
#[doc = "Field `IN_DONE_CH_INT_CLR` writer - Set this bit to clear the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `IN_SUC_EOF_CH_INT_CLR` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `IN_ERR_EOF_CH_INT_CLR` writer - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `IN_DSCR_ERR_CH_INT_CLR` writer - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `IN_DSCR_EMPTY_CH_INT_CLR` writer - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `INFIFO_OVF_CH_INT_CLR` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_INT_CLR_CH_SPEC, bool, O>;
#[doc = "Field `INFIFO_UDF_CH_INT_CLR` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_CH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_INT_CLR_CH_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_ch_int_clr(&mut self) -> IN_DONE_CH_INT_CLR_W<0> {
        IN_DONE_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_ch_int_clr(&mut self) -> IN_SUC_EOF_CH_INT_CLR_W<1> {
        IN_SUC_EOF_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_ch_int_clr(&mut self) -> IN_ERR_EOF_CH_INT_CLR_W<2> {
        IN_ERR_EOF_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err_ch_int_clr(&mut self) -> IN_DSCR_ERR_CH_INT_CLR_W<3> {
        IN_DSCR_ERR_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty_ch_int_clr(&mut self) -> IN_DSCR_EMPTY_CH_INT_CLR_W<4> {
        IN_DSCR_EMPTY_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_ch_int_clr(&mut self) -> INFIFO_OVF_CH_INT_CLR_W<5> {
        INFIFO_OVF_CH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_ch_int_clr(&mut self) -> INFIFO_UDF_CH_INT_CLR_W<6> {
        INFIFO_UDF_CH_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits of channel 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_int_clr_ch](index.html) module"]
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
