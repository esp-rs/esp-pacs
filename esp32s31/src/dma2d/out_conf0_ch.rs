#[doc = "Register `OUT_CONF0_CH%s` reader"]
pub type R = crate::R<OUT_CONF0_CH_SPEC>;
#[doc = "Register `OUT_CONF0_CH%s` writer"]
pub type W = crate::W<OUT_CONF0_CH_SPEC>;
#[doc = "Field `OUT_AUTO_WRBACK_CH` reader - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
pub type OUT_AUTO_WRBACK_CH_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK_CH` writer - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
pub type OUT_AUTO_WRBACK_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE_CH` reader - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_CH_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE_CH` writer - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ECC_AES_EN_CH` reader - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type OUT_ECC_AES_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_ECC_AES_EN_CH` writer - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type OUT_ECC_AES_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_CHECK_OWNER_CH` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_CH_R = crate::BitReader;
#[doc = "Field `OUT_CHECK_OWNER_CH` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST_CH` reader - reserved"]
pub type OUT_LOOP_TEST_CH_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST_CH` writer - reserved"]
pub type OUT_LOOP_TEST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_MEM_BURST_LENGTH_CH` reader - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type OUT_MEM_BURST_LENGTH_CH_R = crate::FieldReader;
#[doc = "Field `OUT_MEM_BURST_LENGTH_CH` writer - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type OUT_MEM_BURST_LENGTH_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT_MACRO_BLOCK_SIZE_CH` reader - Sel TX macro-block size 0: 8pixel*8pixel 1: 8pixel*16pixel 2: 16pixel*16pixel 3: no macro-block , only useful in mode 1 of the link descriptor"]
pub type OUT_MACRO_BLOCK_SIZE_CH_R = crate::FieldReader;
#[doc = "Field `OUT_MACRO_BLOCK_SIZE_CH` writer - Sel TX macro-block size 0: 8pixel*8pixel 1: 8pixel*16pixel 2: 16pixel*16pixel 3: no macro-block , only useful in mode 1 of the link descriptor"]
pub type OUT_MACRO_BLOCK_SIZE_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUT_DSCR_PORT_EN_CH` reader - Set this bit to 1 to obtain descriptor from IP port"]
pub type OUT_DSCR_PORT_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_PORT_EN_CH` writer - Set this bit to 1 to obtain descriptor from IP port"]
pub type OUT_DSCR_PORT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_PAGE_BOUND_EN_CH` reader - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
pub type OUT_PAGE_BOUND_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_PAGE_BOUND_EN_CH` writer - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
pub type OUT_PAGE_BOUND_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_REORDER_EN_CH` reader - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
pub type OUT_REORDER_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_REORDER_EN_CH` writer - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
pub type OUT_REORDER_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RST_CH` reader - Write 1 then write 0 to this bit to reset TX channel"]
pub type OUT_RST_CH_R = crate::BitReader;
#[doc = "Field `OUT_RST_CH` writer - Write 1 then write 0 to this bit to reset TX channel"]
pub type OUT_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_CMD_DISABLE_CH` reader - Write 1 before reset and write 0 after reset"]
pub type OUT_CMD_DISABLE_CH_R = crate::BitReader;
#[doc = "Field `OUT_CMD_DISABLE_CH` writer - Write 1 before reset and write 0 after reset"]
pub type OUT_CMD_DISABLE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ARB_WEIGHT_OPT_DIS_CH` reader - Set this bit to 1 to disable arbiter optimum weight function."]
pub type OUT_ARB_WEIGHT_OPT_DIS_CH_R = crate::BitReader;
#[doc = "Field `OUT_ARB_WEIGHT_OPT_DIS_CH` writer - Set this bit to 1 to disable arbiter optimum weight function."]
pub type OUT_ARB_WEIGHT_OPT_DIS_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
    #[inline(always)]
    pub fn out_auto_wrback_ch(&self) -> OUT_AUTO_WRBACK_CH_R {
        OUT_AUTO_WRBACK_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch(&self) -> OUT_EOF_MODE_CH_R {
        OUT_EOF_MODE_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch(&self) -> OUTDSCR_BURST_EN_CH_R {
        OUTDSCR_BURST_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn out_ecc_aes_en_ch(&self) -> OUT_ECC_AES_EN_CH_R {
        OUT_ECC_AES_EN_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch(&self) -> OUT_CHECK_OWNER_CH_R {
        OUT_CHECK_OWNER_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch(&self) -> OUT_LOOP_TEST_CH_R {
        OUT_LOOP_TEST_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn out_mem_burst_length_ch(&self) -> OUT_MEM_BURST_LENGTH_CH_R {
        OUT_MEM_BURST_LENGTH_CH_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - Sel TX macro-block size 0: 8pixel*8pixel 1: 8pixel*16pixel 2: 16pixel*16pixel 3: no macro-block , only useful in mode 1 of the link descriptor"]
    #[inline(always)]
    pub fn out_macro_block_size_ch(&self) -> OUT_MACRO_BLOCK_SIZE_CH_R {
        OUT_MACRO_BLOCK_SIZE_CH_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Set this bit to 1 to obtain descriptor from IP port"]
    #[inline(always)]
    pub fn out_dscr_port_en_ch(&self) -> OUT_DSCR_PORT_EN_CH_R {
        OUT_DSCR_PORT_EN_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn out_page_bound_en_ch(&self) -> OUT_PAGE_BOUND_EN_CH_R {
        OUT_PAGE_BOUND_EN_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
    #[inline(always)]
    pub fn out_reorder_en_ch(&self) -> OUT_REORDER_EN_CH_R {
        OUT_REORDER_EN_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset TX channel"]
    #[inline(always)]
    pub fn out_rst_ch(&self) -> OUT_RST_CH_R {
        OUT_RST_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    pub fn out_cmd_disable_ch(&self) -> OUT_CMD_DISABLE_CH_R {
        OUT_CMD_DISABLE_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    pub fn out_arb_weight_opt_dis_ch(&self) -> OUT_ARB_WEIGHT_OPT_DIS_CH_R {
        OUT_ARB_WEIGHT_OPT_DIS_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF0_CH")
            .field("out_auto_wrback_ch", &self.out_auto_wrback_ch())
            .field("out_eof_mode_ch", &self.out_eof_mode_ch())
            .field("outdscr_burst_en_ch", &self.outdscr_burst_en_ch())
            .field("out_ecc_aes_en_ch", &self.out_ecc_aes_en_ch())
            .field("out_check_owner_ch", &self.out_check_owner_ch())
            .field("out_loop_test_ch", &self.out_loop_test_ch())
            .field("out_mem_burst_length_ch", &self.out_mem_burst_length_ch())
            .field("out_macro_block_size_ch", &self.out_macro_block_size_ch())
            .field("out_dscr_port_en_ch", &self.out_dscr_port_en_ch())
            .field("out_page_bound_en_ch", &self.out_page_bound_en_ch())
            .field("out_reorder_en_ch", &self.out_reorder_en_ch())
            .field("out_rst_ch", &self.out_rst_ch())
            .field("out_cmd_disable_ch", &self.out_cmd_disable_ch())
            .field(
                "out_arb_weight_opt_dis_ch",
                &self.out_arb_weight_opt_dis_ch(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
    #[inline(always)]
    pub fn out_auto_wrback_ch(&mut self) -> OUT_AUTO_WRBACK_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_AUTO_WRBACK_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch(&mut self) -> OUT_EOF_MODE_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_EOF_MODE_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch(&mut self) -> OUTDSCR_BURST_EN_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUTDSCR_BURST_EN_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn out_ecc_aes_en_ch(&mut self) -> OUT_ECC_AES_EN_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_ECC_AES_EN_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch(&mut self) -> OUT_CHECK_OWNER_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_CHECK_OWNER_CH_W::new(self, 4)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch(&mut self) -> OUT_LOOP_TEST_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_LOOP_TEST_CH_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn out_mem_burst_length_ch(&mut self) -> OUT_MEM_BURST_LENGTH_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_MEM_BURST_LENGTH_CH_W::new(self, 6)
    }
    #[doc = "Bits 9:10 - Sel TX macro-block size 0: 8pixel*8pixel 1: 8pixel*16pixel 2: 16pixel*16pixel 3: no macro-block , only useful in mode 1 of the link descriptor"]
    #[inline(always)]
    pub fn out_macro_block_size_ch(&mut self) -> OUT_MACRO_BLOCK_SIZE_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_MACRO_BLOCK_SIZE_CH_W::new(self, 9)
    }
    #[doc = "Bit 11 - Set this bit to 1 to obtain descriptor from IP port"]
    #[inline(always)]
    pub fn out_dscr_port_en_ch(&mut self) -> OUT_DSCR_PORT_EN_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_DSCR_PORT_EN_CH_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn out_page_bound_en_ch(&mut self) -> OUT_PAGE_BOUND_EN_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_PAGE_BOUND_EN_CH_W::new(self, 12)
    }
    #[doc = "Bit 16 - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
    #[inline(always)]
    pub fn out_reorder_en_ch(&mut self) -> OUT_REORDER_EN_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_REORDER_EN_CH_W::new(self, 16)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset TX channel"]
    #[inline(always)]
    pub fn out_rst_ch(&mut self) -> OUT_RST_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_RST_CH_W::new(self, 24)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    pub fn out_cmd_disable_ch(&mut self) -> OUT_CMD_DISABLE_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_CMD_DISABLE_CH_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    pub fn out_arb_weight_opt_dis_ch(
        &mut self,
    ) -> OUT_ARB_WEIGHT_OPT_DIS_CH_W<'_, OUT_CONF0_CH_SPEC> {
        OUT_ARB_WEIGHT_OPT_DIS_CH_W::new(self, 26)
    }
}
#[doc = "Configures the tx direction of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF0_CH_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf0_ch::R`](R) reader structure"]
impl crate::Readable for OUT_CONF0_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf0_ch::W`](W) writer structure"]
impl crate::Writable for OUT_CONF0_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CONF0_CH%s to value 0x02"]
impl crate::Resettable for OUT_CONF0_CH_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
