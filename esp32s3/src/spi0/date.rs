#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `SPI_SMEM_SPICLK_FUN_DRV` reader - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
pub type SPI_SMEM_SPICLK_FUN_DRV_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_SPICLK_FUN_DRV` writer - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
pub type SPI_SMEM_SPICLK_FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_FMEM_SPICLK_FUN_DRV` reader - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
pub type SPI_FMEM_SPICLK_FUN_DRV_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_SPICLK_FUN_DRV` writer - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
pub type SPI_FMEM_SPICLK_FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SPICLK_PAD_DRV_CTL_EN` reader - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\] of SPICLK PAD."]
pub type SPI_SPICLK_PAD_DRV_CTL_EN_R = crate::BitReader;
#[doc = "Field `SPI_SPICLK_PAD_DRV_CTL_EN` writer - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\] of SPICLK PAD."]
pub type SPI_SPICLK_PAD_DRV_CTL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATE` reader - SPI register version."]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - SPI register version."]
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
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
            .field("spi_smem_spiclk_fun_drv", &self.spi_smem_spiclk_fun_drv())
            .field("spi_fmem_spiclk_fun_drv", &self.spi_fmem_spiclk_fun_drv())
            .field(
                "spi_spiclk_pad_drv_ctl_en",
                &self.spi_spiclk_pad_drv_ctl_en(),
            )
            .field("date", &self.date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_spiclk_fun_drv(&mut self) -> SPI_SMEM_SPICLK_FUN_DRV_W<DATE_SPEC> {
        SPI_SMEM_SPICLK_FUN_DRV_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_spiclk_fun_drv(&mut self) -> SPI_FMEM_SPICLK_FUN_DRV_W<DATE_SPEC> {
        SPI_FMEM_SPICLK_FUN_DRV_W::new(self, 2)
    }
    #[doc = "Bit 4 - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\] and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\] of SPICLK PAD."]
    #[inline(always)]
    pub fn spi_spiclk_pad_drv_ctl_en(&mut self) -> SPI_SPICLK_PAD_DRV_CTL_EN_W<DATE_SPEC> {
        SPI_SPICLK_PAD_DRV_CTL_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:27 - SPI register version."]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W<DATE_SPEC> {
        DATE_W::new(self, 5)
    }
}
#[doc = "SPI0 version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0x0210_1040"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0210_1040;
}
