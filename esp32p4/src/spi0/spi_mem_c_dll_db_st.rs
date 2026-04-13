#[doc = "Register `SPI_MEM_C_DLL_DB_ST%s` reader"]
pub type R = crate::R<SPI_MEM_C_DLL_DB_ST_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_C_DLL_DB_ST")
            .field("val", &self.val())
            .finish()
    }
}
#[doc = "DLL delay buffer status %s\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_c_dll_db_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_C_DLL_DB_ST_SPEC;
impl crate::RegisterSpec for SPI_MEM_C_DLL_DB_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_c_dll_db_st::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_C_DLL_DB_ST_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_C_DLL_DB_ST%s to value 0"]
impl crate::Resettable for SPI_MEM_C_DLL_DB_ST_SPEC {}
