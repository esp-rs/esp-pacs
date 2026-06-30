#[doc = "Register `SPI_MEM_DLL_DB_ST1` reader"]
pub type R = crate::R<SPI_MEM_DLL_DB_ST1_SPEC>;
#[doc = "Field `SPI_MEM_DB_FIFO_CNT_L` reader - Debug for DLL FIFO pointer. Use a 64bits shift register to record pointer changes during the debug window. db_fifo_cnt\\[31:0\\]"]
pub type SPI_MEM_DB_FIFO_CNT_L_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Debug for DLL FIFO pointer. Use a 64bits shift register to record pointer changes during the debug window. db_fifo_cnt\\[31:0\\]"]
    #[inline(always)]
    pub fn spi_mem_db_fifo_cnt_l(&self) -> SPI_MEM_DB_FIFO_CNT_L_R {
        SPI_MEM_DB_FIFO_CNT_L_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_DLL_DB_ST1")
            .field("spi_mem_db_fifo_cnt_l", &self.spi_mem_db_fifo_cnt_l())
            .finish()
    }
}
#[doc = "MSPI DLL debug status1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_dll_db_st1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_DLL_DB_ST1_SPEC;
impl crate::RegisterSpec for SPI_MEM_DLL_DB_ST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_dll_db_st1::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_DLL_DB_ST1_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_DLL_DB_ST1 to value 0"]
impl crate::Resettable for SPI_MEM_DLL_DB_ST1_SPEC {}
