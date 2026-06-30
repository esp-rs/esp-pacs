#[doc = "Register `SPI_MEM_INT_ENA` reader"]
pub type R = crate::R<SPI_MEM_INT_ENA_SPEC>;
#[doc = "Register `SPI_MEM_INT_ENA` writer"]
pub type W = crate::W<SPI_MEM_INT_ENA_SPEC>;
#[doc = "Field `SPI_MEM_PER_END_INT_ENA` reader - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub type SPI_MEM_PER_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PER_END_INT_ENA` writer - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub type SPI_MEM_PER_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PES_END_INT_ENA` reader - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub type SPI_MEM_PES_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PES_END_INT_ENA` writer - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub type SPI_MEM_PES_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_WPE_END_INT_ENA` reader - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
pub type SPI_MEM_WPE_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WPE_END_INT_ENA` writer - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
pub type SPI_MEM_WPE_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_ENA` reader - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type SPI_MEM_BROWN_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_ENA` writer - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type SPI_MEM_BROWN_OUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_per_end_int_ena(&self) -> SPI_MEM_PER_END_INT_ENA_R {
        SPI_MEM_PER_END_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_ena(&self) -> SPI_MEM_PES_END_INT_ENA_R {
        SPI_MEM_PES_END_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_ena(&self) -> SPI_MEM_WPE_END_INT_ENA_R {
        SPI_MEM_WPE_END_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_ena(&self) -> SPI_MEM_SLV_ST_END_INT_ENA_R {
        SPI_MEM_SLV_ST_END_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_ena(&self) -> SPI_MEM_MST_ST_END_INT_ENA_R {
        SPI_MEM_MST_ST_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_ena(&self) -> SPI_MEM_BROWN_OUT_INT_ENA_R {
        SPI_MEM_BROWN_OUT_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_INT_ENA")
            .field("spi_mem_per_end_int_ena", &self.spi_mem_per_end_int_ena())
            .field("spi_mem_pes_end_int_ena", &self.spi_mem_pes_end_int_ena())
            .field("spi_mem_wpe_end_int_ena", &self.spi_mem_wpe_end_int_ena())
            .field(
                "spi_mem_slv_st_end_int_ena",
                &self.spi_mem_slv_st_end_int_ena(),
            )
            .field(
                "spi_mem_mst_st_end_int_ena",
                &self.spi_mem_mst_st_end_int_ena(),
            )
            .field(
                "spi_mem_brown_out_int_ena",
                &self.spi_mem_brown_out_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_per_end_int_ena(
        &mut self,
    ) -> SPI_MEM_PER_END_INT_ENA_W<'_, SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_PER_END_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_ena(
        &mut self,
    ) -> SPI_MEM_PES_END_INT_ENA_W<'_, SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_PES_END_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_ena(
        &mut self,
    ) -> SPI_MEM_WPE_END_INT_ENA_W<'_, SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_WPE_END_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_ena(
        &mut self,
    ) -> SPI_MEM_SLV_ST_END_INT_ENA_W<'_, SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_SLV_ST_END_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_ena(
        &mut self,
    ) -> SPI_MEM_MST_ST_END_INT_ENA_W<'_, SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_MST_ST_END_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 10 - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_ena(
        &mut self,
    ) -> SPI_MEM_BROWN_OUT_INT_ENA_W<'_, SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_BROWN_OUT_INT_ENA_W::new(self, 10)
    }
}
#[doc = "SPI1 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_INT_ENA_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_int_ena::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_int_ena::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_INT_ENA to value 0"]
impl crate::Resettable for SPI_MEM_INT_ENA_SPEC {}
