#[doc = "Register `BACKUP_DMA_CFG1` reader"]
pub type R = crate::R<BACKUP_DMA_CFG1_SPEC>;
#[doc = "Register `BACKUP_DMA_CFG1` writer"]
pub type W = crate::W<BACKUP_DMA_CFG1_SPEC>;
#[doc = "Field `AON_BYPASS` reader - need_des"]
pub type AON_BYPASS_R = crate::BitReader;
#[doc = "Field `AON_BYPASS` writer - need_des"]
pub type AON_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn aon_bypass(&self) -> AON_BYPASS_R {
        AON_BYPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_DMA_CFG1")
            .field("aon_bypass", &format_args!("{}", self.aon_bypass().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_DMA_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn aon_bypass(&mut self) -> AON_BYPASS_W<BACKUP_DMA_CFG1_SPEC> {
        AON_BYPASS_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_dma_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_dma_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_DMA_CFG1_SPEC;
impl crate::RegisterSpec for BACKUP_DMA_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_dma_cfg1::R`](R) reader structure"]
impl crate::Readable for BACKUP_DMA_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_dma_cfg1::W`](W) writer structure"]
impl crate::Writable for BACKUP_DMA_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_DMA_CFG1 to value 0"]
impl crate::Resettable for BACKUP_DMA_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
