#[doc = "Register `BACKUP_DMA_CFG0` reader"]
pub type R = crate::R<BACKUP_DMA_CFG0_SPEC>;
#[doc = "Register `BACKUP_DMA_CFG0` writer"]
pub type W = crate::W<BACKUP_DMA_CFG0_SPEC>;
#[doc = "Field `BURST_LIMIT_AON` reader - Set this field to configure max value of burst in signle transfer."]
pub type BURST_LIMIT_AON_R = crate::FieldReader;
#[doc = "Field `BURST_LIMIT_AON` writer - Set this field to configure max value of burst in signle transfer."]
pub type BURST_LIMIT_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `READ_INTERVAL_AON` reader - Set this field to configure read registers' interval time in reading mode."]
pub type READ_INTERVAL_AON_R = crate::FieldReader;
#[doc = "Field `READ_INTERVAL_AON` writer - Set this field to configure read registers' interval time in reading mode."]
pub type READ_INTERVAL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BRANCH_LINK_LENGTH_AON` reader - Set this field to configure link address."]
pub type BRANCH_LINK_LENGTH_AON_R = crate::FieldReader;
#[doc = "Field `BRANCH_LINK_LENGTH_AON` writer - Set this field to configure link address."]
pub type BRANCH_LINK_LENGTH_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Set this field to configure max value of burst in signle transfer."]
    #[inline(always)]
    pub fn burst_limit_aon(&self) -> BURST_LIMIT_AON_R {
        BURST_LIMIT_AON_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:11 - Set this field to configure read registers' interval time in reading mode."]
    #[inline(always)]
    pub fn read_interval_aon(&self) -> READ_INTERVAL_AON_R {
        READ_INTERVAL_AON_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bits 12:15 - Set this field to configure link address."]
    #[inline(always)]
    pub fn branch_link_length_aon(&self) -> BRANCH_LINK_LENGTH_AON_R {
        BRANCH_LINK_LENGTH_AON_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_DMA_CFG0")
            .field("burst_limit_aon", &self.burst_limit_aon())
            .field("read_interval_aon", &self.read_interval_aon())
            .field("branch_link_length_aon", &self.branch_link_length_aon())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Set this field to configure max value of burst in signle transfer."]
    #[inline(always)]
    pub fn burst_limit_aon(&mut self) -> BURST_LIMIT_AON_W<BACKUP_DMA_CFG0_SPEC> {
        BURST_LIMIT_AON_W::new(self, 0)
    }
    #[doc = "Bits 5:11 - Set this field to configure read registers' interval time in reading mode."]
    #[inline(always)]
    pub fn read_interval_aon(&mut self) -> READ_INTERVAL_AON_W<BACKUP_DMA_CFG0_SPEC> {
        READ_INTERVAL_AON_W::new(self, 5)
    }
    #[doc = "Bits 12:15 - Set this field to configure link address."]
    #[inline(always)]
    pub fn branch_link_length_aon(&mut self) -> BRANCH_LINK_LENGTH_AON_W<BACKUP_DMA_CFG0_SPEC> {
        BRANCH_LINK_LENGTH_AON_W::new(self, 12)
    }
}
#[doc = "configure regdma always on register\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets BACKUP_DMA_CFG0 to value 0x014a"]
impl crate::Resettable for BACKUP_DMA_CFG0_SPEC {
    const RESET_VALUE: u32 = 0x014a;
}
