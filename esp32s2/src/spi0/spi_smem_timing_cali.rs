#[doc = "Register `SPI_SMEM_TIMING_CALI` reader"]
pub type R = crate::R<SPI_SMEM_TIMING_CALI_SPEC>;
#[doc = "Register `SPI_SMEM_TIMING_CALI` writer"]
pub type W = crate::W<SPI_SMEM_TIMING_CALI_SPEC>;
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` reader - "]
pub type SPI_SMEM_TIMING_CLK_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` writer - "]
pub type SPI_SMEM_TIMING_CLK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_TIMING_CALI` reader - "]
pub type SPI_SMEM_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_TIMING_CALI` writer - "]
pub type SPI_SMEM_TIMING_CALI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` reader - "]
pub type SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` writer - "]
pub type SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(&self) -> SPI_SMEM_TIMING_CLK_ENA_R {
        SPI_SMEM_TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_smem_timing_cali(&self) -> SPI_SMEM_TIMING_CALI_R {
        SPI_SMEM_TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(&self) -> SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R {
        SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_TIMING_CALI")
            .field(
                "spi_smem_extra_dummy_cyclelen",
                &self.spi_smem_extra_dummy_cyclelen(),
            )
            .field("spi_smem_timing_cali", &self.spi_smem_timing_cali())
            .field("spi_smem_timing_clk_ena", &self.spi_smem_timing_clk_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(
        &mut self,
    ) -> SPI_SMEM_TIMING_CLK_ENA_W<SPI_SMEM_TIMING_CALI_SPEC> {
        SPI_SMEM_TIMING_CLK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_smem_timing_cali(&mut self) -> SPI_SMEM_TIMING_CALI_W<SPI_SMEM_TIMING_CALI_SPEC> {
        SPI_SMEM_TIMING_CALI_W::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(
        &mut self,
    ) -> SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W<SPI_SMEM_TIMING_CALI_SPEC> {
        SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W::new(self, 2)
    }
}
#[doc = "SPI Memory SRAM Timing Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_timing_cali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_timing_cali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for SPI_SMEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_timing_cali::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_TIMING_CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_timing_cali::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_TIMING_CALI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_TIMING_CALI to value 0"]
impl crate::Resettable for SPI_SMEM_TIMING_CALI_SPEC {}
