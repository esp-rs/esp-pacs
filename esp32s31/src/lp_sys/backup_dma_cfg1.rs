#[doc = "Register `BACKUP_DMA_CFG1` reader"]
pub type R = crate::R<BACKUP_DMA_CFG1_SPEC>;
#[doc = "Register `BACKUP_DMA_CFG1` writer"]
pub type W = crate::W<BACKUP_DMA_CFG1_SPEC>;
#[doc = "Field `LINK_WAIT_TOUT_THRES_AON` reader - "]
pub type LINK_WAIT_TOUT_THRES_AON_R = crate::FieldReader<u16>;
#[doc = "Field `LINK_WAIT_TOUT_THRES_AON` writer - "]
pub type LINK_WAIT_TOUT_THRES_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LINK_WORK_TOUT_THRES_AON` reader - "]
pub type LINK_WORK_TOUT_THRES_AON_R = crate::FieldReader<u16>;
#[doc = "Field `LINK_WORK_TOUT_THRES_AON` writer - "]
pub type LINK_WORK_TOUT_THRES_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BRANCH_LINK_LENGTH_AON` reader - "]
pub type BRANCH_LINK_LENGTH_AON_R = crate::FieldReader;
#[doc = "Field `BRANCH_LINK_LENGTH_AON` writer - "]
pub type BRANCH_LINK_LENGTH_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AON_BYPASS` reader - "]
pub type AON_BYPASS_R = crate::BitReader;
#[doc = "Field `AON_BYPASS` writer - "]
pub type AON_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn link_wait_tout_thres_aon(&self) -> LINK_WAIT_TOUT_THRES_AON_R {
        LINK_WAIT_TOUT_THRES_AON_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn link_work_tout_thres_aon(&self) -> LINK_WORK_TOUT_THRES_AON_R {
        LINK_WORK_TOUT_THRES_AON_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn branch_link_length_aon(&self) -> BRANCH_LINK_LENGTH_AON_R {
        BRANCH_LINK_LENGTH_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn aon_bypass(&self) -> AON_BYPASS_R {
        AON_BYPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_DMA_CFG1")
            .field("link_wait_tout_thres_aon", &self.link_wait_tout_thres_aon())
            .field("link_work_tout_thres_aon", &self.link_work_tout_thres_aon())
            .field("branch_link_length_aon", &self.branch_link_length_aon())
            .field("aon_bypass", &self.aon_bypass())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn link_wait_tout_thres_aon(
        &mut self,
    ) -> LINK_WAIT_TOUT_THRES_AON_W<'_, BACKUP_DMA_CFG1_SPEC> {
        LINK_WAIT_TOUT_THRES_AON_W::new(self, 0)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn link_work_tout_thres_aon(
        &mut self,
    ) -> LINK_WORK_TOUT_THRES_AON_W<'_, BACKUP_DMA_CFG1_SPEC> {
        LINK_WORK_TOUT_THRES_AON_W::new(self, 10)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn branch_link_length_aon(&mut self) -> BRANCH_LINK_LENGTH_AON_W<'_, BACKUP_DMA_CFG1_SPEC> {
        BRANCH_LINK_LENGTH_AON_W::new(self, 20)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn aon_bypass(&mut self) -> AON_BYPASS_W<'_, BACKUP_DMA_CFG1_SPEC> {
        AON_BYPASS_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_DMA_CFG1_SPEC;
impl crate::RegisterSpec for BACKUP_DMA_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_dma_cfg1::R`](R) reader structure"]
impl crate::Readable for BACKUP_DMA_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_dma_cfg1::W`](W) writer structure"]
impl crate::Writable for BACKUP_DMA_CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BACKUP_DMA_CFG1 to value 0x0001_9064"]
impl crate::Resettable for BACKUP_DMA_CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0001_9064;
}
