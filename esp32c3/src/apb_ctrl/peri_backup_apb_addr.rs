#[doc = "Register `PERI_BACKUP_APB_ADDR` reader"]
pub struct R(crate::R<PERI_BACKUP_APB_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_BACKUP_APB_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_BACKUP_APB_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_BACKUP_APB_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_BACKUP_APB_ADDR` writer"]
pub struct W(crate::W<PERI_BACKUP_APB_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_BACKUP_APB_ADDR_SPEC>;
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
impl From<crate::W<PERI_BACKUP_APB_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_BACKUP_APB_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_APB_START_ADDR` reader - reg_backup_apb_start_addr"]
pub type BACKUP_APB_START_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BACKUP_APB_START_ADDR` writer - reg_backup_apb_start_addr"]
pub type BACKUP_APB_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERI_BACKUP_APB_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - reg_backup_apb_start_addr"]
    #[inline(always)]
    pub fn backup_apb_start_addr(&self) -> BACKUP_APB_START_ADDR_R {
        BACKUP_APB_START_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_backup_apb_start_addr"]
    #[inline(always)]
    pub fn backup_apb_start_addr(&mut self) -> BACKUP_APB_START_ADDR_W<0> {
        BACKUP_APB_START_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_APB_ADDR_REG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_apb_addr](index.html) module"]
pub struct PERI_BACKUP_APB_ADDR_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_APB_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_backup_apb_addr::R](R) reader structure"]
impl crate::Readable for PERI_BACKUP_APB_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_backup_apb_addr::W](W) writer structure"]
impl crate::Writable for PERI_BACKUP_APB_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_BACKUP_APB_ADDR to value 0"]
impl crate::Resettable for PERI_BACKUP_APB_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
