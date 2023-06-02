#[doc = "Register `VDD_SPI_STATUS` reader"]
pub struct R(crate::R<VDD_SPI_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDD_SPI_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDD_SPI_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDD_SPI_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STABLE_VDD_SPI_PWR_DRV` reader - need_des"]
pub type STABLE_VDD_SPI_PWR_DRV_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn stable_vdd_spi_pwr_drv(&self) -> STABLE_VDD_SPI_PWR_DRV_R {
        STABLE_VDD_SPI_PWR_DRV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDD_SPI_STATUS")
            .field(
                "stable_vdd_spi_pwr_drv",
                &format_args!("{}", self.stable_vdd_spi_pwr_drv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VDD_SPI_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_spi_status](index.html) module"]
pub struct VDD_SPI_STATUS_SPEC;
impl crate::RegisterSpec for VDD_SPI_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdd_spi_status::R](R) reader structure"]
impl crate::Readable for VDD_SPI_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VDD_SPI_STATUS to value 0"]
impl crate::Resettable for VDD_SPI_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
