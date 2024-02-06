#[doc = "Register `PERI_BACKUP_INT_RAW` reader"]
pub type R = crate::R<PERI_BACKUP_INT_RAW_SPEC>;
#[doc = "Field `PERI_BACKUP_DONE_INT_RAW` reader - reg_peri_backup_done_int_raw"]
pub type PERI_BACKUP_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_ERR_INT_RAW` reader - reg_peri_backup_err_int_raw"]
pub type PERI_BACKUP_ERR_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reg_peri_backup_done_int_raw"]
    #[inline(always)]
    pub fn peri_backup_done_int_raw(&self) -> PERI_BACKUP_DONE_INT_RAW_R {
        PERI_BACKUP_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_raw"]
    #[inline(always)]
    pub fn peri_backup_err_int_raw(&self) -> PERI_BACKUP_ERR_INT_RAW_R {
        PERI_BACKUP_ERR_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_BACKUP_INT_RAW")
            .field(
                "peri_backup_done_int_raw",
                &format_args!("{}", self.peri_backup_done_int_raw().bit()),
            )
            .field(
                "peri_backup_err_int_raw",
                &format_args!("{}", self.peri_backup_err_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_BACKUP_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_RAW_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_BACKUP_INT_RAW_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_backup_int_raw::R`](R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_RAW_SPEC {}
#[doc = "`reset()` method sets PERI_BACKUP_INT_RAW to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
