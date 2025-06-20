#[doc = "Register `BACKUP_DMA_CFG2` reader"]
pub type R = crate::R<BACKUP_DMA_CFG2_SPEC>;
#[doc = "Register `BACKUP_DMA_CFG2` writer"]
pub type W = crate::W<BACKUP_DMA_CFG2_SPEC>;
#[doc = "Field `LINK_ADDR_AON` reader - Set this field to configure link address."]
pub type LINK_ADDR_AON_R = crate::FieldReader<u32>;
#[doc = "Field `LINK_ADDR_AON` writer - Set this field to configure link address."]
pub type LINK_ADDR_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set this field to configure link address."]
    #[inline(always)]
    pub fn link_addr_aon(&self) -> LINK_ADDR_AON_R {
        LINK_ADDR_AON_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_DMA_CFG2")
            .field("link_addr_aon", &self.link_addr_aon())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Set this field to configure link address."]
    #[inline(always)]
    pub fn link_addr_aon(&mut self) -> LINK_ADDR_AON_W<BACKUP_DMA_CFG2_SPEC> {
        LINK_ADDR_AON_W::new(self, 0)
    }
}
#[doc = "configure regdma always on register\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_DMA_CFG2_SPEC;
impl crate::RegisterSpec for BACKUP_DMA_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_dma_cfg2::R`](R) reader structure"]
impl crate::Readable for BACKUP_DMA_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_dma_cfg2::W`](W) writer structure"]
impl crate::Writable for BACKUP_DMA_CFG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACKUP_DMA_CFG2 to value 0"]
impl crate::Resettable for BACKUP_DMA_CFG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
