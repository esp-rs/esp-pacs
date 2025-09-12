#[doc = "Register `LOG_SETTING1` reader"]
pub type R = crate::R<LOG_SETTING1_SPEC>;
#[doc = "Register `LOG_SETTING1` writer"]
pub type W = crate::W<LOG_SETTING1_SPEC>;
#[doc = "Field `LOG_DMA_2_ENA` reader - Configures whether to enable DMA_2 bus access logging.bit\\[0\\]: Configures whether to enable DMA_2 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_DMA_2_ENA_R = crate::FieldReader;
#[doc = "Field `LOG_DMA_2_ENA` writer - Configures whether to enable DMA_2 bus access logging.bit\\[0\\]: Configures whether to enable DMA_2 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_DMA_2_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOG_DMA_3_ENA` reader - Configures whether to enable DMA_3 bus access logging.bit\\[0\\]: Configures whether to enable DMA_3 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_DMA_3_ENA_R = crate::FieldReader;
#[doc = "Field `LOG_DMA_3_ENA` writer - Configures whether to enable DMA_3 bus access logging.bit\\[0\\]: Configures whether to enable DMA_3 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_DMA_3_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures whether to enable DMA_2 bus access logging.bit\\[0\\]: Configures whether to enable DMA_2 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_dma_2_ena(&self) -> LOG_DMA_2_ENA_R {
        LOG_DMA_2_ENA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures whether to enable DMA_3 bus access logging.bit\\[0\\]: Configures whether to enable DMA_3 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_dma_3_ena(&self) -> LOG_DMA_3_ENA_R {
        LOG_DMA_3_ENA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_SETTING1")
            .field("log_dma_2_ena", &self.log_dma_2_ena())
            .field("log_dma_3_ena", &self.log_dma_3_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures whether to enable DMA_2 bus access logging.bit\\[0\\]: Configures whether to enable DMA_2 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_dma_2_ena(&mut self) -> LOG_DMA_2_ENA_W<'_, LOG_SETTING1_SPEC> {
        LOG_DMA_2_ENA_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures whether to enable DMA_3 bus access logging.bit\\[0\\]: Configures whether to enable DMA_3 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_dma_3_ena(&mut self) -> LOG_DMA_3_ENA_W<'_, LOG_SETTING1_SPEC> {
        LOG_DMA_3_ENA_W::new(self, 8)
    }
}
#[doc = "Bus access logging configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_setting1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_setting1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_SETTING1_SPEC;
impl crate::RegisterSpec for LOG_SETTING1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_setting1::R`](R) reader structure"]
impl crate::Readable for LOG_SETTING1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_setting1::W`](W) writer structure"]
impl crate::Writable for LOG_SETTING1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOG_SETTING1 to value 0"]
impl crate::Resettable for LOG_SETTING1_SPEC {}
