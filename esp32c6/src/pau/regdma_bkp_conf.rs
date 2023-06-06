#[doc = "Register `REGDMA_BKP_CONF` reader"]
pub struct R(crate::R<REGDMA_BKP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGDMA_BKP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGDMA_BKP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGDMA_BKP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGDMA_BKP_CONF` writer"]
pub struct W(crate::W<REGDMA_BKP_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGDMA_BKP_CONF_SPEC>;
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
impl From<crate::W<REGDMA_BKP_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGDMA_BKP_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_INTERVAL` reader - Link read_interval"]
pub type READ_INTERVAL_R = crate::FieldReader;
#[doc = "Field `READ_INTERVAL` writer - Link read_interval"]
pub type READ_INTERVAL_W<'a, const O: u8> = crate::FieldWriter<'a, REGDMA_BKP_CONF_SPEC, 7, O>;
#[doc = "Field `LINK_TOUT_THRES` reader - link wait timeout threshold"]
pub type LINK_TOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `LINK_TOUT_THRES` writer - link wait timeout threshold"]
pub type LINK_TOUT_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGDMA_BKP_CONF_SPEC, 10, O, u16>;
#[doc = "Field `BURST_LIMIT` reader - burst limit"]
pub type BURST_LIMIT_R = crate::FieldReader;
#[doc = "Field `BURST_LIMIT` writer - burst limit"]
pub type BURST_LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, REGDMA_BKP_CONF_SPEC, 5, O>;
#[doc = "Field `BACKUP_TOUT_THRES` reader - Backup timeout threshold"]
pub type BACKUP_TOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `BACKUP_TOUT_THRES` writer - Backup timeout threshold"]
pub type BACKUP_TOUT_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGDMA_BKP_CONF_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:6 - Link read_interval"]
    #[inline(always)]
    pub fn read_interval(&self) -> READ_INTERVAL_R {
        READ_INTERVAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:16 - link wait timeout threshold"]
    #[inline(always)]
    pub fn link_tout_thres(&self) -> LINK_TOUT_THRES_R {
        LINK_TOUT_THRES_R::new(((self.bits >> 7) & 0x03ff) as u16)
    }
    #[doc = "Bits 17:21 - burst limit"]
    #[inline(always)]
    pub fn burst_limit(&self) -> BURST_LIMIT_R {
        BURST_LIMIT_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:31 - Backup timeout threshold"]
    #[inline(always)]
    pub fn backup_tout_thres(&self) -> BACKUP_TOUT_THRES_R {
        BACKUP_TOUT_THRES_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_BKP_CONF")
            .field(
                "read_interval",
                &format_args!("{}", self.read_interval().bits()),
            )
            .field(
                "link_tout_thres",
                &format_args!("{}", self.link_tout_thres().bits()),
            )
            .field(
                "burst_limit",
                &format_args!("{}", self.burst_limit().bits()),
            )
            .field(
                "backup_tout_thres",
                &format_args!("{}", self.backup_tout_thres().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_BKP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Link read_interval"]
    #[inline(always)]
    #[must_use]
    pub fn read_interval(&mut self) -> READ_INTERVAL_W<0> {
        READ_INTERVAL_W::new(self)
    }
    #[doc = "Bits 7:16 - link wait timeout threshold"]
    #[inline(always)]
    #[must_use]
    pub fn link_tout_thres(&mut self) -> LINK_TOUT_THRES_W<7> {
        LINK_TOUT_THRES_W::new(self)
    }
    #[doc = "Bits 17:21 - burst limit"]
    #[inline(always)]
    #[must_use]
    pub fn burst_limit(&mut self) -> BURST_LIMIT_W<17> {
        BURST_LIMIT_W::new(self)
    }
    #[doc = "Bits 22:31 - Backup timeout threshold"]
    #[inline(always)]
    #[must_use]
    pub fn backup_tout_thres(&mut self) -> BACKUP_TOUT_THRES_W<22> {
        BACKUP_TOUT_THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "backup config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regdma_bkp_conf](index.html) module"]
pub struct REGDMA_BKP_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_BKP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regdma_bkp_conf::R](R) reader structure"]
impl crate::Readable for REGDMA_BKP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regdma_bkp_conf::W](W) writer structure"]
impl crate::Writable for REGDMA_BKP_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGDMA_BKP_CONF to value 0x7d10_1920"]
impl crate::Resettable for REGDMA_BKP_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x7d10_1920;
}
