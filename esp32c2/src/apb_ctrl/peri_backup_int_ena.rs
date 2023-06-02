#[doc = "Register `PERI_BACKUP_INT_ENA` reader"]
pub struct R(crate::R<PERI_BACKUP_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_BACKUP_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_BACKUP_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_BACKUP_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_BACKUP_INT_ENA` writer"]
pub struct W(crate::W<PERI_BACKUP_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_BACKUP_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PERI_BACKUP_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_BACKUP_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_BACKUP_DONE_INT_ENA` reader - reg_peri_backup_done_int_ena"]
pub type PERI_BACKUP_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_DONE_INT_ENA` writer - reg_peri_backup_done_int_ena"]
pub type PERI_BACKUP_DONE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, PERI_BACKUP_INT_ENA_SPEC, O>;
#[doc = "Field `PERI_BACKUP_ERR_INT_ENA` reader - reg_peri_backup_err_int_ena"]
pub type PERI_BACKUP_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_ERR_INT_ENA` writer - reg_peri_backup_err_int_ena"]
pub type PERI_BACKUP_ERR_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, PERI_BACKUP_INT_ENA_SPEC, O>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_peri_backup_done_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_done_int_ena(&mut self) -> PERI_BACKUP_DONE_INT_ENA_W<0> {
        PERI_BACKUP_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_err_int_ena(&mut self) -> PERI_BACKUP_ERR_INT_ENA_W<1> {
        PERI_BACKUP_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_ena](index.html) module"]
pub struct PERI_BACKUP_INT_ENA_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_backup_int_ena::R](R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_backup_int_ena::W](W) writer structure"]
impl crate::Writable for PERI_BACKUP_INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_BACKUP_INT_ENA to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
