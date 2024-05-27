#[doc = "Register `OUT_CONF0` reader"]
pub type R = crate::R<OUT_CONF0_SPEC>;
#[doc = "Register `OUT_CONF0` writer"]
pub type W = crate::W<OUT_CONF0_SPEC>;
#[doc = "Field `OUT_RST_` reader - This bit is used to reset AXI_DMA channel0 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST__R = crate::BitReader;
#[doc = "Field `OUT_RST_` writer - This bit is used to reset AXI_DMA channel0 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST__W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST` reader - reserved"]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - reserved"]
pub type OUT_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE` reader - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel0 is generated when data need to transmit has been popped from FIFO in AXI_DMA"]
pub type OUT_EOF_MODE_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel0 is generated when data need to transmit has been popped from FIFO in AXI_DMA"]
pub type OUT_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_EN` reader - Set this bit to 1 to enable etm control mode, dma Tx channel0 is triggered by etm task."]
pub type OUT_ETM_EN_R = crate::BitReader;
#[doc = "Field `OUT_ETM_EN` writer - Set this bit to 1 to enable etm control mode, dma Tx channel0 is triggered by etm task."]
pub type OUT_ETM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_BURST_SIZE_SEL` reader - 3'b000-3'b100:burst length 8byte~128byte"]
pub type OUT_BURST_SIZE_SEL_R = crate::FieldReader;
#[doc = "Field `OUT_BURST_SIZE_SEL` writer - 3'b000-3'b100:burst length 8byte~128byte"]
pub type OUT_BURST_SIZE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT_CMD_DISABLE` reader - 1:mean disable cmd of this ch0"]
pub type OUT_CMD_DISABLE_R = crate::BitReader;
#[doc = "Field `OUT_CMD_DISABLE` writer - 1:mean disable cmd of this ch0"]
pub type OUT_CMD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ECC_AEC_EN` reader - 1: mean access ecc or aes domain,0: mean not"]
pub type OUT_ECC_AEC_EN_R = crate::BitReader;
#[doc = "Field `OUT_ECC_AEC_EN` writer - 1: mean access ecc or aes domain,0: mean not"]
pub type OUT_ECC_AEC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset AXI_DMA channel0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_(&self) -> OUT_RST__R {
        OUT_RST__R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel0 is generated when data need to transmit has been popped from FIFO in AXI_DMA"]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable etm control mode, dma Tx channel0 is triggered by etm task."]
    #[inline(always)]
    pub fn out_etm_en(&self) -> OUT_ETM_EN_R {
        OUT_ETM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 3'b000-3'b100:burst length 8byte~128byte"]
    #[inline(always)]
    pub fn out_burst_size_sel(&self) -> OUT_BURST_SIZE_SEL_R {
        OUT_BURST_SIZE_SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 1:mean disable cmd of this ch0"]
    #[inline(always)]
    pub fn out_cmd_disable(&self) -> OUT_CMD_DISABLE_R {
        OUT_CMD_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: mean access ecc or aes domain,0: mean not"]
    #[inline(always)]
    pub fn out_ecc_aec_en(&self) -> OUT_ECC_AEC_EN_R {
        OUT_ECC_AEC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to 1 to enable INCR burst transfer for Tx channel0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF0")
            .field("out_rst_", &self.out_rst_())
            .field("out_loop_test", &self.out_loop_test())
            .field("out_auto_wrback", &self.out_auto_wrback())
            .field("out_eof_mode", &self.out_eof_mode())
            .field("out_etm_en", &self.out_etm_en())
            .field("out_burst_size_sel", &self.out_burst_size_sel())
            .field("out_cmd_disable", &self.out_cmd_disable())
            .field("out_ecc_aec_en", &self.out_ecc_aec_en())
            .field("outdscr_burst_en", &self.outdscr_burst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset AXI_DMA channel0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn out_rst_(&mut self) -> OUT_RST__W<OUT_CONF0_SPEC> {
        OUT_RST__W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<OUT_CONF0_SPEC> {
        OUT_LOOP_TEST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<OUT_CONF0_SPEC> {
        OUT_AUTO_WRBACK_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel0 is generated when data need to transmit has been popped from FIFO in AXI_DMA"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<OUT_CONF0_SPEC> {
        OUT_EOF_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable etm control mode, dma Tx channel0 is triggered by etm task."]
    #[inline(always)]
    #[must_use]
    pub fn out_etm_en(&mut self) -> OUT_ETM_EN_W<OUT_CONF0_SPEC> {
        OUT_ETM_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - 3'b000-3'b100:burst length 8byte~128byte"]
    #[inline(always)]
    #[must_use]
    pub fn out_burst_size_sel(&mut self) -> OUT_BURST_SIZE_SEL_W<OUT_CONF0_SPEC> {
        OUT_BURST_SIZE_SEL_W::new(self, 5)
    }
    #[doc = "Bit 8 - 1:mean disable cmd of this ch0"]
    #[inline(always)]
    #[must_use]
    pub fn out_cmd_disable(&mut self) -> OUT_CMD_DISABLE_W<OUT_CONF0_SPEC> {
        OUT_CMD_DISABLE_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: mean access ecc or aes domain,0: mean not"]
    #[inline(always)]
    #[must_use]
    pub fn out_ecc_aec_en(&mut self) -> OUT_ECC_AEC_EN_W<OUT_CONF0_SPEC> {
        OUT_ECC_AEC_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to 1 to enable INCR burst transfer for Tx channel0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<OUT_CONF0_SPEC> {
        OUTDSCR_BURST_EN_W::new(self, 10)
    }
}
#[doc = "Configure 0 register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF0_SPEC;
impl crate::RegisterSpec for OUT_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf0::R`](R) reader structure"]
impl crate::Readable for OUT_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf0::W`](W) writer structure"]
impl crate::Writable for OUT_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CONF0 to value 0x08"]
impl crate::Resettable for OUT_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
