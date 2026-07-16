#[doc = "Register `DLL_DB_ST1` reader"]
pub type R = crate::R<DLL_DB_ST1_SPEC>;
#[doc = "Field `DB_FIFO_CNT_L` reader - "]
pub type DB_FIFO_CNT_L_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn db_fifo_cnt_l(&self) -> DB_FIFO_CNT_L_R {
        DB_FIFO_CNT_L_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL_DB_ST1")
            .field("db_fifo_cnt_l", &self.db_fifo_cnt_l())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_db_st1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLL_DB_ST1_SPEC;
impl crate::RegisterSpec for DLL_DB_ST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_db_st1::R`](R) reader structure"]
impl crate::Readable for DLL_DB_ST1_SPEC {}
#[doc = "`reset()` method sets DLL_DB_ST1 to value 0"]
impl crate::Resettable for DLL_DB_ST1_SPEC {}
