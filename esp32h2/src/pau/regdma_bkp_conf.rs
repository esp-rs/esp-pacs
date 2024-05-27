///Register `REGDMA_BKP_CONF` reader
pub type R = crate::R<REGDMA_BKP_CONF_SPEC>;
///Register `REGDMA_BKP_CONF` writer
pub type W = crate::W<REGDMA_BKP_CONF_SPEC>;
///Field `READ_INTERVAL` reader - Link read_interval
pub type READ_INTERVAL_R = crate::FieldReader;
///Field `READ_INTERVAL` writer - Link read_interval
pub type READ_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `LINK_TOUT_THRES` reader - link wait timeout threshold
pub type LINK_TOUT_THRES_R = crate::FieldReader<u16>;
///Field `LINK_TOUT_THRES` writer - link wait timeout threshold
pub type LINK_TOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `BURST_LIMIT` reader - burst limit
pub type BURST_LIMIT_R = crate::FieldReader;
///Field `BURST_LIMIT` writer - burst limit
pub type BURST_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BACKUP_TOUT_THRES` reader - Backup timeout threshold
pub type BACKUP_TOUT_THRES_R = crate::FieldReader<u16>;
///Field `BACKUP_TOUT_THRES` writer - Backup timeout threshold
pub type BACKUP_TOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:6 - Link read_interval
    #[inline(always)]
    pub fn read_interval(&self) -> READ_INTERVAL_R {
        READ_INTERVAL_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:16 - link wait timeout threshold
    #[inline(always)]
    pub fn link_tout_thres(&self) -> LINK_TOUT_THRES_R {
        LINK_TOUT_THRES_R::new(((self.bits >> 7) & 0x03ff) as u16)
    }
    ///Bits 17:21 - burst limit
    #[inline(always)]
    pub fn burst_limit(&self) -> BURST_LIMIT_R {
        BURST_LIMIT_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bits 22:31 - Backup timeout threshold
    #[inline(always)]
    pub fn backup_tout_thres(&self) -> BACKUP_TOUT_THRES_R {
        BACKUP_TOUT_THRES_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_BKP_CONF")
            .field("read_interval", &self.read_interval())
            .field("link_tout_thres", &self.link_tout_thres())
            .field("burst_limit", &self.burst_limit())
            .field("backup_tout_thres", &self.backup_tout_thres())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Link read_interval
    #[inline(always)]
    #[must_use]
    pub fn read_interval(&mut self) -> READ_INTERVAL_W<REGDMA_BKP_CONF_SPEC> {
        READ_INTERVAL_W::new(self, 0)
    }
    ///Bits 7:16 - link wait timeout threshold
    #[inline(always)]
    #[must_use]
    pub fn link_tout_thres(&mut self) -> LINK_TOUT_THRES_W<REGDMA_BKP_CONF_SPEC> {
        LINK_TOUT_THRES_W::new(self, 7)
    }
    ///Bits 17:21 - burst limit
    #[inline(always)]
    #[must_use]
    pub fn burst_limit(&mut self) -> BURST_LIMIT_W<REGDMA_BKP_CONF_SPEC> {
        BURST_LIMIT_W::new(self, 17)
    }
    ///Bits 22:31 - Backup timeout threshold
    #[inline(always)]
    #[must_use]
    pub fn backup_tout_thres(&mut self) -> BACKUP_TOUT_THRES_W<REGDMA_BKP_CONF_SPEC> {
        BACKUP_TOUT_THRES_W::new(self, 22)
    }
}
/**backup config

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_bkp_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_bkp_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGDMA_BKP_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_BKP_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`regdma_bkp_conf::R`](R) reader structure
impl crate::Readable for REGDMA_BKP_CONF_SPEC {}
///`write(|w| ..)` method takes [`regdma_bkp_conf::W`](W) writer structure
impl crate::Writable for REGDMA_BKP_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REGDMA_BKP_CONF to value 0x7d10_1920
impl crate::Resettable for REGDMA_BKP_CONF_SPEC {
    const RESET_VALUE: u32 = 0x7d10_1920;
}
