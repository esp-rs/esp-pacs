#[doc = "Register `MEM_ECC_ERR_ADDR` reader"]
pub type R = crate::R<MEM_ECC_ERR_ADDR_SPEC>;
#[doc = "Field `MEM_ECC_ERR_ADDR` reader - This bits show the first MSPI ECC error address. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
pub type MEM_ECC_ERR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - This bits show the first MSPI ECC error address. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
    #[inline(always)]
    pub fn mem_ecc_err_addr(&self) -> MEM_ECC_ERR_ADDR_R {
        MEM_ECC_ERR_ADDR_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_ECC_ERR_ADDR")
            .field("mem_ecc_err_addr", &self.mem_ecc_err_addr())
            .finish()
    }
}
#[doc = "MSPI ECC error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ecc_err_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_ECC_ERR_ADDR_SPEC;
impl crate::RegisterSpec for MEM_ECC_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ecc_err_addr::R`](R) reader structure"]
impl crate::Readable for MEM_ECC_ERR_ADDR_SPEC {}
#[doc = "`reset()` method sets MEM_ECC_ERR_ADDR to value 0"]
impl crate::Resettable for MEM_ECC_ERR_ADDR_SPEC {}
