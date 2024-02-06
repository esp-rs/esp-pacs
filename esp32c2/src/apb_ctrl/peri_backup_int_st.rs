#[doc = "Register `PERI_BACKUP_INT_ST` reader"]
pub type R = crate::R<PERI_BACKUP_INT_ST_SPEC>;
#[doc = "Field `PERI_BACKUP_DONE_INT_ST` reader - reg_peri_backup_done_int_st"]
pub type PERI_BACKUP_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_ERR_INT_ST` reader - reg_peri_backup_err_int_st"]
pub type PERI_BACKUP_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reg_peri_backup_done_int_st"]
    #[inline(always)]
    pub fn peri_backup_done_int_st(&self) -> PERI_BACKUP_DONE_INT_ST_R {
        PERI_BACKUP_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_st"]
    #[inline(always)]
    pub fn peri_backup_err_int_st(&self) -> PERI_BACKUP_ERR_INT_ST_R {
        PERI_BACKUP_ERR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_BACKUP_INT_ST")
            .field(
                "peri_backup_done_int_st",
                &format_args!("{}", self.peri_backup_done_int_st().bit()),
            )
            .field(
                "peri_backup_err_int_st",
                &format_args!("{}", self.peri_backup_err_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_BACKUP_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ST_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_BACKUP_INT_ST_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_backup_int_st::R`](R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_ST_SPEC {}
#[doc = "`reset()` method sets PERI_BACKUP_INT_ST to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
