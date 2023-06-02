#[doc = "Register `ECC_ERR_ADDR` reader"]
pub struct R(crate::R<ECC_ERR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_ERR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_ERR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_ERR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC_ERR_ADDR` reader - These bits show the first MSPI ECC error address when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM, including ECC byte error and data error. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
pub type ECC_ERR_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits show the first MSPI ECC error address when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM, including ECC byte error and data error. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
    #[inline(always)]
    pub fn ecc_err_addr(&self) -> ECC_ERR_ADDR_R {
        ECC_ERR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_ERR_ADDR")
            .field(
                "ecc_err_addr",
                &format_args!("{}", self.ecc_err_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECC_ERR_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MSPI ECC error address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_err_addr](index.html) module"]
pub struct ECC_ERR_ADDR_SPEC;
impl crate::RegisterSpec for ECC_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_err_addr::R](R) reader structure"]
impl crate::Readable for ECC_ERR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_ERR_ADDR to value 0"]
impl crate::Resettable for ECC_ERR_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
