#[doc = "Register `HP_TCM_PARITY_INT_RECORD` reader"]
pub type R = crate::R<HP_TCM_PARITY_INT_RECORD_SPEC>;
#[doc = "Field `HP_TCM_PARITY_ERR_INT_ADDR` reader - hp tcm_parity_err_addr"]
pub type HP_TCM_PARITY_ERR_INT_ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - hp tcm_parity_err_addr"]
    #[inline(always)]
    pub fn hp_tcm_parity_err_int_addr(&self) -> HP_TCM_PARITY_ERR_INT_ADDR_R {
        HP_TCM_PARITY_ERR_INT_ADDR_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_PARITY_INT_RECORD")
            .field(
                "hp_tcm_parity_err_int_addr",
                &format_args!("{}", self.hp_tcm_parity_err_int_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_TCM_PARITY_INT_RECORD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_parity_int_record::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_PARITY_INT_RECORD_SPEC;
impl crate::RegisterSpec for HP_TCM_PARITY_INT_RECORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_parity_int_record::R`](R) reader structure"]
impl crate::Readable for HP_TCM_PARITY_INT_RECORD_SPEC {}
#[doc = "`reset()` method sets HP_TCM_PARITY_INT_RECORD to value 0"]
impl crate::Resettable for HP_TCM_PARITY_INT_RECORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
