#[doc = "Register `SPI_SMEM_TIMING_CALI` reader"]
pub struct R(crate::R<SPI_SMEM_TIMING_CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_TIMING_CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_TIMING_CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_TIMING_CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SMEM_TIMING_CALI` writer"]
pub struct W(crate::W<SPI_SMEM_TIMING_CALI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SMEM_TIMING_CALI_SPEC>;
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
impl From<crate::W<SPI_SMEM_TIMING_CALI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SMEM_TIMING_CALI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` reader - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
pub type SPI_SMEM_TIMING_CLK_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` writer - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
pub type SPI_SMEM_TIMING_CLK_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_SMEM_TIMING_CALI_SPEC, O>;
#[doc = "Field `SPI_SMEM_TIMING_CALI` reader - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
pub type SPI_SMEM_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_TIMING_CALI` writer - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
pub type SPI_SMEM_TIMING_CALI_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_SMEM_TIMING_CALI_SPEC, O>;
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` reader - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to Ext_RAM. Active when SPI_SMEM_TIMING_CALI bit is set."]
pub type SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` writer - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to Ext_RAM. Active when SPI_SMEM_TIMING_CALI bit is set."]
pub type SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_SMEM_TIMING_CALI_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(&self) -> SPI_SMEM_TIMING_CLK_ENA_R {
        SPI_SMEM_TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_cali(&self) -> SPI_SMEM_TIMING_CALI_R {
        SPI_SMEM_TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to Ext_RAM. Active when SPI_SMEM_TIMING_CALI bit is set."]
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(&self) -> SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R {
        SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_TIMING_CALI")
            .field(
                "spi_smem_timing_clk_ena",
                &format_args!("{}", self.spi_smem_timing_clk_ena().bit()),
            )
            .field(
                "spi_smem_timing_cali",
                &format_args!("{}", self.spi_smem_timing_cali().bit()),
            )
            .field(
                "spi_smem_extra_dummy_cyclelen",
                &format_args!("{}", self.spi_smem_extra_dummy_cyclelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SMEM_TIMING_CALI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_timing_clk_ena(&mut self) -> SPI_SMEM_TIMING_CLK_ENA_W<0> {
        SPI_SMEM_TIMING_CLK_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_timing_cali(&mut self) -> SPI_SMEM_TIMING_CALI_W<1> {
        SPI_SMEM_TIMING_CALI_W::new(self)
    }
    #[doc = "Bits 2:4 - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to Ext_RAM. Active when SPI_SMEM_TIMING_CALI bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_extra_dummy_cyclelen(&mut self) -> SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W<2> {
        SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 Ext_RAM timing compensation register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_timing_cali](index.html) module"]
pub struct SPI_SMEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for SPI_SMEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_timing_cali::R](R) reader structure"]
impl crate::Readable for SPI_SMEM_TIMING_CALI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_smem_timing_cali::W](W) writer structure"]
impl crate::Writable for SPI_SMEM_TIMING_CALI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_SMEM_TIMING_CALI to value 0"]
impl crate::Resettable for SPI_SMEM_TIMING_CALI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
