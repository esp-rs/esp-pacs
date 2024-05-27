///Register `FIFO_STATUS` reader
pub type R = crate::R<FIFO_STATUS_SPEC>;
///Field `FIFO_EMPTY` reader - 1 indicate that fifo is empty
pub type FIFO_EMPTY_R = crate::BitReader;
///Field `WORK_STATUS` reader - mem_full interrupt status
pub type WORK_STATUS_R = crate::BitReader;
impl R {
    ///Bit 0 - 1 indicate that fifo is empty
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - mem_full interrupt status
    #[inline(always)]
    pub fn work_status(&self) -> WORK_STATUS_R {
        WORK_STATUS_R::new(((self.bits >> 1) & 1) != 0)
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
/**fifo status register

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIFO_STATUS_SPEC;
impl crate::RegisterSpec for FIFO_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fifo_status::R`](R) reader structure
impl crate::Readable for FIFO_STATUS_SPEC {}
///`reset()` method sets FIFO_STATUS to value 0x01
impl crate::Resettable for FIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
