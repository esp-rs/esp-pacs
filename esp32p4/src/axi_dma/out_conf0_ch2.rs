#[doc = "Register `OUT_CONF0_CH2` reader"]
pub type R = crate::R<OUT_CONF0_CH2_SPEC>;
#[doc = "Register `OUT_CONF0_CH2` writer"]
pub type W = crate::W<OUT_CONF0_CH2_SPEC>;
#[doc = "Field `OUT_RST_CH2` reader - This bit is used to reset AXI_DMA channel2 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH2_R = crate::BitReader;
#[doc = "Field `OUT_RST_CH2` writer - This bit is used to reset AXI_DMA channel2 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST_CH2` reader - reserved"]
pub type OUT_LOOP_TEST_CH2_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST_CH2` writer - reserved"]
pub type OUT_LOOP_TEST_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK_CH2` reader - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH2_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK_CH2` writer - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE_CH2` reader - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel2 is generated when data need to transmit has been popped from FIFO in AXI_DMA"]
pub type OUT_EOF_MODE_CH2_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE_CH2` writer - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel2 is generated when data need to transmit has been popped from FIFO in AXI_DMA"]
pub type OUT_EOF_MODE_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_EN_CH2` reader - Set this bit to 1 to enable etm control mode, dma Tx channel2 is triggered by etm task."]
pub type OUT_ETM_EN_CH2_R = crate::BitReader;
#[doc = "Field `OUT_ETM_EN_CH2` writer - Set this bit to 1 to enable etm control mode, dma Tx channel2 is triggered by etm task."]
pub type OUT_ETM_EN_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_BURST_SIZE_SEL_CH2` reader - 3'b000-3'b100:burst length 8byte~128byte"]
pub type OUT_BURST_SIZE_SEL_CH2_R = crate::FieldReader;
#[doc = "Field `OUT_BURST_SIZE_SEL_CH2` writer - 3'b000-3'b100:burst length 8byte~128byte"]
pub type OUT_BURST_SIZE_SEL_CH2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT_CMD_DISABLE_CH2` reader - 1:mean disable cmd of this ch2"]
pub type OUT_CMD_DISABLE_CH2_R = crate::BitReader;
#[doc = "Field `OUT_CMD_DISABLE_CH2` writer - 1:mean disable cmd of this ch2"]
pub type OUT_CMD_DISABLE_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ECC_AEC_EN_CH2` reader - 1: mean access ecc or aes domain,0: mean not"]
pub type OUT_ECC_AEC_EN_CH2_R = crate::BitReader;
#[doc = "Field `OUT_ECC_AEC_EN_CH2` writer - 1: mean access ecc or aes domain,0: mean not"]
pub type OUT_ECC_AEC_EN_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN_CH2` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel2 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH2_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN_CH2` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel2 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset AXI_DMA channel2 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch2(&self) -> OUT_RST_CH2_R {
        OUT_RST_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch2(&self) -> OUT_LOOP_TEST_CH2_R {
        OUT_LOOP_TEST_CH2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch2(&self) -> OUT_AUTO_WRBACK_CH2_R {
        OUT_AUTO_WRBACK_CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel2 is generated when data need to transmit has been popped from FIFO in AXI_DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch2(&self) -> OUT_EOF_MODE_CH2_R {
        OUT_EOF_MODE_CH2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable etm control mode, dma Tx channel2 is triggered by etm task."]
    #[inline(always)]
    pub fn out_etm_en_ch2(&self) -> OUT_ETM_EN_CH2_R {
        OUT_ETM_EN_CH2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 3'b000-3'b100:burst length 8byte~128byte"]
    #[inline(always)]
    pub fn out_burst_size_sel_ch2(&self) -> OUT_BURST_SIZE_SEL_CH2_R {
        OUT_BURST_SIZE_SEL_CH2_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 1:mean disable cmd of this ch2"]
    #[inline(always)]
    pub fn out_cmd_disable_ch2(&self) -> OUT_CMD_DISABLE_CH2_R {
        OUT_CMD_DISABLE_CH2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: mean access ecc or aes domain,0: mean not"]
    #[inline(always)]
    pub fn out_ecc_aec_en_ch2(&self) -> OUT_ECC_AEC_EN_CH2_R {
        OUT_ECC_AEC_EN_CH2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to 1 to enable INCR burst transfer for Tx channel2 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch2(&self) -> OUTDSCR_BURST_EN_CH2_R {
        OUTDSCR_BURST_EN_CH2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF0_CH2")
            .field("out_rst_ch2", &format_args!("{}", self.out_rst_ch2().bit()))
            .field(
                "out_loop_test_ch2",
                &format_args!("{}", self.out_loop_test_ch2().bit()),
            )
            .field(
                "out_auto_wrback_ch2",
                &format_args!("{}", self.out_auto_wrback_ch2().bit()),
            )
            .field(
                "out_eof_mode_ch2",
                &format_args!("{}", self.out_eof_mode_ch2().bit()),
            )
            .field(
                "out_etm_en_ch2",
                &format_args!("{}", self.out_etm_en_ch2().bit()),
            )
            .field(
                "out_burst_size_sel_ch2",
                &format_args!("{}", self.out_burst_size_sel_ch2().bits()),
            )
            .field(
                "out_cmd_disable_ch2",
                &format_args!("{}", self.out_cmd_disable_ch2().bit()),
            )
            .field(
                "out_ecc_aec_en_ch2",
                &format_args!("{}", self.out_ecc_aec_en_ch2().bit()),
            )
            .field(
                "outdscr_burst_en_ch2",
                &format_args!("{}", self.outdscr_burst_en_ch2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CONF0_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset AXI_DMA channel2 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn out_rst_ch2(&mut self) -> OUT_RST_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUT_RST_CH2_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test_ch2(&mut self) -> OUT_LOOP_TEST_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUT_LOOP_TEST_CH2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback_ch2(&mut self) -> OUT_AUTO_WRBACK_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUT_AUTO_WRBACK_CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel2 is generated when data need to transmit has been popped from FIFO in AXI_DMA"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode_ch2(&mut self) -> OUT_EOF_MODE_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUT_EOF_MODE_CH2_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable etm control mode, dma Tx channel2 is triggered by etm task."]
    #[inline(always)]
    #[must_use]
    pub fn out_etm_en_ch2(&mut self) -> OUT_ETM_EN_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUT_ETM_EN_CH2_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - 3'b000-3'b100:burst length 8byte~128byte"]
    #[inline(always)]
    #[must_use]
    pub fn out_burst_size_sel_ch2(&mut self) -> OUT_BURST_SIZE_SEL_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUT_BURST_SIZE_SEL_CH2_W::new(self, 5)
    }
    #[doc = "Bit 8 - 1:mean disable cmd of this ch2"]
    #[inline(always)]
    #[must_use]
    pub fn out_cmd_disable_ch2(&mut self) -> OUT_CMD_DISABLE_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUT_CMD_DISABLE_CH2_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: mean access ecc or aes domain,0: mean not"]
    #[inline(always)]
    #[must_use]
    pub fn out_ecc_aec_en_ch2(&mut self) -> OUT_ECC_AEC_EN_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUT_ECC_AEC_EN_CH2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to 1 to enable INCR burst transfer for Tx channel2 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en_ch2(&mut self) -> OUTDSCR_BURST_EN_CH2_W<OUT_CONF0_CH2_SPEC> {
        OUTDSCR_BURST_EN_CH2_W::new(self, 10)
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
#[doc = "Configure 0 register of Tx channel2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF0_CH2_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf0_ch2::R`](R) reader structure"]
impl crate::Readable for OUT_CONF0_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf0_ch2::W`](W) writer structure"]
impl crate::Writable for OUT_CONF0_CH2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CONF0_CH2 to value 0x08"]
impl crate::Resettable for OUT_CONF0_CH2_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
