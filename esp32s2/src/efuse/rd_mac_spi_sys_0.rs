#[doc = "Register `RD_MAC_SPI_SYS_0` reader"]
pub struct R(crate::R<RD_MAC_SPI_SYS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MAC_SPI_SYS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MAC_SPI_SYS_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MAC_SPI_SYS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAC_0` reader - Stores the low 32 bits of MAC address."]
pub type MAC_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the low 32 bits of MAC address."]
    #[inline(always)]
    pub fn mac_0(&self) -> MAC_0_R {
        MAC_0_R::new(self.bits)
    }
}
#[doc = "Register 0 of BLOCK1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_0](index.html) module"]
pub struct RD_MAC_SPI_SYS_0_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mac_spi_sys_0::R](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_0 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
