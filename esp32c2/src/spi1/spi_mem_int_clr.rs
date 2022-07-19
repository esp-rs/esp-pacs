#[doc = "Register `SPI_MEM_INT_CLR` writer"]
pub struct W(crate::W<SPI_MEM_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_INT_CLR_SPEC>;
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
impl From<crate::W<SPI_MEM_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_PER_END_INT_CLR` writer - The clear bit for SPI_MEM_PER_END_INT interrupt."]
pub type SPI_MEM_PER_END_INT_CLR_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_INT_CLR_SPEC, bool, 0>;
#[doc = "Field `SPI_MEM_PES_END_INT_CLR` writer - The clear bit for SPI_MEM_PES_END_INT interrupt."]
pub type SPI_MEM_PES_END_INT_CLR_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_INT_CLR_SPEC, bool, 1>;
#[doc = "Field `SPI_MEM_WPE_END_INT_CLR` writer - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
pub type SPI_MEM_WPE_END_INT_CLR_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_INT_CLR_SPEC, bool, 2>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_CLR_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_INT_CLR_SPEC, bool, 3>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_CLR_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_INT_CLR_SPEC, bool, 4>;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_CLR` writer - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type SPI_MEM_BROWN_OUT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_INT_CLR_SPEC, bool, 5>;
impl W {
    #[doc = "Bit 0 - The clear bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_per_end_int_clr(&mut self) -> SPI_MEM_PER_END_INT_CLR_W {
        SPI_MEM_PER_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The clear bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_clr(&mut self) -> SPI_MEM_PES_END_INT_CLR_W {
        SPI_MEM_PES_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_clr(&mut self) -> SPI_MEM_WPE_END_INT_CLR_W {
        SPI_MEM_WPE_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_clr(&mut self) -> SPI_MEM_SLV_ST_END_INT_CLR_W {
        SPI_MEM_SLV_ST_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_clr(&mut self) -> SPI_MEM_MST_ST_END_INT_CLR_W {
        SPI_MEM_MST_ST_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_clr(&mut self) -> SPI_MEM_BROWN_OUT_INT_CLR_W {
        SPI_MEM_BROWN_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_clr](index.html) module"]
pub struct SPI_MEM_INT_CLR_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_int_clr::W](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_INT_CLR to value 0"]
impl crate::Resettable for SPI_MEM_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
