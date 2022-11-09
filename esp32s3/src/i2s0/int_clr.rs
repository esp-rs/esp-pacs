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
#[doc = "Field `RX_DONE_INT_CLR` writer - Set this bit to clear the i2s_rx_done_int interrupt"]
pub type RX_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `TX_DONE_INT_CLR` writer - Set this bit to clear the i2s_tx_done_int interrupt"]
pub type TX_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `RX_HUNG_INT_CLR` writer - Set this bit to clear the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `TX_HUNG_INT_CLR` writer - Set this bit to clear the i2s_tx_hung_int interrupt"]
pub type TX_HUNG_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the i2s_rx_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done_int_clr(&mut self) -> RX_DONE_INT_CLR_W<0> {
        RX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the i2s_tx_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_done_int_clr(&mut self) -> TX_DONE_INT_CLR_W<1> {
        TX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W<2> {
        RX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W<3> {
        TX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S interrupt clear register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
