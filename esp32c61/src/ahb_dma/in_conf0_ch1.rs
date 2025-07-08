#[doc = "Register `IN_CONF0_CH1` reader"]
pub type R = crate::R<IN_CONF0_CH1_SPEC>;
#[doc = "Register `IN_CONF0_CH1` writer"]
pub type W = crate::W<IN_CONF0_CH1_SPEC>;
#[doc = "Field `IN_RST_CH1` reader - Write 1 and then 0 to reset AHB_DMA channel 1 RX FSM and RX FIFO pointer."]
pub type IN_RST_CH1_R = crate::BitReader;
#[doc = "Field `IN_RST_CH1` writer - Write 1 and then 0 to reset AHB_DMA channel 1 RX FSM and RX FIFO pointer."]
pub type IN_RST_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST_CH1` reader - reserved"]
pub type IN_LOOP_TEST_CH1_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST_CH1` writer - reserved"]
pub type IN_LOOP_TEST_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN_CH1` reader - Configures whether to enable INCR burst transfer for RX channel 1 to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type INDSCR_BURST_EN_CH1_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN_CH1` writer - Configures whether to enable INCR burst transfer for RX channel 1 to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type INDSCR_BURST_EN_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN_CH1` reader - Configures whether to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type MEM_TRANS_EN_CH1_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN_CH1` writer - Configures whether to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type MEM_TRANS_EN_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ETM_EN_CH1` reader - Configures whether to enable ETM control for RX channel1.\\\\0: Disable\\\\1: Enable\\\\"]
pub type IN_ETM_EN_CH1_R = crate::BitReader;
#[doc = "Field `IN_ETM_EN_CH1` writer - Configures whether to enable ETM control for RX channel1.\\\\0: Disable\\\\1: Enable\\\\"]
pub type IN_ETM_EN_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DATA_BURST_MODE_SEL_CH1` reader - Configures max burst size for Rx channel1.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type IN_DATA_BURST_MODE_SEL_CH1_R = crate::FieldReader;
#[doc = "Field `IN_DATA_BURST_MODE_SEL_CH1` writer - Configures max burst size for Rx channel1.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type IN_DATA_BURST_MODE_SEL_CH1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Write 1 and then 0 to reset AHB_DMA channel 1 RX FSM and RX FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch1(&self) -> IN_RST_CH1_R {
        IN_RST_CH1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch1(&self) -> IN_LOOP_TEST_CH1_R {
        IN_LOOP_TEST_CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether to enable INCR burst transfer for RX channel 1 to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn indscr_burst_en_ch1(&self) -> INDSCR_BURST_EN_CH1_R {
        INDSCR_BURST_EN_CH1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_trans_en_ch1(&self) -> MEM_TRANS_EN_CH1_R {
        MEM_TRANS_EN_CH1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether to enable ETM control for RX channel1.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn in_etm_en_ch1(&self) -> IN_ETM_EN_CH1_R {
        IN_ETM_EN_CH1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Configures max burst size for Rx channel1.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn in_data_burst_mode_sel_ch1(&self) -> IN_DATA_BURST_MODE_SEL_CH1_R {
        IN_DATA_BURST_MODE_SEL_CH1_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF0_CH1")
            .field("in_rst_ch1", &self.in_rst_ch1())
            .field("in_loop_test_ch1", &self.in_loop_test_ch1())
            .field("indscr_burst_en_ch1", &self.indscr_burst_en_ch1())
            .field("mem_trans_en_ch1", &self.mem_trans_en_ch1())
            .field("in_etm_en_ch1", &self.in_etm_en_ch1())
            .field(
                "in_data_burst_mode_sel_ch1",
                &self.in_data_burst_mode_sel_ch1(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 and then 0 to reset AHB_DMA channel 1 RX FSM and RX FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch1(&mut self) -> IN_RST_CH1_W<IN_CONF0_CH1_SPEC> {
        IN_RST_CH1_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch1(&mut self) -> IN_LOOP_TEST_CH1_W<IN_CONF0_CH1_SPEC> {
        IN_LOOP_TEST_CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to enable INCR burst transfer for RX channel 1 to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn indscr_burst_en_ch1(&mut self) -> INDSCR_BURST_EN_CH1_W<IN_CONF0_CH1_SPEC> {
        INDSCR_BURST_EN_CH1_W::new(self, 2)
    }
    #[doc = "Bit 4 - Configures whether to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_trans_en_ch1(&mut self) -> MEM_TRANS_EN_CH1_W<IN_CONF0_CH1_SPEC> {
        MEM_TRANS_EN_CH1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether to enable ETM control for RX channel1.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn in_etm_en_ch1(&mut self) -> IN_ETM_EN_CH1_W<IN_CONF0_CH1_SPEC> {
        IN_ETM_EN_CH1_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Configures max burst size for Rx channel1.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn in_data_burst_mode_sel_ch1(
        &mut self,
    ) -> IN_DATA_BURST_MODE_SEL_CH1_W<IN_CONF0_CH1_SPEC> {
        IN_DATA_BURST_MODE_SEL_CH1_W::new(self, 6)
    }
}
#[doc = "Configuration register 0 of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF0_CH1_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf0_ch1::R`](R) reader structure"]
impl crate::Readable for IN_CONF0_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf0_ch1::W`](W) writer structure"]
impl crate::Writable for IN_CONF0_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CONF0_CH1 to value 0"]
impl crate::Resettable for IN_CONF0_CH1_SPEC {}
