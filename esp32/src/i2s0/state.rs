#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `TX_IDLE` reader - "]
pub type TX_IDLE_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RESET_BACK` reader - "]
pub type TX_FIFO_RESET_BACK_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RESET_BACK` reader - "]
pub type RX_FIFO_RESET_BACK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TX_IDLE_R {
        TX_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_reset_back(&self) -> TX_FIFO_RESET_BACK_R {
        TX_FIFO_RESET_BACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_reset_back(&self) -> RX_FIFO_RESET_BACK_R {
        RX_FIFO_RESET_BACK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("tx_idle", &self.tx_idle())
            .field("tx_fifo_reset_back", &self.tx_fifo_reset_back())
            .field("rx_fifo_reset_back", &self.rx_fifo_reset_back())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0x07"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
