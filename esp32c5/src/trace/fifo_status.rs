#[doc = "Register `FIFO_STATUS` reader"]
pub type R = crate::R<FIFO_STATUS_SPEC>;
#[doc = "Field `FIFO_EMPTY` reader - Represent whether the FIFO is empty. \\\\1: Empty \\\\0: Not empty"]
pub type FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `WORK_STATUS` reader - Represent the state of the encoder: \\\\0: Idle state \\\\1: Working state\\\\ 2: Wait state becasue hart is halted or in reset \\\\3: Lost state\\\\"]
pub type WORK_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Represent whether the FIFO is empty. \\\\1: Empty \\\\0: Not empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Represent the state of the encoder: \\\\0: Idle state \\\\1: Working state\\\\ 2: Wait state becasue hart is halted or in reset \\\\3: Lost state\\\\"]
    #[inline(always)]
    pub fn work_status(&self) -> WORK_STATUS_R {
        WORK_STATUS_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_STATUS")
            .field("fifo_empty", &self.fifo_empty())
            .field("work_status", &self.work_status())
            .finish()
    }
}
#[doc = "FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_STATUS_SPEC;
impl crate::RegisterSpec for FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_status::R`](R) reader structure"]
impl crate::Readable for FIFO_STATUS_SPEC {}
#[doc = "`reset()` method sets FIFO_STATUS to value 0x01"]
impl crate::Resettable for FIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
