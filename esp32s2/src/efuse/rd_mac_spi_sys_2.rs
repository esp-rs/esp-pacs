#[doc = "Register `RD_MAC_SPI_SYS_2` reader"]
pub struct R(crate::R<RD_MAC_SPI_SYS_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MAC_SPI_SYS_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MAC_SPI_SYS_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MAC_SPI_SYS_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_PAD_CONF_1` reader - Stores the first part of SPI_PAD_CONF."]
pub type SPI_PAD_CONF_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the first part of SPI_PAD_CONF."]
    #[inline(always)]
    pub fn spi_pad_conf_1(&self) -> SPI_PAD_CONF_1_R {
        SPI_PAD_CONF_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_2")
            .field(
                "spi_pad_conf_1",
                &format_args!("{}", self.spi_pad_conf_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_MAC_SPI_SYS_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 2 of BLOCK1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_2](index.html) module"]
pub struct RD_MAC_SPI_SYS_2_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mac_spi_sys_2::R](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_2 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
