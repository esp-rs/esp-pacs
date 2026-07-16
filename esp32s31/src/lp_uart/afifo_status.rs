#[doc = "Register `AFIFO_STATUS` reader"]
pub type R = crate::R<AFIFO_STATUS_SPEC>;
#[doc = "Field `TX_AFIFO_FULL` reader - Represents whether or not the APB TX asynchronous FIFO is full.\\\\ 0: Not full\\\\ 1: Full\\\\"]
pub type TX_AFIFO_FULL_R = crate::BitReader;
#[doc = "Field `TX_AFIFO_EMPTY` reader - Represents whether or not the APB TX asynchronous FIFO is empty.\\\\ 0: Not empty\\\\ 1: Empty\\\\"]
pub type TX_AFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `RX_AFIFO_FULL` reader - Represents whether or not the APB RX asynchronous FIFO is full.\\\\ 0: Not full\\\\ 1: Full\\\\"]
pub type RX_AFIFO_FULL_R = crate::BitReader;
#[doc = "Field `RX_AFIFO_EMPTY` reader - Represents whether or not the APB RX asynchronous FIFO is empty.\\\\ 0: Not empty\\\\ 1: Empty\\\\"]
pub type RX_AFIFO_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not the APB TX asynchronous FIFO is full.\\\\ 0: Not full\\\\ 1: Full\\\\"]
    #[inline(always)]
    pub fn tx_afifo_full(&self) -> TX_AFIFO_FULL_R {
        TX_AFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether or not the APB TX asynchronous FIFO is empty.\\\\ 0: Not empty\\\\ 1: Empty\\\\"]
    #[inline(always)]
    pub fn tx_afifo_empty(&self) -> TX_AFIFO_EMPTY_R {
        TX_AFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents whether or not the APB RX asynchronous FIFO is full.\\\\ 0: Not full\\\\ 1: Full\\\\"]
    #[inline(always)]
    pub fn rx_afifo_full(&self) -> RX_AFIFO_FULL_R {
        RX_AFIFO_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents whether or not the APB RX asynchronous FIFO is empty.\\\\ 0: Not empty\\\\ 1: Empty\\\\"]
    #[inline(always)]
    pub fn rx_afifo_empty(&self) -> RX_AFIFO_EMPTY_R {
        RX_AFIFO_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFIFO_STATUS")
            .field("tx_afifo_full", &self.tx_afifo_full())
            .field("tx_afifo_empty", &self.tx_afifo_empty())
            .field("rx_afifo_full", &self.rx_afifo_full())
            .field("rx_afifo_empty", &self.rx_afifo_empty())
            .finish()
    }
}
#[doc = "LP UART asynchronous FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`afifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFIFO_STATUS_SPEC;
impl crate::RegisterSpec for AFIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afifo_status::R`](R) reader structure"]
impl crate::Readable for AFIFO_STATUS_SPEC {}
#[doc = "`reset()` method sets AFIFO_STATUS to value 0x0a"]
impl crate::Resettable for AFIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0a;
}
