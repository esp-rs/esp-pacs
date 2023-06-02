#[doc = "Register `SPI_MEM_REJECT_ADDR` reader"]
pub struct R(crate::R<SPI_MEM_REJECT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_REJECT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_REJECT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_REJECT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_REJECT_ADDR` reader - reg_spi_mem_reject_addr"]
pub type SPI_MEM_REJECT_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_spi_mem_reject_addr"]
    #[inline(always)]
    pub fn spi_mem_reject_addr(&self) -> SPI_MEM_REJECT_ADDR_R {
        SPI_MEM_REJECT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_REJECT_ADDR")
            .field(
                "spi_mem_reject_addr",
                &format_args!("{}", self.spi_mem_reject_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_REJECT_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "APB_CTRL_SPI_MEM_REJECT_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_reject_addr](index.html) module"]
pub struct SPI_MEM_REJECT_ADDR_SPEC;
impl crate::RegisterSpec for SPI_MEM_REJECT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_reject_addr::R](R) reader structure"]
impl crate::Readable for SPI_MEM_REJECT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_REJECT_ADDR to value 0"]
impl crate::Resettable for SPI_MEM_REJECT_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
