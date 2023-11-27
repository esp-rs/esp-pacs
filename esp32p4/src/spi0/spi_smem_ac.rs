#[doc = "Register `SPI_SMEM_AC` reader"]
pub type R = crate::R<SPI_SMEM_AC_SPEC>;
#[doc = "Register `SPI_SMEM_AC` writer"]
pub type W = crate::W<SPI_SMEM_AC_SPEC>;
#[doc = "Field `SPI_SMEM_CS_SETUP` reader - For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type SPI_SMEM_CS_SETUP_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_CS_SETUP` writer - For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type SPI_SMEM_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_CS_HOLD` reader - For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type SPI_SMEM_CS_HOLD_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_CS_HOLD` writer - For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type SPI_SMEM_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_CS_SETUP_TIME` reader - For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
pub type SPI_SMEM_CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_CS_SETUP_TIME` writer - For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
pub type SPI_SMEM_CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPI_SMEM_CS_HOLD_TIME` reader - For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
pub type SPI_SMEM_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_CS_HOLD_TIME` writer - For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
pub type SPI_SMEM_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPI_SMEM_ECC_CS_HOLD_TIME` reader - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM."]
pub type SPI_SMEM_ECC_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_ECC_CS_HOLD_TIME` writer - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM."]
pub type SPI_SMEM_ECC_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_ECC_SKIP_PAGE_CORNER` reader - 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM."]
pub type SPI_SMEM_ECC_SKIP_PAGE_CORNER_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_ECC_SKIP_PAGE_CORNER` writer - 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM."]
pub type SPI_SMEM_ECC_SKIP_PAGE_CORNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_ECC_16TO18_BYTE_EN` reader - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM."]
pub type SPI_SMEM_ECC_16TO18_BYTE_EN_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_ECC_16TO18_BYTE_EN` writer - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM."]
pub type SPI_SMEM_ECC_16TO18_BYTE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SPI_SMEM_CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SPI_SMEM_CS_HOLD_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_SMEM_SPLIT_TRANS_EN` reader - Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type SPI_SMEM_SPLIT_TRANS_EN_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_SPLIT_TRANS_EN` writer - Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type SPI_SMEM_SPLIT_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_smem_cs_setup(&self) -> SPI_SMEM_CS_SETUP_R {
        SPI_SMEM_CS_SETUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_smem_cs_hold(&self) -> SPI_SMEM_CS_HOLD_R {
        SPI_SMEM_CS_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    #[inline(always)]
    pub fn spi_smem_cs_setup_time(&self) -> SPI_SMEM_CS_SETUP_TIME_R {
        SPI_SMEM_CS_SETUP_TIME_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    #[inline(always)]
    pub fn spi_smem_cs_hold_time(&self) -> SPI_SMEM_CS_HOLD_TIME_R {
        SPI_SMEM_CS_HOLD_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_cs_hold_time(&self) -> SPI_SMEM_ECC_CS_HOLD_TIME_R {
        SPI_SMEM_ECC_CS_HOLD_TIME_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_skip_page_corner(&self) -> SPI_SMEM_ECC_SKIP_PAGE_CORNER_R {
        SPI_SMEM_ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_16to18_byte_en(&self) -> SPI_SMEM_ECC_16TO18_BYTE_EN_R {
        SPI_SMEM_ECC_16TO18_BYTE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn spi_smem_cs_hold_delay(&self) -> SPI_SMEM_CS_HOLD_DELAY_R {
        SPI_SMEM_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    pub fn spi_smem_split_trans_en(&self) -> SPI_SMEM_SPLIT_TRANS_EN_R {
        SPI_SMEM_SPLIT_TRANS_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_AC")
            .field(
                "spi_smem_cs_setup",
                &format_args!("{}", self.spi_smem_cs_setup().bit()),
            )
            .field(
                "spi_smem_cs_hold",
                &format_args!("{}", self.spi_smem_cs_hold().bit()),
            )
            .field(
                "spi_smem_cs_setup_time",
                &format_args!("{}", self.spi_smem_cs_setup_time().bits()),
            )
            .field(
                "spi_smem_cs_hold_time",
                &format_args!("{}", self.spi_smem_cs_hold_time().bits()),
            )
            .field(
                "spi_smem_ecc_cs_hold_time",
                &format_args!("{}", self.spi_smem_ecc_cs_hold_time().bits()),
            )
            .field(
                "spi_smem_ecc_skip_page_corner",
                &format_args!("{}", self.spi_smem_ecc_skip_page_corner().bit()),
            )
            .field(
                "spi_smem_ecc_16to18_byte_en",
                &format_args!("{}", self.spi_smem_ecc_16to18_byte_en().bit()),
            )
            .field(
                "spi_smem_cs_hold_delay",
                &format_args!("{}", self.spi_smem_cs_hold_delay().bits()),
            )
            .field(
                "spi_smem_split_trans_en",
                &format_args!("{}", self.spi_smem_split_trans_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SMEM_AC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_setup(&mut self) -> SPI_SMEM_CS_SETUP_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_SETUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_hold(&mut self) -> SPI_SMEM_CS_HOLD_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_HOLD_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_setup_time(&mut self) -> SPI_SMEM_CS_SETUP_TIME_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_SETUP_TIME_W::new(self, 2)
    }
    #[doc = "Bits 7:11 - For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_hold_time(&mut self) -> SPI_SMEM_CS_HOLD_TIME_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_HOLD_TIME_W::new(self, 7)
    }
    #[doc = "Bits 12:14 - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_ecc_cs_hold_time(&mut self) -> SPI_SMEM_ECC_CS_HOLD_TIME_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_ECC_CS_HOLD_TIME_W::new(self, 12)
    }
    #[doc = "Bit 15 - 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_ecc_skip_page_corner(
        &mut self,
    ) -> SPI_SMEM_ECC_SKIP_PAGE_CORNER_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_ECC_SKIP_PAGE_CORNER_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_ecc_16to18_byte_en(
        &mut self,
    ) -> SPI_SMEM_ECC_16TO18_BYTE_EN_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_ECC_16TO18_BYTE_EN_W::new(self, 16)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_hold_delay(&mut self) -> SPI_SMEM_CS_HOLD_DELAY_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_HOLD_DELAY_W::new(self, 25)
    }
    #[doc = "Bit 31 - Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_split_trans_en(&mut self) -> SPI_SMEM_SPLIT_TRANS_EN_W<SPI_SMEM_AC_SPEC> {
        SPI_SMEM_SPLIT_TRANS_EN_W::new(self, 31)
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
#[doc = "MSPI external RAM ECC and SPI CS timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_ac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_AC_SPEC;
impl crate::RegisterSpec for SPI_SMEM_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_ac::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_AC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_ac::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_AC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_SMEM_AC to value 0x8000_b084"]
impl crate::Resettable for SPI_SMEM_AC_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_b084;
}
