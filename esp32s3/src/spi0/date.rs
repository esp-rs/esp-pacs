#[doc = "Register `DATE` reader"]
pub struct R(crate::R<DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATE` writer"]
pub struct W(crate::W<DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATE_SPEC>;
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
impl From<crate::W<DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_SPICLK_FUN_DRV` reader - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
pub type SPI_SMEM_SPICLK_FUN_DRV_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_SPICLK_FUN_DRV` writer - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
pub type SPI_SMEM_SPICLK_FUN_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, DATE_SPEC, 2, O>;
#[doc = "Field `SPI_FMEM_SPICLK_FUN_DRV` reader - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
pub type SPI_FMEM_SPICLK_FUN_DRV_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_SPICLK_FUN_DRV` writer - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
pub type SPI_FMEM_SPICLK_FUN_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, DATE_SPEC, 2, O>;
#[doc = "Field `SPI_SPICLK_PAD_DRV_CTL_EN` reader - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\] of SPICLK PAD."]
pub type SPI_SPICLK_PAD_DRV_CTL_EN_R = crate::BitReader;
#[doc = "Field `SPI_SPICLK_PAD_DRV_CTL_EN` writer - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\] of SPICLK PAD."]
pub type SPI_SPICLK_PAD_DRV_CTL_EN_W<'a, const O: u8> = crate::BitWriter<'a, DATE_SPEC, O>;
#[doc = "Field `DATE` reader - SPI register version."]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - SPI register version."]
pub type DATE_W<'a, const O: u8> = crate::FieldWriter<'a, DATE_SPEC, 23, O, u32>;
impl R {
    #[doc = "Bits 0:1 - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_spiclk_fun_drv(&self) -> SPI_SMEM_SPICLK_FUN_DRV_R {
        SPI_SMEM_SPICLK_FUN_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_spiclk_fun_drv(&self) -> SPI_FMEM_SPICLK_FUN_DRV_R {
        SPI_FMEM_SPICLK_FUN_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\] of SPICLK PAD."]
    #[inline(always)]
    pub fn spi_spiclk_pad_drv_ctl_en(&self) -> SPI_SPICLK_PAD_DRV_CTL_EN_R {
        SPI_SPICLK_PAD_DRV_CTL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:27 - SPI register version."]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new((self.bits >> 5) & 0x007f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field(
                "spi_smem_spiclk_fun_drv",
                &format_args!("{}", self.spi_smem_spiclk_fun_drv().bits()),
            )
            .field(
                "spi_fmem_spiclk_fun_drv",
                &format_args!("{}", self.spi_fmem_spiclk_fun_drv().bits()),
            )
            .field(
                "spi_spiclk_pad_drv_ctl_en",
                &format_args!("{}", self.spi_spiclk_pad_drv_ctl_en().bit()),
            )
            .field("date", &format_args!("{}", self.date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_spiclk_fun_drv(&mut self) -> SPI_SMEM_SPICLK_FUN_DRV_W<0> {
        SPI_SMEM_SPICLK_FUN_DRV_W::new(self)
    }
    #[doc = "Bits 2:3 - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_spiclk_fun_drv(&mut self) -> SPI_FMEM_SPICLK_FUN_DRV_W<2> {
        SPI_FMEM_SPICLK_FUN_DRV_W::new(self)
    }
    #[doc = "Bit 4 - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\] of SPICLK PAD."]
    #[inline(always)]
    #[must_use]
    pub fn spi_spiclk_pad_drv_ctl_en(&mut self) -> SPI_SPICLK_PAD_DRV_CTL_EN_W<4> {
        SPI_SPICLK_PAD_DRV_CTL_EN_W::new(self)
    }
    #[doc = "Bits 5:27 - SPI register version."]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<5> {
        DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 version control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](index.html) module"]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [date::R](R) reader structure"]
impl crate::Readable for DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [date::W](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATE to value 0x0210_1040"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0210_1040;
}
