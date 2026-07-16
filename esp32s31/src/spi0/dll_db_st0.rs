#[doc = "Register `DLL_DB_ST0` reader"]
pub type R = crate::R<DLL_DB_ST0_SPEC>;
#[doc = "Field `DB_FIFO_CNT_H` reader - Debug for DLL FIFO pointer. Use a 64bits shift register to record pointer changes during the debug window. db_fifo_cnt\\[63:32\\]"]
pub type DB_FIFO_CNT_H_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Debug for DLL FIFO pointer. Use a 64bits shift register to record pointer changes during the debug window. db_fifo_cnt\\[63:32\\]"]
    #[inline(always)]
    pub fn db_fifo_cnt_h(&self) -> DB_FIFO_CNT_H_R {
        DB_FIFO_CNT_H_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL_DB_ST0")
            .field("db_fifo_cnt_h", &self.db_fifo_cnt_h())
            .finish()
    }
}
#[doc = "MSPI DLL debug status0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_db_st0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLL_DB_ST0_SPEC;
impl crate::RegisterSpec for DLL_DB_ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_db_st0::R`](R) reader structure"]
impl crate::Readable for DLL_DB_ST0_SPEC {}
#[doc = "`reset()` method sets DLL_DB_ST0 to value 0"]
impl crate::Resettable for DLL_DB_ST0_SPEC {}
