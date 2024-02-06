#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `TX_REQ` writer - 1: present, a message shall be transmitted. 0: absent"]
pub type TX_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_TX` writer - 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent"]
pub type ABORT_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RELEASE_BUF` writer - 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action"]
pub type RELEASE_BUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_DATA_OVERRUN` writer - 1: clear, the data overrun status bit is cleared. 0: no action."]
pub type CLEAR_DATA_OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELF_RX_REQUEST` writer - 1: present, a message shall be transmitted and received simultaneously. 0: absent."]
pub type SELF_RX_REQUEST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn tx_req(&mut self) -> TX_REQ_W<CMD_SPEC> {
        TX_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent"]
    #[inline(always)]
    #[must_use]
    pub fn abort_tx(&mut self) -> ABORT_TX_W<CMD_SPEC> {
        ABORT_TX_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action"]
    #[inline(always)]
    #[must_use]
    pub fn release_buf(&mut self) -> RELEASE_BUF_W<CMD_SPEC> {
        RELEASE_BUF_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: clear, the data overrun status bit is cleared. 0: no action."]
    #[inline(always)]
    #[must_use]
    pub fn clear_data_overrun(&mut self) -> CLEAR_DATA_OVERRUN_W<CMD_SPEC> {
        CLEAR_DATA_OVERRUN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: present, a message shall be transmitted and received simultaneously. 0: absent."]
    #[inline(always)]
    #[must_use]
    pub fn self_rx_request(&mut self) -> SELF_RX_REQUEST_W<CMD_SPEC> {
        SELF_RX_REQUEST_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TWAI command register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
