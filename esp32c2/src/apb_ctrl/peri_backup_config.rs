#[doc = "Register `PERI_BACKUP_CONFIG` reader"]
pub struct R(crate::R<PERI_BACKUP_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_BACKUP_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_BACKUP_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_BACKUP_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_BACKUP_CONFIG` writer"]
pub struct W(crate::W<PERI_BACKUP_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_BACKUP_CONFIG_SPEC>;
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
impl From<crate::W<PERI_BACKUP_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_BACKUP_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_BACKUP_FLOW_ERR` reader - reg_peri_backup_flow_err"]
pub type PERI_BACKUP_FLOW_ERR_R = crate::FieldReader;
#[doc = "Field `PERI_BACKUP_BURST_LIMIT` reader - reg_peri_backup_burst_limit"]
pub type PERI_BACKUP_BURST_LIMIT_R = crate::FieldReader;
#[doc = "Field `PERI_BACKUP_BURST_LIMIT` writer - reg_peri_backup_burst_limit"]
pub type PERI_BACKUP_BURST_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, PERI_BACKUP_CONFIG_SPEC, 5, O>;
#[doc = "Field `PERI_BACKUP_TOUT_THRES` reader - reg_peri_backup_tout_thres"]
pub type PERI_BACKUP_TOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `PERI_BACKUP_TOUT_THRES` writer - reg_peri_backup_tout_thres"]
pub type PERI_BACKUP_TOUT_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, PERI_BACKUP_CONFIG_SPEC, 10, O, u16>;
#[doc = "Field `PERI_BACKUP_SIZE` reader - reg_peri_backup_size"]
pub type PERI_BACKUP_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PERI_BACKUP_SIZE` writer - reg_peri_backup_size"]
pub type PERI_BACKUP_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, PERI_BACKUP_CONFIG_SPEC, 10, O, u16>;
#[doc = "Field `PERI_BACKUP_START` writer - reg_peri_backup_start"]
pub type PERI_BACKUP_START_W<'a, const O: u8> = crate::BitWriter<'a, PERI_BACKUP_CONFIG_SPEC, O>;
#[doc = "Field `PERI_BACKUP_TO_MEM` reader - reg_peri_backup_to_mem"]
pub type PERI_BACKUP_TO_MEM_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_TO_MEM` writer - reg_peri_backup_to_mem"]
pub type PERI_BACKUP_TO_MEM_W<'a, const O: u8> = crate::BitWriter<'a, PERI_BACKUP_CONFIG_SPEC, O>;
#[doc = "Field `PERI_BACKUP_ENA` reader - reg_peri_backup_ena"]
pub type PERI_BACKUP_ENA_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_ENA` writer - reg_peri_backup_ena"]
pub type PERI_BACKUP_ENA_W<'a, const O: u8> = crate::BitWriter<'a, PERI_BACKUP_CONFIG_SPEC, O>;
impl R {
    #[doc = "Bits 1:2 - reg_peri_backup_flow_err"]
    #[inline(always)]
    pub fn peri_backup_flow_err(&self) -> PERI_BACKUP_FLOW_ERR_R {
        PERI_BACKUP_FLOW_ERR_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:8 - reg_peri_backup_burst_limit"]
    #[inline(always)]
    pub fn peri_backup_burst_limit(&self) -> PERI_BACKUP_BURST_LIMIT_R {
        PERI_BACKUP_BURST_LIMIT_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:18 - reg_peri_backup_tout_thres"]
    #[inline(always)]
    pub fn peri_backup_tout_thres(&self) -> PERI_BACKUP_TOUT_THRES_R {
        PERI_BACKUP_TOUT_THRES_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 19:28 - reg_peri_backup_size"]
    #[inline(always)]
    pub fn peri_backup_size(&self) -> PERI_BACKUP_SIZE_R {
        PERI_BACKUP_SIZE_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - reg_peri_backup_to_mem"]
    #[inline(always)]
    pub fn peri_backup_to_mem(&self) -> PERI_BACKUP_TO_MEM_R {
        PERI_BACKUP_TO_MEM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_peri_backup_ena"]
    #[inline(always)]
    pub fn peri_backup_ena(&self) -> PERI_BACKUP_ENA_R {
        PERI_BACKUP_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_BACKUP_CONFIG")
            .field(
                "peri_backup_flow_err",
                &format_args!("{}", self.peri_backup_flow_err().bits()),
            )
            .field(
                "peri_backup_burst_limit",
                &format_args!("{}", self.peri_backup_burst_limit().bits()),
            )
            .field(
                "peri_backup_tout_thres",
                &format_args!("{}", self.peri_backup_tout_thres().bits()),
            )
            .field(
                "peri_backup_size",
                &format_args!("{}", self.peri_backup_size().bits()),
            )
            .field(
                "peri_backup_to_mem",
                &format_args!("{}", self.peri_backup_to_mem().bit()),
            )
            .field(
                "peri_backup_ena",
                &format_args!("{}", self.peri_backup_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_BACKUP_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:8 - reg_peri_backup_burst_limit"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_burst_limit(&mut self) -> PERI_BACKUP_BURST_LIMIT_W<4> {
        PERI_BACKUP_BURST_LIMIT_W::new(self)
    }
    #[doc = "Bits 9:18 - reg_peri_backup_tout_thres"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_tout_thres(&mut self) -> PERI_BACKUP_TOUT_THRES_W<9> {
        PERI_BACKUP_TOUT_THRES_W::new(self)
    }
    #[doc = "Bits 19:28 - reg_peri_backup_size"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_size(&mut self) -> PERI_BACKUP_SIZE_W<19> {
        PERI_BACKUP_SIZE_W::new(self)
    }
    #[doc = "Bit 29 - reg_peri_backup_start"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_start(&mut self) -> PERI_BACKUP_START_W<29> {
        PERI_BACKUP_START_W::new(self)
    }
    #[doc = "Bit 30 - reg_peri_backup_to_mem"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_to_mem(&mut self) -> PERI_BACKUP_TO_MEM_W<30> {
        PERI_BACKUP_TO_MEM_W::new(self)
    }
    #[doc = "Bit 31 - reg_peri_backup_ena"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_ena(&mut self) -> PERI_BACKUP_ENA_W<31> {
        PERI_BACKUP_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_CONFIG_REG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_config](index.html) module"]
pub struct PERI_BACKUP_CONFIG_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_backup_config::R](R) reader structure"]
impl crate::Readable for PERI_BACKUP_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_backup_config::W](W) writer structure"]
impl crate::Writable for PERI_BACKUP_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_BACKUP_CONFIG to value 0x6480"]
impl crate::Resettable for PERI_BACKUP_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6480;
}
