#[doc = "Register `IN_STATUS` reader"]
pub type R = crate::R<IN_STATUS_SPEC>;
#[doc = "Field `IN_FULL` reader - DMA RX FIFO is full."]
pub type IN_FULL_R = crate::BitReader;
#[doc = "Field `IN_EMPTY` reader - DMA RX FIFO is empty."]
pub type IN_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA RX FIFO is full."]
    #[inline(always)]
    pub fn in_full(&self) -> IN_FULL_R {
        IN_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA RX FIFO is empty."]
    #[inline(always)]
    pub fn in_empty(&self) -> IN_EMPTY_R {
        IN_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_STATUS")
            .field("in_full", &self.in_full())
            .field("in_empty", &self.in_empty())
            .finish()
    }
}
#[doc = "RX FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_STATUS_SPEC;
impl crate::RegisterSpec for IN_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_status::R`](R) reader structure"]
impl crate::Readable for IN_STATUS_SPEC {}
#[doc = "`reset()` method sets IN_STATUS to value 0x02"]
impl crate::Resettable for IN_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
