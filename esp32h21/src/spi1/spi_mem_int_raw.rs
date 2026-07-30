#[doc = "Register `SPI_MEM_INT_RAW` reader"]
pub type R = crate::R<SPI_MEM_INT_RAW_SPEC>;
#[doc = "Register `SPI_MEM_INT_RAW` writer"]
pub type W = crate::W<SPI_MEM_INT_RAW_SPEC>;
#[doc = "Field `SPI_MEM_PER_END_INT_RAW` reader - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
pub type SPI_MEM_PER_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PER_END_INT_RAW` writer - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
pub type SPI_MEM_PER_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PES_END_INT_RAW` reader - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
pub type SPI_MEM_PES_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PES_END_INT_RAW` writer - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
pub type SPI_MEM_PES_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_WPE_END_INT_RAW` reader - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
pub type SPI_MEM_WPE_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WPE_END_INT_RAW` writer - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
pub type SPI_MEM_WPE_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SPI_MEM_SLV_ST_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_RAW` writer - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SPI_MEM_SLV_ST_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
pub type SPI_MEM_MST_ST_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_RAW` writer - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
pub type SPI_MEM_MST_ST_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_RAW` reader - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
pub type SPI_MEM_BROWN_OUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_RAW` writer - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
pub type SPI_MEM_BROWN_OUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("spi_mem_per_end_int_raw", &self.spi_mem_per_end_int_raw())
            .field("spi_mem_pes_end_int_raw", &self.spi_mem_pes_end_int_raw())
            .field("spi_mem_wpe_end_int_raw", &self.spi_mem_wpe_end_int_raw())
            .field(
                "spi_mem_slv_st_end_int_raw",
                &self.spi_mem_slv_st_end_int_raw(),
            )
            .field(
                "spi_mem_mst_st_end_int_raw",
                &self.spi_mem_mst_st_end_int_raw(),
            )
            .field(
                "spi_mem_brown_out_int_raw",
                &self.spi_mem_brown_out_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_per_end_int_raw(
        &mut self,
    ) -> SPI_MEM_PER_END_INT_RAW_W<'_, SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_PER_END_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_raw(
        &mut self,
    ) -> SPI_MEM_PES_END_INT_RAW_W<'_, SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_PES_END_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_raw(
        &mut self,
    ) -> SPI_MEM_WPE_END_INT_RAW_W<'_, SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_WPE_END_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_raw(
        &mut self,
    ) -> SPI_MEM_SLV_ST_END_INT_RAW_W<'_, SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_SLV_ST_END_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_raw(
        &mut self,
    ) -> SPI_MEM_MST_ST_END_INT_RAW_W<'_, SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_MST_ST_END_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 10 - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_raw(
        &mut self,
    ) -> SPI_MEM_BROWN_OUT_INT_RAW_W<'_, SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_BROWN_OUT_INT_RAW_W::new(self, 10)
    }
}
#[doc = "SPI1 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_INT_RAW_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_int_raw::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_int_raw::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_INT_RAW to value 0"]
impl crate::Resettable for SPI_MEM_INT_RAW_SPEC {}
