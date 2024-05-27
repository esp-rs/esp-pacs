#[doc = "Register `TCM_INT_ST` reader"]
pub type R = crate::R<TCM_INT_ST_SPEC>;
#[doc = "Field `TCM_PARITY_ERR_INT_ST` reader - need_des"]
pub type TCM_PARITY_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tcm_parity_err_int_st(&self) -> TCM_PARITY_ERR_INT_ST_R {
        TCM_PARITY_ERR_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_INT_ST")
            .field("tcm_parity_err_int_st", &self.tcm_parity_err_int_st())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_INT_ST_SPEC;
impl crate::RegisterSpec for TCM_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_int_st::R`](R) reader structure"]
impl crate::Readable for TCM_INT_ST_SPEC {}
#[doc = "`reset()` method sets TCM_INT_ST to value 0"]
impl crate::Resettable for TCM_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
