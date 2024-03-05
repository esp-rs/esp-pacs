#[doc = "Register `OUT_CONF0_CH4` reader"]
pub type R = crate::R<OUT_CONF0_CH4_SPEC>;
#[doc = "Register `OUT_CONF0_CH4` writer"]
pub type W = crate::W<OUT_CONF0_CH4_SPEC>;
#[doc = "Field `OUT_AUTO_WRBACK_CH4` reader - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
pub type OUT_AUTO_WRBACK_CH4_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK_CH4` writer - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
pub type OUT_AUTO_WRBACK_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE_CH4` reader - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_CH4_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE_CH4` writer - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN_CH4` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH4_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN_CH4` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ECC_AES_EN_CH4` reader - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type OUT_ECC_AES_EN_CH4_R = crate::BitReader;
#[doc = "Field `OUT_ECC_AES_EN_CH4` writer - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type OUT_ECC_AES_EN_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_CHECK_OWNER_CH4` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_CH4_R = crate::BitReader;
#[doc = "Field `OUT_CHECK_OWNER_CH4` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_MEM_BURST_LENGTH_CH4` reader - Block size of Tx channel 4. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type OUT_MEM_BURST_LENGTH_CH4_R = crate::FieldReader;
#[doc = "Field `OUT_MEM_BURST_LENGTH_CH4` writer - Block size of Tx channel 4. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type OUT_MEM_BURST_LENGTH_CH4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT_PAGE_BOUND_EN_CH4` reader - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
pub type OUT_PAGE_BOUND_EN_CH4_R = crate::BitReader;
#[doc = "Field `OUT_PAGE_BOUND_EN_CH4` writer - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
pub type OUT_PAGE_BOUND_EN_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ARB_WEIGHT_OPT_DIS_CH4` reader - Set this bit to 1 to disable arbiter optimum weight function."]
pub type OUT_ARB_WEIGHT_OPT_DIS_CH4_R = crate::BitReader;
#[doc = "Field `OUT_ARB_WEIGHT_OPT_DIS_CH4` writer - Set this bit to 1 to disable arbiter optimum weight function."]
pub type OUT_ARB_WEIGHT_OPT_DIS_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
    #[inline(always)]
    pub fn out_auto_wrback_ch4(&self) -> OUT_AUTO_WRBACK_CH4_R {
        OUT_AUTO_WRBACK_CH4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch4(&self) -> OUT_EOF_MODE_CH4_R {
        OUT_EOF_MODE_CH4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch4(&self) -> OUTDSCR_BURST_EN_CH4_R {
        OUTDSCR_BURST_EN_CH4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn out_ecc_aes_en_ch4(&self) -> OUT_ECC_AES_EN_CH4_R {
        OUT_ECC_AES_EN_CH4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch4(&self) -> OUT_CHECK_OWNER_CH4_R {
        OUT_CHECK_OWNER_CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Block size of Tx channel 4. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn out_mem_burst_length_ch4(&self) -> OUT_MEM_BURST_LENGTH_CH4_R {
        OUT_MEM_BURST_LENGTH_CH4_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn out_page_bound_en_ch4(&self) -> OUT_PAGE_BOUND_EN_CH4_R {
        OUT_PAGE_BOUND_EN_CH4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    pub fn out_arb_weight_opt_dis_ch4(&self) -> OUT_ARB_WEIGHT_OPT_DIS_CH4_R {
        OUT_ARB_WEIGHT_OPT_DIS_CH4_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF0_CH4")
            .field(
                "out_auto_wrback_ch4",
                &format_args!("{}", self.out_auto_wrback_ch4().bit()),
            )
            .field(
                "out_eof_mode_ch4",
                &format_args!("{}", self.out_eof_mode_ch4().bit()),
            )
            .field(
                "outdscr_burst_en_ch4",
                &format_args!("{}", self.outdscr_burst_en_ch4().bit()),
            )
            .field(
                "out_ecc_aes_en_ch4",
                &format_args!("{}", self.out_ecc_aes_en_ch4().bit()),
            )
            .field(
                "out_check_owner_ch4",
                &format_args!("{}", self.out_check_owner_ch4().bit()),
            )
            .field(
                "out_mem_burst_length_ch4",
                &format_args!("{}", self.out_mem_burst_length_ch4().bits()),
            )
            .field(
                "out_page_bound_en_ch4",
                &format_args!("{}", self.out_page_bound_en_ch4().bit()),
            )
            .field(
                "out_arb_weight_opt_dis_ch4",
                &format_args!("{}", self.out_arb_weight_opt_dis_ch4().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CONF0_CH4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback_ch4(&mut self) -> OUT_AUTO_WRBACK_CH4_W<OUT_CONF0_CH4_SPEC> {
        OUT_AUTO_WRBACK_CH4_W::new(self, 0)
    }
    #[doc = "Bit 1 - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode_ch4(&mut self) -> OUT_EOF_MODE_CH4_W<OUT_CONF0_CH4_SPEC> {
        OUT_EOF_MODE_CH4_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en_ch4(&mut self) -> OUTDSCR_BURST_EN_CH4_W<OUT_CONF0_CH4_SPEC> {
        OUTDSCR_BURST_EN_CH4_W::new(self, 2)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn out_ecc_aes_en_ch4(&mut self) -> OUT_ECC_AES_EN_CH4_W<OUT_CONF0_CH4_SPEC> {
        OUT_ECC_AES_EN_CH4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn out_check_owner_ch4(&mut self) -> OUT_CHECK_OWNER_CH4_W<OUT_CONF0_CH4_SPEC> {
        OUT_CHECK_OWNER_CH4_W::new(self, 4)
    }
    #[doc = "Bits 6:8 - Block size of Tx channel 4. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn out_mem_burst_length_ch4(&mut self) -> OUT_MEM_BURST_LENGTH_CH4_W<OUT_CONF0_CH4_SPEC> {
        OUT_MEM_BURST_LENGTH_CH4_W::new(self, 6)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    #[must_use]
    pub fn out_page_bound_en_ch4(&mut self) -> OUT_PAGE_BOUND_EN_CH4_W<OUT_CONF0_CH4_SPEC> {
        OUT_PAGE_BOUND_EN_CH4_W::new(self, 12)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    #[must_use]
    pub fn out_arb_weight_opt_dis_ch4(
        &mut self,
    ) -> OUT_ARB_WEIGHT_OPT_DIS_CH4_W<OUT_CONF0_CH4_SPEC> {
        OUT_ARB_WEIGHT_OPT_DIS_CH4_W::new(self, 26)
    }
}
#[doc = "TX CH4 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF0_CH4_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf0_ch4::R`](R) reader structure"]
impl crate::Readable for OUT_CONF0_CH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf0_ch4::W`](W) writer structure"]
impl crate::Writable for OUT_CONF0_CH4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CONF0_CH4 to value 0x02"]
impl crate::Resettable for OUT_CONF0_CH4_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
