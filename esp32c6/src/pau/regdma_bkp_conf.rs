#[doc = "Register `REGDMA_BKP_CONF` reader"]
pub type R = crate::R<REGDMA_BKP_CONF_SPEC>;
#[doc = "Register `REGDMA_BKP_CONF` writer"]
pub type W = crate::W<REGDMA_BKP_CONF_SPEC>;
#[doc = "Field `READ_INTERVAL` reader - Link read_interval"]
pub type READ_INTERVAL_R = crate::FieldReader;
#[doc = "Field `READ_INTERVAL` writer - Link read_interval"]
pub type READ_INTERVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `LINK_TOUT_THRES` reader - link wait timeout threshold"]
pub type LINK_TOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `LINK_TOUT_THRES` writer - link wait timeout threshold"]
pub type LINK_TOUT_THRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `BURST_LIMIT` reader - burst limit"]
pub type BURST_LIMIT_R = crate::FieldReader;
#[doc = "Field `BURST_LIMIT` writer - burst limit"]
pub type BURST_LIMIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `BACKUP_TOUT_THRES` reader - Backup timeout threshold"]
pub type BACKUP_TOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `BACKUP_TOUT_THRES` writer - Backup timeout threshold"]
pub type BACKUP_TOUT_THRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
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
    pub fn read_interval(&mut self) -> READ_INTERVAL_W<REGDMA_BKP_CONF_SPEC, 0> {
        READ_INTERVAL_W::new(self)
    }
    #[doc = "Bits 7:16 - link wait timeout threshold"]
    #[inline(always)]
    #[must_use]
    pub fn link_tout_thres(&mut self) -> LINK_TOUT_THRES_W<REGDMA_BKP_CONF_SPEC, 7> {
        LINK_TOUT_THRES_W::new(self)
    }
    #[doc = "Bits 17:21 - burst limit"]
    #[inline(always)]
    #[must_use]
    pub fn burst_limit(&mut self) -> BURST_LIMIT_W<REGDMA_BKP_CONF_SPEC, 17> {
        BURST_LIMIT_W::new(self)
    }
    #[doc = "Bits 22:31 - Backup timeout threshold"]
    #[inline(always)]
    #[must_use]
    pub fn backup_tout_thres(&mut self) -> BACKUP_TOUT_THRES_W<REGDMA_BKP_CONF_SPEC, 22> {
        BACKUP_TOUT_THRES_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "backup config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regdma_bkp_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_bkp_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_BKP_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_BKP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_bkp_conf::R`](R) reader structure"]
impl crate::Readable for REGDMA_BKP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_bkp_conf::W`](W) writer structure"]
impl crate::Writable for REGDMA_BKP_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGDMA_BKP_CONF to value 0x7d10_1920"]
impl crate::Resettable for REGDMA_BKP_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x7d10_1920;
}
