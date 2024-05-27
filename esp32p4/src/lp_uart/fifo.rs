///Register `FIFO` reader
pub type R = crate::R<FIFO_SPEC>;
///Field `RXFIFO_RD_BYTE` reader - UART 0 accesses FIFO via this register.
pub type RXFIFO_RD_BYTE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - UART 0 accesses FIFO via this register.
    #[inline(always)]
    pub fn rxfifo_rd_byte(&self) -> RXFIFO_RD_BYTE_R {
        RXFIFO_RD_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO")
            .field("rxfifo_rd_byte", &self.rxfifo_rd_byte())
            .finish()
    }
}
/**FIFO data register

You can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fifo::R`](R) reader structure
impl crate::Readable for FIFO_SPEC {}
///`reset()` method sets FIFO to value 0
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
