#[doc = "Register `SMEM_AC` reader"]
pub type R = crate::R<SMEM_AC_SPEC>;
#[doc = "Register `SMEM_AC` writer"]
pub type W = crate::W<SMEM_AC_SPEC>;
#[doc = "Field `SMEM_CS_SETUP` reader - For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type SMEM_CS_SETUP_R = crate::BitReader;
#[doc = "Field `SMEM_CS_SETUP` writer - For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type SMEM_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_CS_HOLD` reader - For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type SMEM_CS_HOLD_R = crate::BitReader;
#[doc = "Field `SMEM_CS_HOLD` writer - For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type SMEM_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_CS_SETUP_TIME` reader - For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
pub type SMEM_CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `SMEM_CS_SETUP_TIME` writer - For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
pub type SMEM_CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SMEM_CS_HOLD_TIME` reader - For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
pub type SMEM_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SMEM_CS_HOLD_TIME` writer - For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
pub type SMEM_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SMEM_ECC_CS_HOLD_TIME` reader - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM."]
pub type SMEM_ECC_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SMEM_ECC_CS_HOLD_TIME` writer - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM."]
pub type SMEM_ECC_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMEM_ECC_SKIP_PAGE_CORNER` reader - 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM."]
pub type SMEM_ECC_SKIP_PAGE_CORNER_R = crate::BitReader;
#[doc = "Field `SMEM_ECC_SKIP_PAGE_CORNER` writer - 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM."]
pub type SMEM_ECC_SKIP_PAGE_CORNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_ECC_16TO18_BYTE_EN` reader - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM."]
pub type SMEM_ECC_16TO18_BYTE_EN_R = crate::BitReader;
#[doc = "Field `SMEM_ECC_16TO18_BYTE_EN` writer - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM."]
pub type SMEM_ECC_16TO18_BYTE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SMEM_CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `SMEM_CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SMEM_CS_HOLD_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SMEM_SPLIT_TRANS_EN` reader - Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type SMEM_SPLIT_TRANS_EN_R = crate::BitReader;
#[doc = "Field `SMEM_SPLIT_TRANS_EN` writer - Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type SMEM_SPLIT_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn smem_cs_setup(&self) -> SMEM_CS_SETUP_R {
        SMEM_CS_SETUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn smem_cs_hold(&self) -> SMEM_CS_HOLD_R {
        SMEM_CS_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    #[inline(always)]
    pub fn smem_cs_setup_time(&self) -> SMEM_CS_SETUP_TIME_R {
        SMEM_CS_SETUP_TIME_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    #[inline(always)]
    pub fn smem_cs_hold_time(&self) -> SMEM_CS_HOLD_TIME_R {
        SMEM_CS_HOLD_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM."]
    #[inline(always)]
    pub fn smem_ecc_cs_hold_time(&self) -> SMEM_ECC_CS_HOLD_TIME_R {
        SMEM_ECC_CS_HOLD_TIME_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM."]
    #[inline(always)]
    pub fn smem_ecc_skip_page_corner(&self) -> SMEM_ECC_SKIP_PAGE_CORNER_R {
        SMEM_ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM."]
    #[inline(always)]
    pub fn smem_ecc_16to18_byte_en(&self) -> SMEM_ECC_16TO18_BYTE_EN_R {
        SMEM_ECC_16TO18_BYTE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn smem_cs_hold_delay(&self) -> SMEM_CS_HOLD_DELAY_R {
        SMEM_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    pub fn smem_split_trans_en(&self) -> SMEM_SPLIT_TRANS_EN_R {
        SMEM_SPLIT_TRANS_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_AC")
            .field("smem_cs_setup", &self.smem_cs_setup())
            .field("smem_cs_hold", &self.smem_cs_hold())
            .field("smem_cs_setup_time", &self.smem_cs_setup_time())
            .field("smem_cs_hold_time", &self.smem_cs_hold_time())
            .field("smem_ecc_cs_hold_time", &self.smem_ecc_cs_hold_time())
            .field(
                "smem_ecc_skip_page_corner",
                &self.smem_ecc_skip_page_corner(),
            )
            .field("smem_ecc_16to18_byte_en", &self.smem_ecc_16to18_byte_en())
            .field("smem_cs_hold_delay", &self.smem_cs_hold_delay())
            .field("smem_split_trans_en", &self.smem_split_trans_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn smem_cs_setup(&mut self) -> SMEM_CS_SETUP_W<SMEM_AC_SPEC> {
        SMEM_CS_SETUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn smem_cs_hold(&mut self) -> SMEM_CS_HOLD_W<SMEM_AC_SPEC> {
        SMEM_CS_HOLD_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    #[inline(always)]
    pub fn smem_cs_setup_time(&mut self) -> SMEM_CS_SETUP_TIME_W<SMEM_AC_SPEC> {
        SMEM_CS_SETUP_TIME_W::new(self, 2)
    }
    #[doc = "Bits 7:11 - For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    #[inline(always)]
    pub fn smem_cs_hold_time(&mut self) -> SMEM_CS_HOLD_TIME_W<SMEM_AC_SPEC> {
        SMEM_CS_HOLD_TIME_W::new(self, 7)
    }
    #[doc = "Bits 12:14 - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM."]
    #[inline(always)]
    pub fn smem_ecc_cs_hold_time(&mut self) -> SMEM_ECC_CS_HOLD_TIME_W<SMEM_AC_SPEC> {
        SMEM_ECC_CS_HOLD_TIME_W::new(self, 12)
    }
    #[doc = "Bit 15 - 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM."]
    #[inline(always)]
    pub fn smem_ecc_skip_page_corner(&mut self) -> SMEM_ECC_SKIP_PAGE_CORNER_W<SMEM_AC_SPEC> {
        SMEM_ECC_SKIP_PAGE_CORNER_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM."]
    #[inline(always)]
    pub fn smem_ecc_16to18_byte_en(&mut self) -> SMEM_ECC_16TO18_BYTE_EN_W<SMEM_AC_SPEC> {
        SMEM_ECC_16TO18_BYTE_EN_W::new(self, 16)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn smem_cs_hold_delay(&mut self) -> SMEM_CS_HOLD_DELAY_W<SMEM_AC_SPEC> {
        SMEM_CS_HOLD_DELAY_W::new(self, 25)
    }
    #[doc = "Bit 31 - Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    pub fn smem_split_trans_en(&mut self) -> SMEM_SPLIT_TRANS_EN_W<SMEM_AC_SPEC> {
        SMEM_SPLIT_TRANS_EN_W::new(self, 31)
    }
}
#[doc = "MSPI external RAM ECC and SPI CS timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_AC_SPEC;
impl crate::RegisterSpec for SMEM_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_ac::R`](R) reader structure"]
impl crate::Readable for SMEM_AC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_ac::W`](W) writer structure"]
impl crate::Writable for SMEM_AC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_AC to value 0xb084"]
impl crate::Resettable for SMEM_AC_SPEC {
    const RESET_VALUE: u32 = 0xb084;
}
