#[doc = "Register `SPI_MEM_CTRL2` reader"]
pub struct R(crate::R<SPI_MEM_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CTRL2` writer"]
pub struct W(crate::W<SPI_MEM_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MEM_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CS_SETUP_TIME` reader - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
pub type SPI_MEM_CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CS_SETUP_TIME` writer - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
pub type SPI_MEM_CS_SETUP_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_CTRL2_SPEC, 5, O>;
#[doc = "Field `SPI_MEM_CS_HOLD_TIME` reader - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
pub type SPI_MEM_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CS_HOLD_TIME` writer - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
pub type SPI_MEM_CS_HOLD_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_MEM_CTRL2_SPEC, 5, O>;
#[doc = "Field `SPI_MEM_ECC_CS_HOLD_TIME` reader - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
pub type SPI_MEM_ECC_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_ECC_SKIP_PAGE_CORNER` reader - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub type SPI_MEM_ECC_SKIP_PAGE_CORNER_R = crate::BitReader;
#[doc = "Field `SPI_MEM_ECC_16TO18_BYTE_EN` reader - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub type SPI_MEM_ECC_16TO18_BYTE_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SPLIT_TRANS_EN` reader - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type SPI_MEM_SPLIT_TRANS_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SPI_MEM_CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SPI_MEM_CS_HOLD_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_CTRL2_SPEC, 6, O>;
#[doc = "Field `SPI_MEM_SYNC_RESET` writer - The spi0_mst_st and spi0_slv_st will be reset."]
pub type SPI_MEM_SYNC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL2_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn spi_mem_cs_setup_time(&self) -> SPI_MEM_CS_SETUP_TIME_R {
        SPI_MEM_CS_SETUP_TIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_time(&self) -> SPI_MEM_CS_HOLD_TIME_R {
        SPI_MEM_CS_HOLD_TIME_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn spi_mem_ecc_cs_hold_time(&self) -> SPI_MEM_ECC_CS_HOLD_TIME_R {
        SPI_MEM_ECC_CS_HOLD_TIME_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn spi_mem_ecc_skip_page_corner(&self) -> SPI_MEM_ECC_SKIP_PAGE_CORNER_R {
        SPI_MEM_ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn spi_mem_ecc_16to18_byte_en(&self) -> SPI_MEM_ECC_16TO18_BYTE_EN_R {
        SPI_MEM_ECC_16TO18_BYTE_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    pub fn spi_mem_split_trans_en(&self) -> SPI_MEM_SPLIT_TRANS_EN_R {
        SPI_MEM_SPLIT_TRANS_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_delay(&self) -> SPI_MEM_CS_HOLD_DELAY_R {
        SPI_MEM_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CTRL2")
            .field(
                "spi_mem_cs_setup_time",
                &format_args!("{}", self.spi_mem_cs_setup_time().bits()),
            )
            .field(
                "spi_mem_cs_hold_time",
                &format_args!("{}", self.spi_mem_cs_hold_time().bits()),
            )
            .field(
                "spi_mem_ecc_cs_hold_time",
                &format_args!("{}", self.spi_mem_ecc_cs_hold_time().bits()),
            )
            .field(
                "spi_mem_ecc_skip_page_corner",
                &format_args!("{}", self.spi_mem_ecc_skip_page_corner().bit()),
            )
            .field(
                "spi_mem_ecc_16to18_byte_en",
                &format_args!("{}", self.spi_mem_ecc_16to18_byte_en().bit()),
            )
            .field(
                "spi_mem_split_trans_en",
                &format_args!("{}", self.spi_mem_split_trans_en().bit()),
            )
            .field(
                "spi_mem_cs_hold_delay",
                &format_args!("{}", self.spi_mem_cs_hold_delay().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs_setup_time(&mut self) -> SPI_MEM_CS_SETUP_TIME_W<0> {
        SPI_MEM_CS_SETUP_TIME_W::new(self)
    }
    #[doc = "Bits 5:9 - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs_hold_time(&mut self) -> SPI_MEM_CS_HOLD_TIME_W<5> {
        SPI_MEM_CS_HOLD_TIME_W::new(self)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs_hold_delay(&mut self) -> SPI_MEM_CS_HOLD_DELAY_W<25> {
        SPI_MEM_CS_HOLD_DELAY_W::new(self)
    }
    #[doc = "Bit 31 - The spi0_mst_st and spi0_slv_st will be reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_sync_reset(&mut self) -> SPI_MEM_SYNC_RESET_W<31> {
        SPI_MEM_SYNC_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 control2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ctrl2](index.html) module"]
pub struct SPI_MEM_CTRL2_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_ctrl2::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_ctrl2::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL2 to value 0x2c21"]
impl crate::Resettable for SPI_MEM_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2c21;
}
