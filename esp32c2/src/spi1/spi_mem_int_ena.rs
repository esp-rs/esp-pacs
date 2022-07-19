#[doc = "Register `SPI_MEM_INT_ENA` reader"]
pub struct R(crate::R<SPI_MEM_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_INT_ENA` writer"]
pub struct W(crate::W<SPI_MEM_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_INT_ENA_SPEC>;
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
impl From<crate::W<SPI_MEM_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_PER_END_INT_ENA` reader - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub type SPI_MEM_PER_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_PER_END_INT_ENA` writer - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub type SPI_MEM_PER_END_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_INT_ENA_SPEC, bool, 0>;
#[doc = "Field `SPI_MEM_PES_END_INT_ENA` reader - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub type SPI_MEM_PES_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_PES_END_INT_ENA` writer - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub type SPI_MEM_PES_END_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_INT_ENA_SPEC, bool, 1>;
#[doc = "Field `SPI_MEM_WPE_END_INT_ENA` reader - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
pub type SPI_MEM_WPE_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_WPE_END_INT_ENA` writer - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
pub type SPI_MEM_WPE_END_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_INT_ENA_SPEC, bool, 2>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_INT_ENA_SPEC, bool, 3>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_INT_ENA_SPEC, bool, 4>;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_ENA` reader - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type SPI_MEM_BROWN_OUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_ENA` writer - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type SPI_MEM_BROWN_OUT_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_INT_ENA_SPEC, bool, 5>;
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
    #[doc = "Bit 5 - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_ena(&self) -> SPI_MEM_BROWN_OUT_INT_ENA_R {
        SPI_MEM_BROWN_OUT_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_per_end_int_ena(&mut self) -> SPI_MEM_PER_END_INT_ENA_W {
        SPI_MEM_PER_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_ena(&mut self) -> SPI_MEM_PES_END_INT_ENA_W {
        SPI_MEM_PES_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_ena(&mut self) -> SPI_MEM_WPE_END_INT_ENA_W {
        SPI_MEM_WPE_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_ena(&mut self) -> SPI_MEM_SLV_ST_END_INT_ENA_W {
        SPI_MEM_SLV_ST_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_ena(&mut self) -> SPI_MEM_MST_ST_END_INT_ENA_W {
        SPI_MEM_MST_ST_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_ena(&mut self) -> SPI_MEM_BROWN_OUT_INT_ENA_W {
        SPI_MEM_BROWN_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_ena](index.html) module"]
pub struct SPI_MEM_INT_ENA_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_int_ena::R](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_int_ena::W](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_INT_ENA to value 0"]
impl crate::Resettable for SPI_MEM_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
