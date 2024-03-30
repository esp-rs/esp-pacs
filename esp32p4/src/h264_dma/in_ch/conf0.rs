#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `INDSCR_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Rx transmitting link descriptor when accessing SRAM."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Rx transmitting link descriptor when accessing SRAM."]
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ECC_AES_EN` reader - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type IN_ECC_AES_EN_R = crate::BitReader;
#[doc = "Field `IN_ECC_AES_EN` writer - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type IN_ECC_AES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `IN_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_MEM_BURST_LENGTH` reader - Block size of Rx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type IN_MEM_BURST_LENGTH_R = crate::FieldReader;
#[doc = "Field `IN_MEM_BURST_LENGTH` writer - Block size of Rx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type IN_MEM_BURST_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_PAGE_BOUND_EN` reader - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
pub type IN_PAGE_BOUND_EN_R = crate::BitReader;
#[doc = "Field `IN_PAGE_BOUND_EN` writer - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
pub type IN_PAGE_BOUND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_RST` reader - Write 1 then write 0 to this bit to reset Rx channel"]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - Write 1 then write 0 to this bit to reset Rx channel"]
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_CMD_DISABLE` reader - Write 1 before reset and write 0 after reset"]
pub type IN_CMD_DISABLE_R = crate::BitReader;
#[doc = "Field `IN_CMD_DISABLE` writer - Write 1 before reset and write 0 after reset"]
pub type IN_CMD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ARB_WEIGHT_OPT_DIS` reader - Set this bit to 1 to disable arbiter optimum weight function."]
pub type IN_ARB_WEIGHT_OPT_DIS_R = crate::BitReader;
#[doc = "Field `IN_ARB_WEIGHT_OPT_DIS` writer - Set this bit to 1 to disable arbiter optimum weight function."]
pub type IN_ARB_WEIGHT_OPT_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx transmitting link descriptor when accessing SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn in_ecc_aes_en(&self) -> IN_ECC_AES_EN_R {
        IN_ECC_AES_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner(&self) -> IN_CHECK_OWNER_R {
        IN_CHECK_OWNER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Block size of Rx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn in_mem_burst_length(&self) -> IN_MEM_BURST_LENGTH_R {
        IN_MEM_BURST_LENGTH_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn in_page_bound_en(&self) -> IN_PAGE_BOUND_EN_R {
        IN_PAGE_BOUND_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset Rx channel"]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    pub fn in_cmd_disable(&self) -> IN_CMD_DISABLE_R {
        IN_CMD_DISABLE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    pub fn in_arb_weight_opt_dis(&self) -> IN_ARB_WEIGHT_OPT_DIS_R {
        IN_ARB_WEIGHT_OPT_DIS_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field(
                "indscr_burst_en",
                &format_args!("{}", self.indscr_burst_en().bit()),
            )
            .field(
                "in_ecc_aes_en",
                &format_args!("{}", self.in_ecc_aes_en().bit()),
            )
            .field(
                "in_check_owner",
                &format_args!("{}", self.in_check_owner().bit()),
            )
            .field(
                "in_mem_burst_length",
                &format_args!("{}", self.in_mem_burst_length().bits()),
            )
            .field(
                "in_page_bound_en",
                &format_args!("{}", self.in_page_bound_en().bit()),
            )
            .field("in_rst", &format_args!("{}", self.in_rst().bit()))
            .field(
                "in_cmd_disable",
                &format_args!("{}", self.in_cmd_disable().bit()),
            )
            .field(
                "in_arb_weight_opt_dis",
                &format_args!("{}", self.in_arb_weight_opt_dis().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx transmitting link descriptor when accessing SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<CONF0_SPEC> {
        INDSCR_BURST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn in_ecc_aes_en(&mut self) -> IN_ECC_AES_EN_W<CONF0_SPEC> {
        IN_ECC_AES_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn in_check_owner(&mut self) -> IN_CHECK_OWNER_W<CONF0_SPEC> {
        IN_CHECK_OWNER_W::new(self, 4)
    }
    #[doc = "Bits 6:8 - Block size of Rx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn in_mem_burst_length(&mut self) -> IN_MEM_BURST_LENGTH_W<CONF0_SPEC> {
        IN_MEM_BURST_LENGTH_W::new(self, 6)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    #[must_use]
    pub fn in_page_bound_en(&mut self) -> IN_PAGE_BOUND_EN_W<CONF0_SPEC> {
        IN_PAGE_BOUND_EN_W::new(self, 12)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset Rx channel"]
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> IN_RST_W<CONF0_SPEC> {
        IN_RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    #[must_use]
    pub fn in_cmd_disable(&mut self) -> IN_CMD_DISABLE_W<CONF0_SPEC> {
        IN_CMD_DISABLE_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    #[must_use]
    pub fn in_arb_weight_opt_dis(&mut self) -> IN_ARB_WEIGHT_OPT_DIS_W<CONF0_SPEC> {
        IN_ARB_WEIGHT_OPT_DIS_W::new(self, 26)
    }
}
#[doc = "RX CHx config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
