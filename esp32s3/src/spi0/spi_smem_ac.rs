#[doc = "Register `SPI_SMEM_AC` reader"]
pub struct R(crate::R<SPI_SMEM_AC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_AC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_AC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_AC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SMEM_AC` writer"]
pub struct W(crate::W<SPI_SMEM_AC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SMEM_AC_SPEC>;
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
impl From<crate::W<SPI_SMEM_AC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SMEM_AC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_CS_SETUP` reader - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
pub type SPI_SMEM_CS_SETUP_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_CS_SETUP` writer - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
pub type SPI_SMEM_CS_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SMEM_AC_SPEC, O>;
#[doc = "Field `SPI_SMEM_CS_HOLD` reader - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
pub type SPI_SMEM_CS_HOLD_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_CS_HOLD` writer - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
pub type SPI_SMEM_CS_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SMEM_AC_SPEC, O>;
#[doc = "Field `SPI_SMEM_CS_SETUP_TIME` reader - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
pub type SPI_SMEM_CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_CS_SETUP_TIME` writer - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
pub type SPI_SMEM_CS_SETUP_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_SMEM_AC_SPEC, 5, O>;
#[doc = "Field `SPI_SMEM_CS_HOLD_TIME` reader - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
pub type SPI_SMEM_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_CS_HOLD_TIME` writer - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
pub type SPI_SMEM_CS_HOLD_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_SMEM_AC_SPEC, 5, O>;
#[doc = "Field `SPI_SMEM_ECC_CS_HOLD_TIME` reader - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the MSPI CS hold cycles in ECC mode when accesses to external RAM."]
pub type SPI_SMEM_ECC_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_ECC_CS_HOLD_TIME` writer - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the MSPI CS hold cycles in ECC mode when accesses to external RAM."]
pub type SPI_SMEM_ECC_CS_HOLD_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_SMEM_AC_SPEC, 3, O>;
#[doc = "Field `SPI_SMEM_ECC_SKIP_PAGE_CORNER` reader - 1: MSPI skips page corner when accesses to external RAM. 0: Not skip page corner when accesses to external RAM."]
pub type SPI_SMEM_ECC_SKIP_PAGE_CORNER_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_ECC_SKIP_PAGE_CORNER` writer - 1: MSPI skips page corner when accesses to external RAM. 0: Not skip page corner when accesses to external RAM."]
pub type SPI_SMEM_ECC_SKIP_PAGE_CORNER_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_SMEM_AC_SPEC, O>;
#[doc = "Field `SPI_SMEM_ECC_16TO18_BYTE_EN` reader - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses to external RAM."]
pub type SPI_SMEM_ECC_16TO18_BYTE_EN_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_ECC_16TO18_BYTE_EN` writer - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses to external RAM."]
pub type SPI_SMEM_ECC_16TO18_BYTE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SMEM_AC_SPEC, O>;
#[doc = "Field `SPI_SMEM_ECC_ERR_INT_EN` reader - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
pub type SPI_SMEM_ECC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_ECC_ERR_INT_EN` writer - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
pub type SPI_SMEM_ECC_ERR_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SMEM_AC_SPEC, O>;
#[doc = "Field `SPI_SMEM_CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SPI_SMEM_CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SPI_SMEM_CS_HOLD_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_SMEM_AC_SPEC, 6, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
    #[inline(always)]
    pub fn spi_smem_cs_setup(&self) -> SPI_SMEM_CS_SETUP_R {
        SPI_SMEM_CS_SETUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
    #[inline(always)]
    pub fn spi_smem_cs_hold(&self) -> SPI_SMEM_CS_HOLD_R {
        SPI_SMEM_CS_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn spi_smem_cs_setup_time(&self) -> SPI_SMEM_CS_SETUP_TIME_R {
        SPI_SMEM_CS_SETUP_TIME_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn spi_smem_cs_hold_time(&self) -> SPI_SMEM_CS_HOLD_TIME_R {
        SPI_SMEM_CS_HOLD_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the MSPI CS hold cycles in ECC mode when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_cs_hold_time(&self) -> SPI_SMEM_ECC_CS_HOLD_TIME_R {
        SPI_SMEM_ECC_CS_HOLD_TIME_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 1: MSPI skips page corner when accesses to external RAM. 0: Not skip page corner when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_skip_page_corner(&self) -> SPI_SMEM_ECC_SKIP_PAGE_CORNER_R {
        SPI_SMEM_ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_16to18_byte_en(&self) -> SPI_SMEM_ECC_16TO18_BYTE_EN_R {
        SPI_SMEM_ECC_16TO18_BYTE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_err_int_en(&self) -> SPI_SMEM_ECC_ERR_INT_EN_R {
        SPI_SMEM_ECC_ERR_INT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn spi_smem_cs_hold_delay(&self) -> SPI_SMEM_CS_HOLD_DELAY_R {
        SPI_SMEM_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
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
                "spi_smem_ecc_err_int_en",
                &format_args!("{}", self.spi_smem_ecc_err_int_en().bit()),
            )
            .field(
                "spi_smem_cs_hold_delay",
                &format_args!("{}", self.spi_smem_cs_hold_delay().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SMEM_AC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_setup(&mut self) -> SPI_SMEM_CS_SETUP_W<0> {
        SPI_SMEM_CS_SETUP_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_hold(&mut self) -> SPI_SMEM_CS_HOLD_W<1> {
        SPI_SMEM_CS_HOLD_W::new(self)
    }
    #[doc = "Bits 2:6 - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_setup_time(&mut self) -> SPI_SMEM_CS_SETUP_TIME_W<2> {
        SPI_SMEM_CS_SETUP_TIME_W::new(self)
    }
    #[doc = "Bits 7:11 - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_hold_time(&mut self) -> SPI_SMEM_CS_HOLD_TIME_W<7> {
        SPI_SMEM_CS_HOLD_TIME_W::new(self)
    }
    #[doc = "Bits 12:14 - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the MSPI CS hold cycles in ECC mode when accesses to external RAM."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_ecc_cs_hold_time(&mut self) -> SPI_SMEM_ECC_CS_HOLD_TIME_W<12> {
        SPI_SMEM_ECC_CS_HOLD_TIME_W::new(self)
    }
    #[doc = "Bit 15 - 1: MSPI skips page corner when accesses to external RAM. 0: Not skip page corner when accesses to external RAM."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_ecc_skip_page_corner(&mut self) -> SPI_SMEM_ECC_SKIP_PAGE_CORNER_W<15> {
        SPI_SMEM_ECC_SKIP_PAGE_CORNER_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses to external RAM."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_ecc_16to18_byte_en(&mut self) -> SPI_SMEM_ECC_16TO18_BYTE_EN_W<16> {
        SPI_SMEM_ECC_16TO18_BYTE_EN_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_ecc_err_int_en(&mut self) -> SPI_SMEM_ECC_ERR_INT_EN_W<24> {
        SPI_SMEM_ECC_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_cs_hold_delay(&mut self) -> SPI_SMEM_CS_HOLD_DELAY_W<25> {
        SPI_SMEM_CS_HOLD_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI external RAM ECC and SPI CS timing control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_ac](index.html) module"]
pub struct SPI_SMEM_AC_SPEC;
impl crate::RegisterSpec for SPI_SMEM_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_ac::R](R) reader structure"]
impl crate::Readable for SPI_SMEM_AC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_smem_ac::W](W) writer structure"]
impl crate::Writable for SPI_SMEM_AC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_SMEM_AC to value 0xb084"]
impl crate::Resettable for SPI_SMEM_AC_SPEC {
    const RESET_VALUE: Self::Ux = 0xb084;
}
