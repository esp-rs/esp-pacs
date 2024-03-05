#[doc = "Register `BACKUP_DMA_CFG0` reader"]
pub type R = crate::R<BACKUP_DMA_CFG0_SPEC>;
#[doc = "Register `BACKUP_DMA_CFG0` writer"]
pub type W = crate::W<BACKUP_DMA_CFG0_SPEC>;
#[doc = "Field `BURST_LIMIT_AON` reader - need_des"]
pub type BURST_LIMIT_AON_R = crate::FieldReader;
#[doc = "Field `BURST_LIMIT_AON` writer - need_des"]
pub type BURST_LIMIT_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `READ_INTERVAL_AON` reader - need_des"]
pub type READ_INTERVAL_AON_R = crate::FieldReader;
#[doc = "Field `READ_INTERVAL_AON` writer - need_des"]
pub type READ_INTERVAL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LINK_BACKUP_TOUT_THRES_AON` reader - need_des"]
pub type LINK_BACKUP_TOUT_THRES_AON_R = crate::FieldReader<u16>;
#[doc = "Field `LINK_BACKUP_TOUT_THRES_AON` writer - need_des"]
pub type LINK_BACKUP_TOUT_THRES_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LINK_TOUT_THRES_AON` reader - need_des"]
pub type LINK_TOUT_THRES_AON_R = crate::FieldReader<u16>;
#[doc = "Field `LINK_TOUT_THRES_AON` writer - need_des"]
pub type LINK_TOUT_THRES_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn burst_limit_aon(&self) -> BURST_LIMIT_AON_R {
        BURST_LIMIT_AON_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:11 - need_des"]
    #[inline(always)]
    pub fn read_interval_aon(&self) -> READ_INTERVAL_AON_R {
        READ_INTERVAL_AON_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn link_backup_tout_thres_aon(&self) -> LINK_BACKUP_TOUT_THRES_AON_R {
        LINK_BACKUP_TOUT_THRES_AON_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn link_tout_thres_aon(&self) -> LINK_TOUT_THRES_AON_R {
        LINK_TOUT_THRES_AON_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_DMA_CFG0")
            .field(
                "burst_limit_aon",
                &format_args!("{}", self.burst_limit_aon().bits()),
            )
            .field(
                "read_interval_aon",
                &format_args!("{}", self.read_interval_aon().bits()),
            )
            .field(
                "link_backup_tout_thres_aon",
                &format_args!("{}", self.link_backup_tout_thres_aon().bits()),
            )
            .field(
                "link_tout_thres_aon",
                &format_args!("{}", self.link_tout_thres_aon().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_DMA_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn burst_limit_aon(&mut self) -> BURST_LIMIT_AON_W<BACKUP_DMA_CFG0_SPEC> {
        BURST_LIMIT_AON_W::new(self, 0)
    }
    #[doc = "Bits 5:11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn read_interval_aon(&mut self) -> READ_INTERVAL_AON_W<BACKUP_DMA_CFG0_SPEC> {
        READ_INTERVAL_AON_W::new(self, 5)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn link_backup_tout_thres_aon(
        &mut self,
    ) -> LINK_BACKUP_TOUT_THRES_AON_W<BACKUP_DMA_CFG0_SPEC> {
        LINK_BACKUP_TOUT_THRES_AON_W::new(self, 12)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn link_tout_thres_aon(&mut self) -> LINK_TOUT_THRES_AON_W<BACKUP_DMA_CFG0_SPEC> {
        LINK_TOUT_THRES_AON_W::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_dma_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_dma_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_DMA_CFG0_SPEC;
impl crate::RegisterSpec for BACKUP_DMA_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_dma_cfg0::R`](R) reader structure"]
impl crate::Readable for BACKUP_DMA_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_dma_cfg0::W`](W) writer structure"]
impl crate::Writable for BACKUP_DMA_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACKUP_DMA_CFG0 to value 0x1906_414a"]
impl crate::Resettable for BACKUP_DMA_CFG0_SPEC {
    const RESET_VALUE: u32 = 0x1906_414a;
}
