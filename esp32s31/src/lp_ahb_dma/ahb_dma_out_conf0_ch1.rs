#[doc = "Register `AHB_DMA_OUT_CONF0_CH1` reader"]
pub type R = crate::R<AHB_DMA_OUT_CONF0_CH1_SPEC>;
#[doc = "Register `AHB_DMA_OUT_CONF0_CH1` writer"]
pub type W = crate::W<AHB_DMA_OUT_CONF0_CH1_SPEC>;
#[doc = "Field `AHB_DMA_OUT_RST_CH` reader - Configures the reset state of AHB_DMA channel %s TX FSM and TX FIFO pointer.\\\\0: Release reset\\\\1: Reset\\\\"]
pub type AHB_DMA_OUT_RST_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_RST_CH` writer - Configures the reset state of AHB_DMA channel %s TX FSM and TX FIFO pointer.\\\\0: Release reset\\\\1: Reset\\\\"]
pub type AHB_DMA_OUT_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_LOOP_TEST_CH` reader - Reserved."]
pub type AHB_DMA_OUT_LOOP_TEST_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_LOOP_TEST_CH` writer - Reserved."]
pub type AHB_DMA_OUT_LOOP_TEST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_AUTO_WRBACK_CH` reader - Configures whether to enable automatic outlink write-back when all the data in TX FIFO has been transmitted.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_OUT_AUTO_WRBACK_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_AUTO_WRBACK_CH` writer - Configures whether to enable automatic outlink write-back when all the data in TX FIFO has been transmitted.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_OUT_AUTO_WRBACK_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_EOF_MODE_CH` reader - Configures when to generate EOF flag.\\\\0: EOF flag for TX channel %s is generated when data to be transmitted has been pushed into FIFO in AHB_DMA.\\\\ 1: EOF flag for TX channel %s is generated when data to be transmitted has been popped from FIFO in AHB_DMA.\\\\"]
pub type AHB_DMA_OUT_EOF_MODE_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_EOF_MODE_CH` writer - Configures when to generate EOF flag.\\\\0: EOF flag for TX channel %s is generated when data to be transmitted has been pushed into FIFO in AHB_DMA.\\\\ 1: EOF flag for TX channel %s is generated when data to be transmitted has been popped from FIFO in AHB_DMA.\\\\"]
pub type AHB_DMA_OUT_EOF_MODE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUTDSCR_BURST_EN_CH` reader - Configures whether to enable INCR burst transfer for TX channel %s reading descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_OUTDSCR_BURST_EN_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUTDSCR_BURST_EN_CH` writer - Configures whether to enable INCR burst transfer for TX channel %s reading descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_OUTDSCR_BURST_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_ETM_EN_CH` reader - Configures whether to enable ETM control for TX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_OUT_ETM_EN_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_ETM_EN_CH` writer - Configures whether to enable ETM control for TX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_OUT_ETM_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_DATA_BURST_MODE_SEL_CH` reader - Configures max burst size for TX channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type AHB_DMA_OUT_DATA_BURST_MODE_SEL_CH_R = crate::FieldReader;
#[doc = "Field `AHB_DMA_OUT_DATA_BURST_MODE_SEL_CH` writer - Configures max burst size for TX channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type AHB_DMA_OUT_DATA_BURST_MODE_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Configures the reset state of AHB_DMA channel %s TX FSM and TX FIFO pointer.\\\\0: Release reset\\\\1: Reset\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_rst_ch(&self) -> AHB_DMA_OUT_RST_CH_R {
        AHB_DMA_OUT_RST_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn ahb_dma_out_loop_test_ch(&self) -> AHB_DMA_OUT_LOOP_TEST_CH_R {
        AHB_DMA_OUT_LOOP_TEST_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether to enable automatic outlink write-back when all the data in TX FIFO has been transmitted.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_auto_wrback_ch(&self) -> AHB_DMA_OUT_AUTO_WRBACK_CH_R {
        AHB_DMA_OUT_AUTO_WRBACK_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures when to generate EOF flag.\\\\0: EOF flag for TX channel %s is generated when data to be transmitted has been pushed into FIFO in AHB_DMA.\\\\ 1: EOF flag for TX channel %s is generated when data to be transmitted has been popped from FIFO in AHB_DMA.\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_eof_mode_ch(&self) -> AHB_DMA_OUT_EOF_MODE_CH_R {
        AHB_DMA_OUT_EOF_MODE_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether to enable INCR burst transfer for TX channel %s reading descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_outdscr_burst_en_ch(&self) -> AHB_DMA_OUTDSCR_BURST_EN_CH_R {
        AHB_DMA_OUTDSCR_BURST_EN_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether to enable ETM control for TX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_etm_en_ch(&self) -> AHB_DMA_OUT_ETM_EN_CH_R {
        AHB_DMA_OUT_ETM_EN_CH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configures max burst size for TX channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_data_burst_mode_sel_ch(&self) -> AHB_DMA_OUT_DATA_BURST_MODE_SEL_CH_R {
        AHB_DMA_OUT_DATA_BURST_MODE_SEL_CH_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_OUT_CONF0_CH1")
            .field("ahb_dma_out_rst_ch", &self.ahb_dma_out_rst_ch())
            .field("ahb_dma_out_loop_test_ch", &self.ahb_dma_out_loop_test_ch())
            .field(
                "ahb_dma_out_auto_wrback_ch",
                &self.ahb_dma_out_auto_wrback_ch(),
            )
            .field("ahb_dma_out_eof_mode_ch", &self.ahb_dma_out_eof_mode_ch())
            .field(
                "ahb_dma_outdscr_burst_en_ch",
                &self.ahb_dma_outdscr_burst_en_ch(),
            )
            .field("ahb_dma_out_etm_en_ch", &self.ahb_dma_out_etm_en_ch())
            .field(
                "ahb_dma_out_data_burst_mode_sel_ch",
                &self.ahb_dma_out_data_burst_mode_sel_ch(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the reset state of AHB_DMA channel %s TX FSM and TX FIFO pointer.\\\\0: Release reset\\\\1: Reset\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_rst_ch(&mut self) -> AHB_DMA_OUT_RST_CH_W<'_, AHB_DMA_OUT_CONF0_CH1_SPEC> {
        AHB_DMA_OUT_RST_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn ahb_dma_out_loop_test_ch(
        &mut self,
    ) -> AHB_DMA_OUT_LOOP_TEST_CH_W<'_, AHB_DMA_OUT_CONF0_CH1_SPEC> {
        AHB_DMA_OUT_LOOP_TEST_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to enable automatic outlink write-back when all the data in TX FIFO has been transmitted.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_auto_wrback_ch(
        &mut self,
    ) -> AHB_DMA_OUT_AUTO_WRBACK_CH_W<'_, AHB_DMA_OUT_CONF0_CH1_SPEC> {
        AHB_DMA_OUT_AUTO_WRBACK_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures when to generate EOF flag.\\\\0: EOF flag for TX channel %s is generated when data to be transmitted has been pushed into FIFO in AHB_DMA.\\\\ 1: EOF flag for TX channel %s is generated when data to be transmitted has been popped from FIFO in AHB_DMA.\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_eof_mode_ch(
        &mut self,
    ) -> AHB_DMA_OUT_EOF_MODE_CH_W<'_, AHB_DMA_OUT_CONF0_CH1_SPEC> {
        AHB_DMA_OUT_EOF_MODE_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether to enable INCR burst transfer for TX channel %s reading descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_outdscr_burst_en_ch(
        &mut self,
    ) -> AHB_DMA_OUTDSCR_BURST_EN_CH_W<'_, AHB_DMA_OUT_CONF0_CH1_SPEC> {
        AHB_DMA_OUTDSCR_BURST_EN_CH_W::new(self, 4)
    }
    #[doc = "Bit 6 - Configures whether to enable ETM control for TX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_etm_en_ch(
        &mut self,
    ) -> AHB_DMA_OUT_ETM_EN_CH_W<'_, AHB_DMA_OUT_CONF0_CH1_SPEC> {
        AHB_DMA_OUT_ETM_EN_CH_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Configures max burst size for TX channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_data_burst_mode_sel_ch(
        &mut self,
    ) -> AHB_DMA_OUT_DATA_BURST_MODE_SEL_CH_W<'_, AHB_DMA_OUT_CONF0_CH1_SPEC> {
        AHB_DMA_OUT_DATA_BURST_MODE_SEL_CH_W::new(self, 8)
    }
}
#[doc = "Configuration register 0 of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_conf0_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_conf0_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_OUT_CONF0_CH1_SPEC;
impl crate::RegisterSpec for AHB_DMA_OUT_CONF0_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_out_conf0_ch1::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_OUT_CONF0_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_out_conf0_ch1::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_OUT_CONF0_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_OUT_CONF0_CH1 to value 0x08"]
impl crate::Resettable for AHB_DMA_OUT_CONF0_CH1_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
