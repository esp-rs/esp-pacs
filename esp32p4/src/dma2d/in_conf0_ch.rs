#[doc = "Register `IN_CONF0_CH%s` reader"]
pub type R = crate::R<IN_CONF0_CH_SPEC>;
#[doc = "Register `IN_CONF0_CH%s` writer"]
pub type W = crate::W<IN_CONF0_CH_SPEC>;
#[doc = "Field `IN_MEM_TRANS_EN_CH` reader - enable memory trans of the same channel"]
pub type IN_MEM_TRANS_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_MEM_TRANS_EN_CH` writer - enable memory trans of the same channel"]
pub type IN_MEM_TRANS_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Rx transmitting link descriptor when accessing SRAM."]
pub type INDSCR_BURST_EN_CH_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Rx transmitting link descriptor when accessing SRAM."]
pub type INDSCR_BURST_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ECC_AES_EN_CH` reader - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type IN_ECC_AES_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_ECC_AES_EN_CH` writer - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type IN_ECC_AES_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_CHECK_OWNER_CH` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_CH_R = crate::BitReader;
#[doc = "Field `IN_CHECK_OWNER_CH` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST_CH` reader - reserved"]
pub type IN_LOOP_TEST_CH_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST_CH` writer - reserved"]
pub type IN_LOOP_TEST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_MEM_BURST_LENGTH_CH` reader - Block size of Rx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type IN_MEM_BURST_LENGTH_CH_R = crate::FieldReader;
#[doc = "Field `IN_MEM_BURST_LENGTH_CH` writer - Block size of Rx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type IN_MEM_BURST_LENGTH_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_MACRO_BLOCK_SIZE_CH` reader - Sel RX macro-block size 0: 8pixel*8pixel 1: 8pixel*16pixel 2: 16pixel*16pixel 3: no macro-block , only useful in mode 1 of the link descriptor"]
pub type IN_MACRO_BLOCK_SIZE_CH_R = crate::FieldReader;
#[doc = "Field `IN_MACRO_BLOCK_SIZE_CH` writer - Sel RX macro-block size 0: 8pixel*8pixel 1: 8pixel*16pixel 2: 16pixel*16pixel 3: no macro-block , only useful in mode 1 of the link descriptor"]
pub type IN_MACRO_BLOCK_SIZE_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN_DSCR_PORT_EN_CH` reader - Set this bit to 1 to obtain descriptor from IP port"]
pub type IN_DSCR_PORT_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_DSCR_PORT_EN_CH` writer - Set this bit to 1 to obtain descriptor from IP port"]
pub type IN_DSCR_PORT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_PAGE_BOUND_EN_CH` reader - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
pub type IN_PAGE_BOUND_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_PAGE_BOUND_EN_CH` writer - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
pub type IN_PAGE_BOUND_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_REORDER_EN_CH` reader - Enable RX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
pub type IN_REORDER_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_REORDER_EN_CH` writer - Enable RX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
pub type IN_REORDER_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_RST_CH` reader - Write 1 then write 0 to this bit to reset Rx channel"]
pub type IN_RST_CH_R = crate::BitReader;
#[doc = "Field `IN_RST_CH` writer - Write 1 then write 0 to this bit to reset Rx channel"]
pub type IN_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_CMD_DISABLE_CH` reader - Write 1 before reset and write 0 after reset"]
pub type IN_CMD_DISABLE_CH_R = crate::BitReader;
#[doc = "Field `IN_CMD_DISABLE_CH` writer - Write 1 before reset and write 0 after reset"]
pub type IN_CMD_DISABLE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ARB_WEIGHT_OPT_DIS_CH` reader - Set this bit to 1 to disable arbiter optimum weight function."]
pub type IN_ARB_WEIGHT_OPT_DIS_CH_R = crate::BitReader;
#[doc = "Field `IN_ARB_WEIGHT_OPT_DIS_CH` writer - Set this bit to 1 to disable arbiter optimum weight function."]
pub type IN_ARB_WEIGHT_OPT_DIS_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable memory trans of the same channel"]
    #[inline(always)]
    pub fn in_mem_trans_en_ch(&self) -> IN_MEM_TRANS_EN_CH_R {
        IN_MEM_TRANS_EN_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx transmitting link descriptor when accessing SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch(&self) -> INDSCR_BURST_EN_CH_R {
        INDSCR_BURST_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn in_ecc_aes_en_ch(&self) -> IN_ECC_AES_EN_CH_R {
        IN_ECC_AES_EN_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch(&self) -> IN_CHECK_OWNER_CH_R {
        IN_CHECK_OWNER_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch(&self) -> IN_LOOP_TEST_CH_R {
        IN_LOOP_TEST_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Block size of Rx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn in_mem_burst_length_ch(&self) -> IN_MEM_BURST_LENGTH_CH_R {
        IN_MEM_BURST_LENGTH_CH_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - Sel RX macro-block size 0: 8pixel*8pixel 1: 8pixel*16pixel 2: 16pixel*16pixel 3: no macro-block , only useful in mode 1 of the link descriptor"]
    #[inline(always)]
    pub fn in_macro_block_size_ch(&self) -> IN_MACRO_BLOCK_SIZE_CH_R {
        IN_MACRO_BLOCK_SIZE_CH_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Set this bit to 1 to obtain descriptor from IP port"]
    #[inline(always)]
    pub fn in_dscr_port_en_ch(&self) -> IN_DSCR_PORT_EN_CH_R {
        IN_DSCR_PORT_EN_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn in_page_bound_en_ch(&self) -> IN_PAGE_BOUND_EN_CH_R {
        IN_PAGE_BOUND_EN_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable RX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
    #[inline(always)]
    pub fn in_reorder_en_ch(&self) -> IN_REORDER_EN_CH_R {
        IN_REORDER_EN_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset Rx channel"]
    #[inline(always)]
    pub fn in_rst_ch(&self) -> IN_RST_CH_R {
        IN_RST_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    pub fn in_cmd_disable_ch(&self) -> IN_CMD_DISABLE_CH_R {
        IN_CMD_DISABLE_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    pub fn in_arb_weight_opt_dis_ch(&self) -> IN_ARB_WEIGHT_OPT_DIS_CH_R {
        IN_ARB_WEIGHT_OPT_DIS_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF0_CH")
            .field("in_mem_trans_en_ch", &self.in_mem_trans_en_ch())
            .field("indscr_burst_en_ch", &self.indscr_burst_en_ch())
            .field("in_ecc_aes_en_ch", &self.in_ecc_aes_en_ch())
            .field("in_check_owner_ch", &self.in_check_owner_ch())
            .field("in_loop_test_ch", &self.in_loop_test_ch())
            .field("in_mem_burst_length_ch", &self.in_mem_burst_length_ch())
            .field("in_macro_block_size_ch", &self.in_macro_block_size_ch())
            .field("in_dscr_port_en_ch", &self.in_dscr_port_en_ch())
            .field("in_page_bound_en_ch", &self.in_page_bound_en_ch())
            .field("in_reorder_en_ch", &self.in_reorder_en_ch())
            .field("in_rst_ch", &self.in_rst_ch())
            .field("in_cmd_disable_ch", &self.in_cmd_disable_ch())
            .field("in_arb_weight_opt_dis_ch", &self.in_arb_weight_opt_dis_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable memory trans of the same channel"]
    #[inline(always)]
    pub fn in_mem_trans_en_ch(&mut self) -> IN_MEM_TRANS_EN_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_MEM_TRANS_EN_CH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx transmitting link descriptor when accessing SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch(&mut self) -> INDSCR_BURST_EN_CH_W<'_, IN_CONF0_CH_SPEC> {
        INDSCR_BURST_EN_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn in_ecc_aes_en_ch(&mut self) -> IN_ECC_AES_EN_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_ECC_AES_EN_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch(&mut self) -> IN_CHECK_OWNER_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_CHECK_OWNER_CH_W::new(self, 4)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch(&mut self) -> IN_LOOP_TEST_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_LOOP_TEST_CH_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Block size of Rx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn in_mem_burst_length_ch(&mut self) -> IN_MEM_BURST_LENGTH_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_MEM_BURST_LENGTH_CH_W::new(self, 6)
    }
    #[doc = "Bits 9:10 - Sel RX macro-block size 0: 8pixel*8pixel 1: 8pixel*16pixel 2: 16pixel*16pixel 3: no macro-block , only useful in mode 1 of the link descriptor"]
    #[inline(always)]
    pub fn in_macro_block_size_ch(&mut self) -> IN_MACRO_BLOCK_SIZE_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_MACRO_BLOCK_SIZE_CH_W::new(self, 9)
    }
    #[doc = "Bit 11 - Set this bit to 1 to obtain descriptor from IP port"]
    #[inline(always)]
    pub fn in_dscr_port_en_ch(&mut self) -> IN_DSCR_PORT_EN_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_DSCR_PORT_EN_CH_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn in_page_bound_en_ch(&mut self) -> IN_PAGE_BOUND_EN_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_PAGE_BOUND_EN_CH_W::new(self, 12)
    }
    #[doc = "Bit 16 - Enable RX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
    #[inline(always)]
    pub fn in_reorder_en_ch(&mut self) -> IN_REORDER_EN_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_REORDER_EN_CH_W::new(self, 16)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset Rx channel"]
    #[inline(always)]
    pub fn in_rst_ch(&mut self) -> IN_RST_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_RST_CH_W::new(self, 24)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    pub fn in_cmd_disable_ch(&mut self) -> IN_CMD_DISABLE_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_CMD_DISABLE_CH_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    pub fn in_arb_weight_opt_dis_ch(&mut self) -> IN_ARB_WEIGHT_OPT_DIS_CH_W<'_, IN_CONF0_CH_SPEC> {
        IN_ARB_WEIGHT_OPT_DIS_CH_W::new(self, 26)
    }
}
#[doc = "Configures the rx direction of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF0_CH_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf0_ch::R`](R) reader structure"]
impl crate::Readable for IN_CONF0_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf0_ch::W`](W) writer structure"]
impl crate::Writable for IN_CONF0_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CONF0_CH%s to value 0"]
impl crate::Resettable for IN_CONF0_CH_SPEC {}
