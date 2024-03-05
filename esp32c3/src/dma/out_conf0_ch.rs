#[doc = "Register `OUT_CONF0_CH%s` reader"]
pub type R = crate::R<OUT_CONF0_CH_SPEC>;
#[doc = "Register `OUT_CONF0_CH%s` writer"]
pub type W = crate::W<OUT_CONF0_CH_SPEC>;
#[doc = "Field `OUT_RST` reader - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` writer - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST` reader - reserved"]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - reserved"]
pub type OUT_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE` reader - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DATA_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF0_CH")
            .field("out_rst", &format_args!("{}", self.out_rst().bit()))
            .field(
                "out_loop_test",
                &format_args!("{}", self.out_loop_test().bit()),
            )
            .field(
                "out_auto_wrback",
                &format_args!("{}", self.out_auto_wrback().bit()),
            )
            .field(
                "out_eof_mode",
                &format_args!("{}", self.out_eof_mode().bit()),
            )
            .field(
                "outdscr_burst_en",
                &format_args!("{}", self.outdscr_burst_en().bit()),
            )
            .field(
                "out_data_burst_en",
                &format_args!("{}", self.out_data_burst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CONF0_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn out_rst(&mut self) -> OUT_RST_W<OUT_CONF0_CH_SPEC> {
        OUT_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<OUT_CONF0_CH_SPEC> {
        OUT_LOOP_TEST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<OUT_CONF0_CH_SPEC> {
        OUT_AUTO_WRBACK_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<OUT_CONF0_CH_SPEC> {
        OUT_EOF_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<OUT_CONF0_CH_SPEC> {
        OUTDSCR_BURST_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W<OUT_CONF0_CH_SPEC> {
        OUT_DATA_BURST_EN_W::new(self, 5)
    }
}
#[doc = "DMA_OUT_CONF%s_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF0_CH_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf0_ch::R`](R) reader structure"]
impl crate::Readable for OUT_CONF0_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf0_ch::W`](W) writer structure"]
impl crate::Writable for OUT_CONF0_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CONF0_CH%s to value 0x08"]
impl crate::Resettable for OUT_CONF0_CH_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
