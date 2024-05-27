///Register `RD_MAC_SPI_SYS_5` reader
pub type R = crate::R<RD_MAC_SPI_SYS_5_SPEC>;
///Field `SYS_DATA_PART0_2` reader - Stores the second 32 bits of the zeroth part of system data.
pub type SYS_DATA_PART0_2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the second 32 bits of the zeroth part of system data.
    #[inline(always)]
    pub fn sys_data_part0_2(&self) -> SYS_DATA_PART0_2_R {
        SYS_DATA_PART0_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_5")
            .field("sys_data_part0_2", &self.sys_data_part0_2())
            .finish()
    }
}
/**BLOCK1 data register 5.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_MAC_SPI_SYS_5_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_mac_spi_sys_5::R`](R) reader structure
impl crate::Readable for RD_MAC_SPI_SYS_5_SPEC {}
///`reset()` method sets RD_MAC_SPI_SYS_5 to value 0
impl crate::Resettable for RD_MAC_SPI_SYS_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
