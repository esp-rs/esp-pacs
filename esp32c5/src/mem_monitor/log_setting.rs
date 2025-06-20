#[doc = "Register `LOG_SETTING` reader"]
pub type R = crate::R<LOG_SETTING_SPEC>;
#[doc = "Register `LOG_SETTING` writer"]
pub type W = crate::W<LOG_SETTING_SPEC>;
#[doc = "Field `LOG_MODE` reader - Configures monitoring modes.bit\\[0\\]: Configures write monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[1\\]: Configures word monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[2\\]: Configures halfword monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[3\\]: Configures byte monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\"]
pub type LOG_MODE_R = crate::FieldReader;
#[doc = "Field `LOG_MODE` writer - Configures monitoring modes.bit\\[0\\]: Configures write monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[1\\]: Configures word monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[2\\]: Configures halfword monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[3\\]: Configures byte monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\"]
pub type LOG_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOG_MEM_LOOP_ENABLE` reader - Configures the writing mode for recorded data.1: Loop mode\\\\ 0: Non-loop mode\\\\"]
pub type LOG_MEM_LOOP_ENABLE_R = crate::BitReader;
#[doc = "Field `LOG_MEM_LOOP_ENABLE` writer - Configures the writing mode for recorded data.1: Loop mode\\\\ 0: Non-loop mode\\\\"]
pub type LOG_MEM_LOOP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOG_CORE_ENA` reader - Configures whether to enable CPU bus access logging.bit\\[0\\]: Configures whether to enable HP CPU bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_CORE_ENA_R = crate::FieldReader;
#[doc = "Field `LOG_CORE_ENA` writer - Configures whether to enable CPU bus access logging.bit\\[0\\]: Configures whether to enable HP CPU bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_CORE_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOG_DMA_0_ENA` reader - Configures whether to enable DMA_0 bus access logging.bit\\[0\\]: Configures whether to enable DMA_0 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_DMA_0_ENA_R = crate::FieldReader;
#[doc = "Field `LOG_DMA_0_ENA` writer - Configures whether to enable DMA_0 bus access logging.bit\\[0\\]: Configures whether to enable DMA_0 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_DMA_0_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOG_DMA_1_ENA` reader - Configures whether to enable DMA_1 bus access logging.bit\\[0\\]: Configures whether to enable DMA_1 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_DMA_1_ENA_R = crate::FieldReader;
#[doc = "Field `LOG_DMA_1_ENA` writer - Configures whether to enable DMA_1 bus access logging.bit\\[0\\]: Configures whether to enable DMA_1 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
pub type LOG_DMA_1_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Configures monitoring modes.bit\\[0\\]: Configures write monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[1\\]: Configures word monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[2\\]: Configures halfword monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[3\\]: Configures byte monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn log_mode(&self) -> LOG_MODE_R {
        LOG_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Configures the writing mode for recorded data.1: Loop mode\\\\ 0: Non-loop mode\\\\"]
    #[inline(always)]
    pub fn log_mem_loop_enable(&self) -> LOG_MEM_LOOP_ENABLE_R {
        LOG_MEM_LOOP_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Configures whether to enable CPU bus access logging.bit\\[0\\]: Configures whether to enable HP CPU bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_core_ena(&self) -> LOG_CORE_ENA_R {
        LOG_CORE_ENA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configures whether to enable DMA_0 bus access logging.bit\\[0\\]: Configures whether to enable DMA_0 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_dma_0_ena(&self) -> LOG_DMA_0_ENA_R {
        LOG_DMA_0_ENA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Configures whether to enable DMA_1 bus access logging.bit\\[0\\]: Configures whether to enable DMA_1 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_dma_1_ena(&self) -> LOG_DMA_1_ENA_R {
        LOG_DMA_1_ENA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_SETTING")
            .field("log_mode", &self.log_mode())
            .field("log_mem_loop_enable", &self.log_mem_loop_enable())
            .field("log_core_ena", &self.log_core_ena())
            .field("log_dma_0_ena", &self.log_dma_0_ena())
            .field("log_dma_1_ena", &self.log_dma_1_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures monitoring modes.bit\\[0\\]: Configures write monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[1\\]: Configures word monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[2\\]: Configures halfword monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\ bit\\[3\\]: Configures byte monitoring. \\\\ 0: Disable \\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn log_mode(&mut self) -> LOG_MODE_W<LOG_SETTING_SPEC> {
        LOG_MODE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Configures the writing mode for recorded data.1: Loop mode\\\\ 0: Non-loop mode\\\\"]
    #[inline(always)]
    pub fn log_mem_loop_enable(&mut self) -> LOG_MEM_LOOP_ENABLE_W<LOG_SETTING_SPEC> {
        LOG_MEM_LOOP_ENABLE_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Configures whether to enable CPU bus access logging.bit\\[0\\]: Configures whether to enable HP CPU bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_core_ena(&mut self) -> LOG_CORE_ENA_W<LOG_SETTING_SPEC> {
        LOG_CORE_ENA_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Configures whether to enable DMA_0 bus access logging.bit\\[0\\]: Configures whether to enable DMA_0 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_dma_0_ena(&mut self) -> LOG_DMA_0_ENA_W<LOG_SETTING_SPEC> {
        LOG_DMA_0_ENA_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Configures whether to enable DMA_1 bus access logging.bit\\[0\\]: Configures whether to enable DMA_1 bus access logging. \\\\ 0: Disable \\\\ 1: Enable\\\\ Bit\\[7:1\\]: Reserved"]
    #[inline(always)]
    pub fn log_dma_1_ena(&mut self) -> LOG_DMA_1_ENA_W<LOG_SETTING_SPEC> {
        LOG_DMA_1_ENA_W::new(self, 24)
    }
}
#[doc = "Bus access logging configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_SETTING_SPEC;
impl crate::RegisterSpec for LOG_SETTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_setting::R`](R) reader structure"]
impl crate::Readable for LOG_SETTING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_setting::W`](W) writer structure"]
impl crate::Writable for LOG_SETTING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOG_SETTING to value 0x10"]
impl crate::Resettable for LOG_SETTING_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
