///Register `VDD_SPI_STATUS` reader
pub type R = crate::R<VDD_SPI_STATUS_SPEC>;
///Field `STABLE_VDD_SPI_PWR_DRV` reader - need_des
pub type STABLE_VDD_SPI_PWR_DRV_R = crate::BitReader;
impl R {
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn stable_vdd_spi_pwr_drv(&self) -> STABLE_VDD_SPI_PWR_DRV_R {
        STABLE_VDD_SPI_PWR_DRV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDD_SPI_STATUS")
            .field("stable_vdd_spi_pwr_drv", &self.stable_vdd_spi_pwr_drv())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`vdd_spi_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VDD_SPI_STATUS_SPEC;
impl crate::RegisterSpec for VDD_SPI_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vdd_spi_status::R`](R) reader structure
impl crate::Readable for VDD_SPI_STATUS_SPEC {}
///`reset()` method sets VDD_SPI_STATUS to value 0
impl crate::Resettable for VDD_SPI_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
