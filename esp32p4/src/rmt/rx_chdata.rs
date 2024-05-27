///Register `RX_CH%sDATA` reader
pub type R = crate::R<RX_CHDATA_SPEC>;
///Field `CHDATA` reader - Read and write data for channel 0 via APB FIFO.
pub type CHDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> CHDATA_R {
        CHDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CHDATA")
            .field("chdata", &self.chdata())
            .finish()
    }
}
/**The read and write data register for CHANNEL$n by apb fifo access.

You can [`read`](crate::generic::Reg::read) this register and get [`rx_chdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_CHDATA_SPEC;
impl crate::RegisterSpec for RX_CHDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_chdata::R`](R) reader structure
impl crate::Readable for RX_CHDATA_SPEC {}
///`reset()` method sets RX_CH%sDATA to value 0
impl crate::Resettable for RX_CHDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
