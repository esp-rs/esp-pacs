#[doc = "Register `SPI_MEM_DLL_DB_ST0` reader"]
pub type R = crate::R<SPI_MEM_DLL_DB_ST0_SPEC>;
#[doc = "Field `SPI_MEM_DB_FIFO_CNT_H` reader - Debug for DLL FIFO pointer. Use a 64bits shift register to record pointer changes during the debug window. db_fifo_cnt\\[63:32\\]"]
pub type SPI_MEM_DB_FIFO_CNT_H_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Debug for DLL FIFO pointer. Use a 64bits shift register to record pointer changes during the debug window. db_fifo_cnt\\[63:32\\]"]
    #[inline(always)]
    pub fn spi_mem_db_fifo_cnt_h(&self) -> SPI_MEM_DB_FIFO_CNT_H_R {
        SPI_MEM_DB_FIFO_CNT_H_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_DLL_DB_ST0")
            .field("spi_mem_db_fifo_cnt_h", &self.spi_mem_db_fifo_cnt_h())
            .finish()
    }
}
#[doc = "MSPI DLL debug status0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_dll_db_st0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_DLL_DB_ST0_SPEC;
impl crate::RegisterSpec for SPI_MEM_DLL_DB_ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_dll_db_st0::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_DLL_DB_ST0_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_DLL_DB_ST0 to value 0"]
impl crate::Resettable for SPI_MEM_DLL_DB_ST0_SPEC {}
