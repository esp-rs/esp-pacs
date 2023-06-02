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
#[doc = "Field `RX_START_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_RX_START_INT."]
pub type RX_START_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TX_START_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_TX_START_INT."]
pub type TX_START_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `RX_HUNG_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_RX_HUNG_INT."]
pub type RX_HUNG_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TX_HUNG_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_TX_HUNG_INT."]
pub type TX_HUNG_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `SEND_S_REG_Q_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_SEND_S_REG_Q_INT."]
pub type SEND_S_REG_Q_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `SEND_A_REG_Q_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_SEND_A_REG_Q_INT."]
pub type SEND_A_REG_Q_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_OUT_EOF_INT."]
pub type OUTLINK_EOF_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `APP_CTRL0_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `APP_CTRL1_INT_CLR` writer - Set this bit to clear the raw interrupt of UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the raw interrupt of UHCI_RX_START_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rx_start_int_clr(&mut self) -> RX_START_INT_CLR_W<0> {
        RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the raw interrupt of UHCI_TX_START_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_int_clr(&mut self) -> TX_START_INT_CLR_W<1> {
        TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the raw interrupt of UHCI_RX_HUNG_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W<2> {
        RX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the raw interrupt of UHCI_TX_HUNG_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W<3> {
        TX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the raw interrupt of UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    #[must_use]
    pub fn send_s_reg_q_int_clr(&mut self) -> SEND_S_REG_Q_INT_CLR_W<4> {
        SEND_S_REG_Q_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the raw interrupt of UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    #[must_use]
    pub fn send_a_reg_q_int_clr(&mut self) -> SEND_A_REG_Q_INT_CLR_W<5> {
        SEND_A_REG_Q_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the raw interrupt of UHCI_OUT_EOF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_eof_err_int_clr(&mut self) -> OUTLINK_EOF_ERR_INT_CLR_W<6> {
        OUTLINK_EOF_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the raw interrupt of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl0_int_clr(&mut self) -> APP_CTRL0_INT_CLR_W<7> {
        APP_CTRL0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the raw interrupt of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl1_int_clr(&mut self) -> APP_CTRL1_INT_CLR_W<8> {
        APP_CTRL1_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
