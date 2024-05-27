///Register `RDATAH` reader
pub type R = crate::R<RDATAH_SPEC>;
///Field `DATA_LSB` reader - NA
pub type DATA_LSB_R = crate::FieldReader;
///Field `DATA_MSB` reader - This register allows reading a Half-word (byte pair) from the bus unless external FIFO is used. A Half-word should not be read unless there is at least 2 bytes of data waiting, as indicated by the RX FIFO level trigger or RXCOUNT available space in the DATACTRL register
pub type DATA_MSB_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - NA
    #[inline(always)]
    pub fn data_lsb(&self) -> DATA_LSB_R {
        DATA_LSB_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - This register allows reading a Half-word (byte pair) from the bus unless external FIFO is used. A Half-word should not be read unless there is at least 2 bytes of data waiting, as indicated by the RX FIFO level trigger or RXCOUNT available space in the DATACTRL register
    #[inline(always)]
    pub fn data_msb(&self) -> DATA_MSB_R {
        DATA_MSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATAH")
            .field("data_lsb", &self.data_lsb())
            .field("data_msb", &self.data_msb())
            .finish()
    }
}
/**Read Half-word Data (from-bus) register

You can [`read`](crate::generic::Reg::read) this register and get [`rdatah::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDATAH_SPEC;
impl crate::RegisterSpec for RDATAH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rdatah::R`](R) reader structure
impl crate::Readable for RDATAH_SPEC {}
///`reset()` method sets RDATAH to value 0
impl crate::Resettable for RDATAH_SPEC {
    const RESET_VALUE: u32 = 0;
}
