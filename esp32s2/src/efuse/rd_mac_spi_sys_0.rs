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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_0")
            .field("mac_0", &format_args!("{}", self.mac_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_MAC_SPI_SYS_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
    const RESET_VALUE: Self::Ux = 0;
}
