#[doc = "Register `PRO_PMS_DMA_TX_I_ILG_INTR_MAP` reader"]
pub type R = crate::R<PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC>;
#[doc = "Register `PRO_PMS_DMA_TX_I_ILG_INTR_MAP` writer"]
pub type W = crate::W<PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC>;
#[doc = "Field `PRO_PMS_DMA_TX_I_ILG_INTR_MAP` reader - This register is used to map PMS_DMA_TX_I_ILG interrupt signal to one of the CPU interrupts."]
pub type PRO_PMS_DMA_TX_I_ILG_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PRO_PMS_DMA_TX_I_ILG_INTR_MAP` writer - This register is used to map PMS_DMA_TX_I_ILG interrupt signal to one of the CPU interrupts."]
pub type PRO_PMS_DMA_TX_I_ILG_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - This register is used to map PMS_DMA_TX_I_ILG interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_pms_dma_tx_i_ilg_intr_map(&self) -> PRO_PMS_DMA_TX_I_ILG_INTR_MAP_R {
        PRO_PMS_DMA_TX_I_ILG_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_PMS_DMA_TX_I_ILG_INTR_MAP")
            .field(
                "pro_pms_dma_tx_i_ilg_intr_map",
                &format_args!("{}", self.pro_pms_dma_tx_i_ilg_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to map PMS_DMA_TX_I_ILG interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pro_pms_dma_tx_i_ilg_intr_map(
        &mut self,
    ) -> PRO_PMS_DMA_TX_I_ILG_INTR_MAP_W<PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC> {
        PRO_PMS_DMA_TX_I_ILG_INTR_MAP_W::new(self, 0)
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
#[doc = "PMS_DMA_TX_I_ILG interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_dma_tx_i_ilg_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_dma_tx_i_ilg_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC;
impl crate::RegisterSpec for PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_pms_dma_tx_i_ilg_intr_map::R`](R) reader structure"]
impl crate::Readable for PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_pms_dma_tx_i_ilg_intr_map::W`](W) writer structure"]
impl crate::Writable for PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_PMS_DMA_TX_I_ILG_INTR_MAP to value 0x10"]
impl crate::Resettable for PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
