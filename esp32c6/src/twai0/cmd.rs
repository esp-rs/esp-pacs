#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_REQ` writer - 1: present, a message shall be transmitted. 0: absent"]
pub type TX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `ABORT_TX` writer - 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent"]
pub type ABORT_TX_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `RELEASE_BUF` writer - 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action"]
pub type RELEASE_BUF_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `CLEAR_DATA_OVERRUN` writer - 1: clear, the data overrun status bit is cleared. 0: no action."]
pub type CLEAR_DATA_OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `SELF_RX_REQUEST` writer - 1: present, a message shall be transmitted and received simultaneously. 0: absent."]
pub type SELF_RX_REQUEST_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - 1: present, a message shall be transmitted. 0: absent"]
    #[inline(always)]
    #[must_use]
    pub fn tx_req(&mut self) -> TX_REQ_W<0> {
        TX_REQ_W::new(self)
    }
    #[doc = "Bit 1 - 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent"]
    #[inline(always)]
    #[must_use]
    pub fn abort_tx(&mut self) -> ABORT_TX_W<1> {
        ABORT_TX_W::new(self)
    }
    #[doc = "Bit 2 - 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action"]
    #[inline(always)]
    #[must_use]
    pub fn release_buf(&mut self) -> RELEASE_BUF_W<2> {
        RELEASE_BUF_W::new(self)
    }
    #[doc = "Bit 3 - 1: clear, the data overrun status bit is cleared. 0: no action."]
    #[inline(always)]
    #[must_use]
    pub fn clear_data_overrun(&mut self) -> CLEAR_DATA_OVERRUN_W<3> {
        CLEAR_DATA_OVERRUN_W::new(self)
    }
    #[doc = "Bit 4 - 1: present, a message shall be transmitted and received simultaneously. 0: absent."]
    #[inline(always)]
    #[must_use]
    pub fn self_rx_request(&mut self) -> SELF_RX_REQUEST_W<4> {
        SELF_RX_REQUEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWAI command register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
