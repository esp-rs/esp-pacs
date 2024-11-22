#[doc = "Register `OUT_STATUS` reader"]
pub type R = crate::R<OUT_STATUS_SPEC>;
#[doc = "Field `OUT_FULL` reader - DMA TX FIFO is full."]
pub type OUT_FULL_R = crate::BitReader;
#[doc = "Field `OUT_EMPTY` reader - DMA TX FIFO is empty."]
pub type OUT_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA TX FIFO is full."]
    #[inline(always)]
    pub fn out_full(&self) -> OUT_FULL_R {
        OUT_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA TX FIFO is empty."]
    #[inline(always)]
    pub fn out_empty(&self) -> OUT_EMPTY_R {
        OUT_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_STATUS")
            .field("out_full", &self.out_full())
            .field("out_empty", &self.out_empty())
            .finish()
    }
}
#[doc = "TX FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_STATUS_SPEC;
impl crate::RegisterSpec for OUT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_status::R`](R) reader structure"]
impl crate::Readable for OUT_STATUS_SPEC {}
#[doc = "`reset()` method sets OUT_STATUS to value 0x02"]
impl crate::Resettable for OUT_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
