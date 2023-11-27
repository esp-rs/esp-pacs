#[doc = "Register `IN_CONF0_CH5` reader"]
pub type R = crate::R<IN_CONF0_CH5_SPEC>;
#[doc = "Register `IN_CONF0_CH5` writer"]
pub type W = crate::W<IN_CONF0_CH5_SPEC>;
#[doc = "Field `IN_ECC_AES_EN_CH5` reader - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type IN_ECC_AES_EN_CH5_R = crate::BitReader;
#[doc = "Field `IN_ECC_AES_EN_CH5` writer - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type IN_ECC_AES_EN_CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_MEM_BURST_LENGTH_CH5` reader - Block size of Rx channel 1. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type IN_MEM_BURST_LENGTH_CH5_R = crate::FieldReader;
#[doc = "Field `IN_MEM_BURST_LENGTH_CH5` writer - Block size of Rx channel 1. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type IN_MEM_BURST_LENGTH_CH5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_PAGE_BOUND_EN_CH5` reader - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
pub type IN_PAGE_BOUND_EN_CH5_R = crate::BitReader;
#[doc = "Field `IN_PAGE_BOUND_EN_CH5` writer - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
pub type IN_PAGE_BOUND_EN_CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_RST_CH5` reader - Write 1 then write 0 to this bit to reset Rx channel"]
pub type IN_RST_CH5_R = crate::BitReader;
#[doc = "Field `IN_RST_CH5` writer - Write 1 then write 0 to this bit to reset Rx channel"]
pub type IN_RST_CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_CMD_DISABLE_CH5` reader - Write 1 before reset and write 0 after reset"]
pub type IN_CMD_DISABLE_CH5_R = crate::BitReader;
#[doc = "Field `IN_CMD_DISABLE_CH5` writer - Write 1 before reset and write 0 after reset"]
pub type IN_CMD_DISABLE_CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn in_ecc_aes_en_ch5(&self) -> IN_ECC_AES_EN_CH5_R {
        IN_ECC_AES_EN_CH5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Block size of Rx channel 1. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn in_mem_burst_length_ch5(&self) -> IN_MEM_BURST_LENGTH_CH5_R {
        IN_MEM_BURST_LENGTH_CH5_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn in_page_bound_en_ch5(&self) -> IN_PAGE_BOUND_EN_CH5_R {
        IN_PAGE_BOUND_EN_CH5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset Rx channel"]
    #[inline(always)]
    pub fn in_rst_ch5(&self) -> IN_RST_CH5_R {
        IN_RST_CH5_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    pub fn in_cmd_disable_ch5(&self) -> IN_CMD_DISABLE_CH5_R {
        IN_CMD_DISABLE_CH5_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF0_CH5")
            .field(
                "in_ecc_aes_en_ch5",
                &format_args!("{}", self.in_ecc_aes_en_ch5().bit()),
            )
            .field(
                "in_mem_burst_length_ch5",
                &format_args!("{}", self.in_mem_burst_length_ch5().bits()),
            )
            .field(
                "in_page_bound_en_ch5",
                &format_args!("{}", self.in_page_bound_en_ch5().bit()),
            )
            .field("in_rst_ch5", &format_args!("{}", self.in_rst_ch5().bit()))
            .field(
                "in_cmd_disable_ch5",
                &format_args!("{}", self.in_cmd_disable_ch5().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CONF0_CH5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn in_ecc_aes_en_ch5(&mut self) -> IN_ECC_AES_EN_CH5_W<IN_CONF0_CH5_SPEC> {
        IN_ECC_AES_EN_CH5_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Block size of Rx channel 1. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn in_mem_burst_length_ch5(&mut self) -> IN_MEM_BURST_LENGTH_CH5_W<IN_CONF0_CH5_SPEC> {
        IN_MEM_BURST_LENGTH_CH5_W::new(self, 6)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI write data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    #[must_use]
    pub fn in_page_bound_en_ch5(&mut self) -> IN_PAGE_BOUND_EN_CH5_W<IN_CONF0_CH5_SPEC> {
        IN_PAGE_BOUND_EN_CH5_W::new(self, 12)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset Rx channel"]
    #[inline(always)]
    #[must_use]
    pub fn in_rst_ch5(&mut self) -> IN_RST_CH5_W<IN_CONF0_CH5_SPEC> {
        IN_RST_CH5_W::new(self, 24)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    #[must_use]
    pub fn in_cmd_disable_ch5(&mut self) -> IN_CMD_DISABLE_CH5_W<IN_CONF0_CH5_SPEC> {
        IN_CMD_DISABLE_CH5_W::new(self, 25)
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
#[doc = "RX CH5 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF0_CH5_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf0_ch5::R`](R) reader structure"]
impl crate::Readable for IN_CONF0_CH5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf0_ch5::W`](W) writer structure"]
impl crate::Writable for IN_CONF0_CH5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_CONF0_CH5 to value 0"]
impl crate::Resettable for IN_CONF0_CH5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
