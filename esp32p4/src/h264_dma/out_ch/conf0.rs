///Register `CONF0` reader
pub type R = crate::R<CONF0_SPEC>;
///Register `CONF0` writer
pub type W = crate::W<CONF0_SPEC>;
///Field `OUT_AUTO_WRBACK` reader - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received.
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
///Field `OUT_AUTO_WRBACK` writer - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received.
pub type OUT_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_EOF_MODE` reader - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA
pub type OUT_EOF_MODE_R = crate::BitReader;
///Field `OUT_EOF_MODE` writer - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA
pub type OUT_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTDSCR_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
///Field `OUTDSCR_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
pub type OUTDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_ECC_AES_EN` reader - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned.
pub type OUT_ECC_AES_EN_R = crate::BitReader;
///Field `OUT_ECC_AES_EN` writer - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned.
pub type OUT_ECC_AES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor.
pub type OUT_CHECK_OWNER_R = crate::BitReader;
///Field `OUT_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor.
pub type OUT_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_MEM_BURST_LENGTH` reader - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes
pub type OUT_MEM_BURST_LENGTH_R = crate::FieldReader;
///Field `OUT_MEM_BURST_LENGTH` writer - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes
pub type OUT_MEM_BURST_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OUT_PAGE_BOUND_EN` reader - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length
pub type OUT_PAGE_BOUND_EN_R = crate::BitReader;
///Field `OUT_PAGE_BOUND_EN` writer - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length
pub type OUT_PAGE_BOUND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_REORDER_EN` reader - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection
pub type OUT_REORDER_EN_R = crate::BitReader;
///Field `OUT_REORDER_EN` writer - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection
pub type OUT_REORDER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_RST` reader - Write 1 then write 0 to this bit to reset TX channel
pub type OUT_RST_R = crate::BitReader;
///Field `OUT_RST` writer - Write 1 then write 0 to this bit to reset TX channel
pub type OUT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_CMD_DISABLE` reader - Write 1 before reset and write 0 after reset
pub type OUT_CMD_DISABLE_R = crate::BitReader;
///Field `OUT_CMD_DISABLE` writer - Write 1 before reset and write 0 after reset
pub type OUT_CMD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_ARB_WEIGHT_OPT_DIS` reader - Set this bit to 1 to disable arbiter optimum weight function.
pub type OUT_ARB_WEIGHT_OPT_DIS_R = crate::BitReader;
///Field `OUT_ARB_WEIGHT_OPT_DIS` writer - Set this bit to 1 to disable arbiter optimum weight function.
pub type OUT_ARB_WEIGHT_OPT_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received.
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned.
    #[inline(always)]
    pub fn out_ecc_aes_en(&self) -> OUT_ECC_AES_EN_R {
        OUT_ECC_AES_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub fn out_check_owner(&self) -> OUT_CHECK_OWNER_R {
        OUT_CHECK_OWNER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 6:8 - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes
    #[inline(always)]
    pub fn out_mem_burst_length(&self) -> OUT_MEM_BURST_LENGTH_R {
        OUT_MEM_BURST_LENGTH_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bit 12 - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length
    #[inline(always)]
    pub fn out_page_bound_en(&self) -> OUT_PAGE_BOUND_EN_R {
        OUT_PAGE_BOUND_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection
    #[inline(always)]
    pub fn out_reorder_en(&self) -> OUT_REORDER_EN_R {
        OUT_REORDER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Write 1 then write 0 to this bit to reset TX channel
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Write 1 before reset and write 0 after reset
    #[inline(always)]
    pub fn out_cmd_disable(&self) -> OUT_CMD_DISABLE_R {
        OUT_CMD_DISABLE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Set this bit to 1 to disable arbiter optimum weight function.
    #[inline(always)]
    pub fn out_arb_weight_opt_dis(&self) -> OUT_ARB_WEIGHT_OPT_DIS_R {
        OUT_ARB_WEIGHT_OPT_DIS_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("out_auto_wrback", &self.out_auto_wrback())
            .field("out_eof_mode", &self.out_eof_mode())
            .field("outdscr_burst_en", &self.outdscr_burst_en())
            .field("out_ecc_aes_en", &self.out_ecc_aes_en())
            .field("out_check_owner", &self.out_check_owner())
            .field("out_mem_burst_length", &self.out_mem_burst_length())
            .field("out_page_bound_en", &self.out_page_bound_en())
            .field("out_reorder_en", &self.out_reorder_en())
            .field("out_rst", &self.out_rst())
            .field("out_cmd_disable", &self.out_cmd_disable())
            .field("out_arb_weight_opt_dis", &self.out_arb_weight_opt_dis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received.
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<CONF0_SPEC> {
        OUT_AUTO_WRBACK_W::new(self, 0)
    }
    ///Bit 1 - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<CONF0_SPEC> {
        OUT_EOF_MODE_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<CONF0_SPEC> {
        OUTDSCR_BURST_EN_W::new(self, 2)
    }
    ///Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned.
    #[inline(always)]
    #[must_use]
    pub fn out_ecc_aes_en(&mut self) -> OUT_ECC_AES_EN_W<CONF0_SPEC> {
        OUT_ECC_AES_EN_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    #[must_use]
    pub fn out_check_owner(&mut self) -> OUT_CHECK_OWNER_W<CONF0_SPEC> {
        OUT_CHECK_OWNER_W::new(self, 4)
    }
    ///Bits 6:8 - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes
    #[inline(always)]
    #[must_use]
    pub fn out_mem_burst_length(&mut self) -> OUT_MEM_BURST_LENGTH_W<CONF0_SPEC> {
        OUT_MEM_BURST_LENGTH_W::new(self, 6)
    }
    ///Bit 12 - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length
    #[inline(always)]
    #[must_use]
    pub fn out_page_bound_en(&mut self) -> OUT_PAGE_BOUND_EN_W<CONF0_SPEC> {
        OUT_PAGE_BOUND_EN_W::new(self, 12)
    }
    ///Bit 16 - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection
    #[inline(always)]
    #[must_use]
    pub fn out_reorder_en(&mut self) -> OUT_REORDER_EN_W<CONF0_SPEC> {
        OUT_REORDER_EN_W::new(self, 16)
    }
    ///Bit 24 - Write 1 then write 0 to this bit to reset TX channel
    #[inline(always)]
    #[must_use]
    pub fn out_rst(&mut self) -> OUT_RST_W<CONF0_SPEC> {
        OUT_RST_W::new(self, 24)
    }
    ///Bit 25 - Write 1 before reset and write 0 after reset
    #[inline(always)]
    #[must_use]
    pub fn out_cmd_disable(&mut self) -> OUT_CMD_DISABLE_W<CONF0_SPEC> {
        OUT_CMD_DISABLE_W::new(self, 25)
    }
    ///Bit 26 - Set this bit to 1 to disable arbiter optimum weight function.
    #[inline(always)]
    #[must_use]
    pub fn out_arb_weight_opt_dis(&mut self) -> OUT_ARB_WEIGHT_OPT_DIS_W<CONF0_SPEC> {
        OUT_ARB_WEIGHT_OPT_DIS_W::new(self, 26)
    }
}
/**TX CHx config0 register

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf0::R`](R) reader structure
impl crate::Readable for CONF0_SPEC {}
///`write(|w| ..)` method takes [`conf0::W`](W) writer structure
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF0 to value 0x02
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
