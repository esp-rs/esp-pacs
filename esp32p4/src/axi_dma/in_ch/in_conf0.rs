#[doc = "Register `IN_CONF0` reader"]
pub type R = crate::R<IN_CONF0_SPEC>;
#[doc = "Register `IN_CONF0` writer"]
pub type W = crate::W<IN_CONF0_SPEC>;
#[doc = "Field `IN_RST` reader - This bit is used to reset AXI_DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - This bit is used to reset AXI_DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST` reader - reserved"]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - reserved"]
pub type IN_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN` reader - Set this bit 1 to enable automatic transmitting data from memory to memory via AXI_DMA."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - Set this bit 1 to enable automatic transmitting data from memory to memory via AXI_DMA."]
pub type MEM_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ETM_EN` reader - Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task."]
pub type IN_ETM_EN_R = crate::BitReader;
#[doc = "Field `IN_ETM_EN` writer - Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task."]
pub type IN_ETM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_BURST_SIZE_SEL` reader - 3'b000-3'b100:burst length 8byte~128byte"]
pub type IN_BURST_SIZE_SEL_R = crate::FieldReader;
#[doc = "Field `IN_BURST_SIZE_SEL` writer - 3'b000-3'b100:burst length 8byte~128byte"]
pub type IN_BURST_SIZE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_CMD_DISABLE` reader - 1:mean disable cmd of this ch0"]
pub type IN_CMD_DISABLE_R = crate::BitReader;
#[doc = "Field `IN_CMD_DISABLE` writer - 1:mean disable cmd of this ch0"]
pub type IN_CMD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ECC_AEC_EN` reader - 1: mean access ecc or aes domain,0: mean not"]
pub type IN_ECC_AEC_EN_R = crate::BitReader;
#[doc = "Field `IN_ECC_AEC_EN` writer - 1: mean access ecc or aes domain,0: mean not"]
pub type IN_ECC_AEC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset AXI_DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit 1 to enable automatic transmitting data from memory to memory via AXI_DMA."]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task."]
    #[inline(always)]
    pub fn in_etm_en(&self) -> IN_ETM_EN_R {
        IN_ETM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 3'b000-3'b100:burst length 8byte~128byte"]
    #[inline(always)]
    pub fn in_burst_size_sel(&self) -> IN_BURST_SIZE_SEL_R {
        IN_BURST_SIZE_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 1:mean disable cmd of this ch0"]
    #[inline(always)]
    pub fn in_cmd_disable(&self) -> IN_CMD_DISABLE_R {
        IN_CMD_DISABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: mean access ecc or aes domain,0: mean not"]
    #[inline(always)]
    pub fn in_ecc_aec_en(&self) -> IN_ECC_AEC_EN_R {
        IN_ECC_AEC_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF0")
            .field("in_rst", &self.in_rst())
            .field("in_loop_test", &self.in_loop_test())
            .field("mem_trans_en", &self.mem_trans_en())
            .field("in_etm_en", &self.in_etm_en())
            .field("in_burst_size_sel", &self.in_burst_size_sel())
            .field("in_cmd_disable", &self.in_cmd_disable())
            .field("in_ecc_aec_en", &self.in_ecc_aec_en())
            .field("indscr_burst_en", &self.indscr_burst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset AXI_DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> IN_RST_W<IN_CONF0_SPEC> {
        IN_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<IN_CONF0_SPEC> {
        IN_LOOP_TEST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit 1 to enable automatic transmitting data from memory to memory via AXI_DMA."]
    #[inline(always)]
    #[must_use]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<IN_CONF0_SPEC> {
        MEM_TRANS_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task."]
    #[inline(always)]
    #[must_use]
    pub fn in_etm_en(&mut self) -> IN_ETM_EN_W<IN_CONF0_SPEC> {
        IN_ETM_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 3'b000-3'b100:burst length 8byte~128byte"]
    #[inline(always)]
    #[must_use]
    pub fn in_burst_size_sel(&mut self) -> IN_BURST_SIZE_SEL_W<IN_CONF0_SPEC> {
        IN_BURST_SIZE_SEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - 1:mean disable cmd of this ch0"]
    #[inline(always)]
    #[must_use]
    pub fn in_cmd_disable(&mut self) -> IN_CMD_DISABLE_W<IN_CONF0_SPEC> {
        IN_CMD_DISABLE_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: mean access ecc or aes domain,0: mean not"]
    #[inline(always)]
    #[must_use]
    pub fn in_ecc_aec_en(&mut self) -> IN_ECC_AEC_EN_W<IN_CONF0_SPEC> {
        IN_ECC_AEC_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<IN_CONF0_SPEC> {
        INDSCR_BURST_EN_W::new(self, 9)
    }
}
#[doc = "Configure 0 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF0_SPEC;
impl crate::RegisterSpec for IN_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf0::R`](R) reader structure"]
impl crate::Readable for IN_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf0::W`](W) writer structure"]
impl crate::Writable for IN_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CONF0 to value 0"]
impl crate::Resettable for IN_CONF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
