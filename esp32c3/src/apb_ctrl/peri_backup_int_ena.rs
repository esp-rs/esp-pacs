#[doc = "Register `PERI_BACKUP_INT_ENA` reader"]
pub type R = crate::R<PERI_BACKUP_INT_ENA_SPEC>;
#[doc = "Register `PERI_BACKUP_INT_ENA` writer"]
pub type W = crate::W<PERI_BACKUP_INT_ENA_SPEC>;
#[doc = "Field `PERI_BACKUP_DONE_INT_ENA` reader - reg_peri_backup_done_int_ena"]
pub type PERI_BACKUP_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_DONE_INT_ENA` writer - reg_peri_backup_done_int_ena"]
pub type PERI_BACKUP_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERI_BACKUP_ERR_INT_ENA` reader - reg_peri_backup_err_int_ena"]
pub type PERI_BACKUP_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_ERR_INT_ENA` writer - reg_peri_backup_err_int_ena"]
pub type PERI_BACKUP_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_peri_backup_done_int_ena"]
    #[inline(always)]
    pub fn peri_backup_done_int_ena(&self) -> PERI_BACKUP_DONE_INT_ENA_R {
        PERI_BACKUP_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_ena"]
    #[inline(always)]
    pub fn peri_backup_err_int_ena(&self) -> PERI_BACKUP_ERR_INT_ENA_R {
        PERI_BACKUP_ERR_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_BACKUP_INT_ENA")
            .field(
                "peri_backup_done_int_ena",
                &format_args!("{}", self.peri_backup_done_int_ena().bit()),
            )
            .field(
                "peri_backup_err_int_ena",
                &format_args!("{}", self.peri_backup_err_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_BACKUP_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_peri_backup_done_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_done_int_ena(
        &mut self,
    ) -> PERI_BACKUP_DONE_INT_ENA_W<PERI_BACKUP_INT_ENA_SPEC> {
        PERI_BACKUP_DONE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_err_int_ena(
        &mut self,
    ) -> PERI_BACKUP_ERR_INT_ENA_W<PERI_BACKUP_INT_ENA_SPEC> {
        PERI_BACKUP_ERR_INT_ENA_W::new(self, 1)
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_BACKUP_INT_ENA_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_backup_int_ena::R`](R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_backup_int_ena::W`](W) writer structure"]
impl crate::Writable for PERI_BACKUP_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_BACKUP_INT_ENA to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
