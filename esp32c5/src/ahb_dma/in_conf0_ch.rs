#[doc = "Register `IN_CONF0_CH%s` reader"]
pub type R = crate::R<IN_CONF0_CH_SPEC>;
#[doc = "Register `IN_CONF0_CH%s` writer"]
pub type W = crate::W<IN_CONF0_CH_SPEC>;
#[doc = "Field `IN_RST_CH` reader - Write 1 and then 0 to reset AHB_DMA channel 0 RX FSM and RX FIFO pointer."]
pub type IN_RST_CH_R = crate::BitReader;
#[doc = "Field `IN_RST_CH` writer - Write 1 and then 0 to reset AHB_DMA channel 0 RX FSM and RX FIFO pointer."]
pub type IN_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST_CH` reader - Reserved."]
pub type IN_LOOP_TEST_CH_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST_CH` writer - Reserved."]
pub type IN_LOOP_TEST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN_CH` reader - Configures whether or not to enable INCR burst transfer for RX channel %s to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type INDSCR_BURST_EN_CH_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN_CH` writer - Configures whether or not to enable INCR burst transfer for RX channel %s to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type INDSCR_BURST_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN_CH` reader - Configures whether or not to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type MEM_TRANS_EN_CH_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN_CH` writer - Configures whether or not to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type MEM_TRANS_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ETM_EN_CH` reader - Configures whether or not to enable ETM control for RX channel%s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type IN_ETM_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_ETM_EN_CH` writer - Configures whether or not to enable ETM control for RX channel%s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type IN_ETM_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DATA_BURST_MODE_SEL_CH` reader - Configures max burst size for Rx channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type IN_DATA_BURST_MODE_SEL_CH_R = crate::FieldReader;
#[doc = "Field `IN_DATA_BURST_MODE_SEL_CH` writer - Configures max burst size for Rx channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type IN_DATA_BURST_MODE_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Write 1 and then 0 to reset AHB_DMA channel 0 RX FSM and RX FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch(&self) -> IN_RST_CH_R {
        IN_RST_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn in_loop_test_ch(&self) -> IN_LOOP_TEST_CH_R {
        IN_LOOP_TEST_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable INCR burst transfer for RX channel %s to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn indscr_burst_en_ch(&self) -> INDSCR_BURST_EN_CH_R {
        INDSCR_BURST_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_trans_en_ch(&self) -> MEM_TRANS_EN_CH_R {
        MEM_TRANS_EN_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ETM control for RX channel%s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn in_etm_en_ch(&self) -> IN_ETM_EN_CH_R {
        IN_ETM_EN_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Configures max burst size for Rx channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn in_data_burst_mode_sel_ch(&self) -> IN_DATA_BURST_MODE_SEL_CH_R {
        IN_DATA_BURST_MODE_SEL_CH_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF0_CH")
            .field("in_rst_ch", &self.in_rst_ch())
            .field("in_loop_test_ch", &self.in_loop_test_ch())
            .field("indscr_burst_en_ch", &self.indscr_burst_en_ch())
            .field("mem_trans_en_ch", &self.mem_trans_en_ch())
            .field("in_etm_en_ch", &self.in_etm_en_ch())
            .field(
                "in_data_burst_mode_sel_ch",
                &self.in_data_burst_mode_sel_ch(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 and then 0 to reset AHB_DMA channel 0 RX FSM and RX FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch(&mut self) -> IN_RST_CH_W<IN_CONF0_CH_SPEC> {
        IN_RST_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn in_loop_test_ch(&mut self) -> IN_LOOP_TEST_CH_W<IN_CONF0_CH_SPEC> {
        IN_LOOP_TEST_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable INCR burst transfer for RX channel %s to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn indscr_burst_en_ch(&mut self) -> INDSCR_BURST_EN_CH_W<IN_CONF0_CH_SPEC> {
        INDSCR_BURST_EN_CH_W::new(self, 2)
    }
    #[doc = "Bit 4 - Configures whether or not to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_trans_en_ch(&mut self) -> MEM_TRANS_EN_CH_W<IN_CONF0_CH_SPEC> {
        MEM_TRANS_EN_CH_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ETM control for RX channel%s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn in_etm_en_ch(&mut self) -> IN_ETM_EN_CH_W<IN_CONF0_CH_SPEC> {
        IN_ETM_EN_CH_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Configures max burst size for Rx channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn in_data_burst_mode_sel_ch(&mut self) -> IN_DATA_BURST_MODE_SEL_CH_W<IN_CONF0_CH_SPEC> {
        IN_DATA_BURST_MODE_SEL_CH_W::new(self, 6)
    }
}
#[doc = "Configuration register 0 of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF0_CH_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf0_ch::R`](R) reader structure"]
impl crate::Readable for IN_CONF0_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf0_ch::W`](W) writer structure"]
impl crate::Writable for IN_CONF0_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CONF0_CH%s to value 0"]
impl crate::Resettable for IN_CONF0_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
