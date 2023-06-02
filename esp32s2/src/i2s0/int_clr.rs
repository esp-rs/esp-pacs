#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAKE_DATA_INT_CLR` writer - Set this bit to clear I2S_RX_TAKE_DATA_INT interrupt."]
pub type TAKE_DATA_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `PUT_DATA_INT_CLR` writer - Set this bit to clear I2S_TX_PUT_DATA_INT interrupt."]
pub type PUT_DATA_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `RX_WFULL_INT_CLR` writer - Set this bit to clear I2S_RX_WFULL_INT interrupt."]
pub type RX_WFULL_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `RX_REMPTY_INT_CLR` writer - Set this bit to clear I2S_RX_REMPTY_INT interrupt."]
pub type RX_REMPTY_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TX_WFULL_INT_CLR` writer - Set this bit to clear I2S_TX_WFULL_INT interrupt."]
pub type TX_WFULL_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TX_REMPTY_INT_CLR` writer - Set this bit to clear I2S_TX_REMPTY_INT interrupt."]
pub type TX_REMPTY_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `RX_HUNG_INT_CLR` writer - Set this bit to clear I2S_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TX_HUNG_INT_CLR` writer - Set this bit to clear I2S_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `IN_DONE_INT_CLR` writer - Set this bit to clear I2S_IN_DONE_INT interrupt."]
pub type IN_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `IN_SUC_EOF_INT_CLR` writer - Set this bit to clear I2S_IN_SUC_EOF_INT interrupt."]
pub type IN_SUC_EOF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `IN_ERR_EOF_INT_CLR` writer - Reserved."]
pub type IN_ERR_EOF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OUT_DONE_INT_CLR` writer - Set this bit to clear I2S_OUT_DONE_INT interrupt."]
pub type OUT_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OUT_EOF_INT_CLR` writer - Set this bit to clear I2S_OUT_EOF_INT interrupt."]
pub type OUT_EOF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `IN_DSCR_ERR_INT_CLR` writer - Set this bit to clear I2S_IN_DSCR_ERR_INT interrupt."]
pub type IN_DSCR_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OUT_DSCR_ERR_INT_CLR` writer - Set this bit to clear I2S_OUT_DSCR_ERR_INT interrupt."]
pub type OUT_DSCR_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `IN_DSCR_EMPTY_INT_CLR` writer - Set this bit to clear I2S_IN_DSCR_EMPTY_INT interrupt."]
pub type IN_DSCR_EMPTY_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` writer - Set this bit to clear I2S_OUT_TOTAL_EOF_INT interrupt."]
pub type OUT_TOTAL_EOF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `V_SYNC_INT_CLR` writer - Set this bit to clear I2S_V_SYNC_INT interrupt."]
pub type V_SYNC_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear I2S_RX_TAKE_DATA_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn take_data_int_clr(&mut self) -> TAKE_DATA_INT_CLR_W<0> {
        TAKE_DATA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear I2S_TX_PUT_DATA_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn put_data_int_clr(&mut self) -> PUT_DATA_INT_CLR_W<1> {
        PUT_DATA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear I2S_RX_WFULL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wfull_int_clr(&mut self) -> RX_WFULL_INT_CLR_W<2> {
        RX_WFULL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear I2S_RX_REMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_rempty_int_clr(&mut self) -> RX_REMPTY_INT_CLR_W<3> {
        RX_REMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear I2S_TX_WFULL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wfull_int_clr(&mut self) -> TX_WFULL_INT_CLR_W<4> {
        TX_WFULL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear I2S_TX_REMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_rempty_int_clr(&mut self) -> TX_REMPTY_INT_CLR_W<5> {
        TX_REMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear I2S_RX_HUNG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W<6> {
        RX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear I2S_TX_HUNG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W<7> {
        TX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear I2S_IN_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_int_clr(&mut self) -> IN_DONE_INT_CLR_W<8> {
        IN_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear I2S_IN_SUC_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_int_clr(&mut self) -> IN_SUC_EOF_INT_CLR_W<9> {
        IN_SUC_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_int_clr(&mut self) -> IN_ERR_EOF_INT_CLR_W<10> {
        IN_ERR_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear I2S_OUT_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_int_clr(&mut self) -> OUT_DONE_INT_CLR_W<11> {
        OUT_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear I2S_OUT_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_clr(&mut self) -> OUT_EOF_INT_CLR_W<12> {
        OUT_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear I2S_IN_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err_int_clr(&mut self) -> IN_DSCR_ERR_INT_CLR_W<13> {
        IN_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear I2S_OUT_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_int_clr(&mut self) -> OUT_DSCR_ERR_INT_CLR_W<14> {
        OUT_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear I2S_IN_DSCR_EMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty_int_clr(&mut self) -> IN_DSCR_EMPTY_INT_CLR_W<15> {
        IN_DSCR_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear I2S_OUT_TOTAL_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_int_clr(&mut self) -> OUT_TOTAL_EOF_INT_CLR_W<16> {
        OUT_TOTAL_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to clear I2S_V_SYNC_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn v_sync_int_clr(&mut self) -> V_SYNC_INT_CLR_W<17> {
        V_SYNC_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
