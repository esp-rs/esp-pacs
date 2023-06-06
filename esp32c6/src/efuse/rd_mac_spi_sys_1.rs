#[doc = "Register `RD_MAC_SPI_SYS_1` reader"]
pub struct R(crate::R<RD_MAC_SPI_SYS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MAC_SPI_SYS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MAC_SPI_SYS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MAC_SPI_SYS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAC_1` reader - Stores the high 16 bits of MAC address."]
pub type MAC_1_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_EXT` reader - Stores the extended bits of MAC address."]
pub type MAC_EXT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Stores the high 16 bits of MAC address."]
    #[inline(always)]
    pub fn mac_1(&self) -> MAC_1_R {
        MAC_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Stores the extended bits of MAC address."]
    #[inline(always)]
    pub fn mac_ext(&self) -> MAC_EXT_R {
        MAC_EXT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_1")
            .field("mac_1", &format_args!("{}", self.mac_1().bits()))
            .field("mac_ext", &format_args!("{}", self.mac_ext().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_MAC_SPI_SYS_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "BLOCK1 data register $n.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_1](index.html) module"]
pub struct RD_MAC_SPI_SYS_1_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mac_spi_sys_1::R](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_1 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
