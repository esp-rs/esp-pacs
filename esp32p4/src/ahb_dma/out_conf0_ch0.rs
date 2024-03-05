#[doc = "Register `OUT_CONF0_CH0` reader"]
pub type R = crate::R<OUT_CONF0_CH0_SPEC>;
#[doc = "Register `OUT_CONF0_CH0` writer"]
pub type W = crate::W<OUT_CONF0_CH0_SPEC>;
#[doc = "Field `OUT_RST_CH0` reader - This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH0_R = crate::BitReader;
#[doc = "Field `OUT_RST_CH0` writer - This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST_CH0` reader - reserved"]
pub type OUT_LOOP_TEST_CH0_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST_CH0` writer - reserved"]
pub type OUT_LOOP_TEST_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK_CH0` reader - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH0_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK_CH0` writer - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE_CH0` reader - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA"]
pub type OUT_EOF_MODE_CH0_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE_CH0` writer - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA"]
pub type OUT_EOF_MODE_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN_CH0` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH0_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN_CH0` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DATA_BURST_EN_CH0` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_CH0_R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN_CH0` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_EN_CH0` reader - Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task."]
pub type OUT_ETM_EN_CH0_R = crate::BitReader;
#[doc = "Field `OUT_ETM_EN_CH0` writer - Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task."]
pub type OUT_ETM_EN_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch0(&self) -> OUT_RST_CH0_R {
        OUT_RST_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch0(&self) -> OUT_LOOP_TEST_CH0_R {
        OUT_LOOP_TEST_CH0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch0(&self) -> OUT_AUTO_WRBACK_CH0_R {
        OUT_AUTO_WRBACK_CH0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch0(&self) -> OUT_EOF_MODE_CH0_R {
        OUT_EOF_MODE_CH0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch0(&self) -> OUTDSCR_BURST_EN_CH0_R {
        OUTDSCR_BURST_EN_CH0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch0(&self) -> OUT_DATA_BURST_EN_CH0_R {
        OUT_DATA_BURST_EN_CH0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task."]
    #[inline(always)]
    pub fn out_etm_en_ch0(&self) -> OUT_ETM_EN_CH0_R {
        OUT_ETM_EN_CH0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF0_CH0")
            .field("out_rst_ch0", &format_args!("{}", self.out_rst_ch0().bit()))
            .field(
                "out_loop_test_ch0",
                &format_args!("{}", self.out_loop_test_ch0().bit()),
            )
            .field(
                "out_auto_wrback_ch0",
                &format_args!("{}", self.out_auto_wrback_ch0().bit()),
            )
            .field(
                "out_eof_mode_ch0",
                &format_args!("{}", self.out_eof_mode_ch0().bit()),
            )
            .field(
                "outdscr_burst_en_ch0",
                &format_args!("{}", self.outdscr_burst_en_ch0().bit()),
            )
            .field(
                "out_data_burst_en_ch0",
                &format_args!("{}", self.out_data_burst_en_ch0().bit()),
            )
            .field(
                "out_etm_en_ch0",
                &format_args!("{}", self.out_etm_en_ch0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CONF0_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn out_rst_ch0(&mut self) -> OUT_RST_CH0_W<OUT_CONF0_CH0_SPEC> {
        OUT_RST_CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test_ch0(&mut self) -> OUT_LOOP_TEST_CH0_W<OUT_CONF0_CH0_SPEC> {
        OUT_LOOP_TEST_CH0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback_ch0(&mut self) -> OUT_AUTO_WRBACK_CH0_W<OUT_CONF0_CH0_SPEC> {
        OUT_AUTO_WRBACK_CH0_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode_ch0(&mut self) -> OUT_EOF_MODE_CH0_W<OUT_CONF0_CH0_SPEC> {
        OUT_EOF_MODE_CH0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en_ch0(&mut self) -> OUTDSCR_BURST_EN_CH0_W<OUT_CONF0_CH0_SPEC> {
        OUTDSCR_BURST_EN_CH0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn out_data_burst_en_ch0(&mut self) -> OUT_DATA_BURST_EN_CH0_W<OUT_CONF0_CH0_SPEC> {
        OUT_DATA_BURST_EN_CH0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task."]
    #[inline(always)]
    #[must_use]
    pub fn out_etm_en_ch0(&mut self) -> OUT_ETM_EN_CH0_W<OUT_CONF0_CH0_SPEC> {
        OUT_ETM_EN_CH0_W::new(self, 6)
    }
}
#[doc = "Configure 0 register of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF0_CH0_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf0_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_CONF0_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf0_ch0::W`](W) writer structure"]
impl crate::Writable for OUT_CONF0_CH0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CONF0_CH0 to value 0x08"]
impl crate::Resettable for OUT_CONF0_CH0_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
