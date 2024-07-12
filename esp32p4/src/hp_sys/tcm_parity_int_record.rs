#[doc = "Register `TCM_PARITY_INT_RECORD` reader"]
pub type R = crate::R<TCM_PARITY_INT_RECORD_SPEC>;
#[doc = "Field `TCM_PARITY_ERR_INT_ADDR` reader - hp tcm_parity_err_addr"]
pub type TCM_PARITY_ERR_INT_ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - hp tcm_parity_err_addr"]
    #[inline(always)]
    pub fn tcm_parity_err_int_addr(&self) -> TCM_PARITY_ERR_INT_ADDR_R {
        TCM_PARITY_ERR_INT_ADDR_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_PARITY_INT_RECORD")
            .field("tcm_parity_err_int_addr", &self.tcm_parity_err_int_addr())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_parity_int_record::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_PARITY_INT_RECORD_SPEC;
impl crate::RegisterSpec for TCM_PARITY_INT_RECORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_parity_int_record::R`](R) reader structure"]
impl crate::Readable for TCM_PARITY_INT_RECORD_SPEC {}
#[doc = "`reset()` method sets TCM_PARITY_INT_RECORD to value 0"]
impl crate::Resettable for TCM_PARITY_INT_RECORD_SPEC {
    const RESET_VALUE: u32 = 0;
}
