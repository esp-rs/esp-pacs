#[doc = "Register `SPI_MEM_INT_RAW` reader"]
pub struct R(crate::R<SPI_MEM_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_INT_RAW` writer"]
pub struct W(crate::W<SPI_MEM_INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_INT_RAW_SPEC>;
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
impl From<crate::W<SPI_MEM_INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_PER_END_INT_RAW` reader - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
pub type SPI_MEM_PER_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PER_END_INT_RAW` writer - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
pub type SPI_MEM_PER_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_INT_RAW_SPEC, O>;
#[doc = "Field `SPI_MEM_PES_END_INT_RAW` reader - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
pub type SPI_MEM_PES_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PES_END_INT_RAW` writer - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
pub type SPI_MEM_PES_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_INT_RAW_SPEC, O>;
#[doc = "Field `SPI_MEM_WPE_END_INT_RAW` reader - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
pub type SPI_MEM_WPE_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WPE_END_INT_RAW` writer - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
pub type SPI_MEM_WPE_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_INT_RAW_SPEC, O>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SPI_MEM_SLV_ST_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_RAW` writer - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SPI_MEM_SLV_ST_END_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_INT_RAW_SPEC, O>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
pub type SPI_MEM_MST_ST_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_RAW` writer - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
pub type SPI_MEM_MST_ST_END_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_INT_RAW_SPEC, O>;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_RAW` reader - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
pub type SPI_MEM_BROWN_OUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_RAW` writer - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
pub type SPI_MEM_BROWN_OUT_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_per_end_int_raw(&self) -> SPI_MEM_PER_END_INT_RAW_R {
        SPI_MEM_PER_END_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_raw(&self) -> SPI_MEM_PES_END_INT_RAW_R {
        SPI_MEM_PES_END_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_raw(&self) -> SPI_MEM_WPE_END_INT_RAW_R {
        SPI_MEM_WPE_END_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_raw(&self) -> SPI_MEM_SLV_ST_END_INT_RAW_R {
        SPI_MEM_SLV_ST_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_raw(&self) -> SPI_MEM_MST_ST_END_INT_RAW_R {
        SPI_MEM_MST_ST_END_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_raw(&self) -> SPI_MEM_BROWN_OUT_INT_RAW_R {
        SPI_MEM_BROWN_OUT_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_INT_RAW")
            .field(
                "spi_mem_per_end_int_raw",
                &format_args!("{}", self.spi_mem_per_end_int_raw().bit()),
            )
            .field(
                "spi_mem_pes_end_int_raw",
                &format_args!("{}", self.spi_mem_pes_end_int_raw().bit()),
            )
            .field(
                "spi_mem_wpe_end_int_raw",
                &format_args!("{}", self.spi_mem_wpe_end_int_raw().bit()),
            )
            .field(
                "spi_mem_slv_st_end_int_raw",
                &format_args!("{}", self.spi_mem_slv_st_end_int_raw().bit()),
            )
            .field(
                "spi_mem_mst_st_end_int_raw",
                &format_args!("{}", self.spi_mem_mst_st_end_int_raw().bit()),
            )
            .field(
                "spi_mem_brown_out_int_raw",
                &format_args!("{}", self.spi_mem_brown_out_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_per_end_int_raw(&mut self) -> SPI_MEM_PER_END_INT_RAW_W<0> {
        SPI_MEM_PER_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pes_end_int_raw(&mut self) -> SPI_MEM_PES_END_INT_RAW_W<1> {
        SPI_MEM_PES_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wpe_end_int_raw(&mut self) -> SPI_MEM_WPE_END_INT_RAW_W<2> {
        SPI_MEM_WPE_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_slv_st_end_int_raw(&mut self) -> SPI_MEM_SLV_ST_END_INT_RAW_W<3> {
        SPI_MEM_SLV_ST_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_mst_st_end_int_raw(&mut self) -> SPI_MEM_MST_ST_END_INT_RAW_W<4> {
        SPI_MEM_MST_ST_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 10 - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_brown_out_int_raw(&mut self) -> SPI_MEM_BROWN_OUT_INT_RAW_W<10> {
        SPI_MEM_BROWN_OUT_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_raw](index.html) module"]
pub struct SPI_MEM_INT_RAW_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_int_raw::R](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_int_raw::W](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_INT_RAW to value 0"]
impl crate::Resettable for SPI_MEM_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
