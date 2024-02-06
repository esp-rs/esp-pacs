#[doc = "Register `SPI_MEM_DOUT_MODE` reader"]
pub type R = crate::R<SPI_MEM_DOUT_MODE_SPEC>;
#[doc = "Register `SPI_MEM_DOUT_MODE` writer"]
pub type W = crate::W<SPI_MEM_DOUT_MODE_SPEC>;
#[doc = "Field `SPI_MEM_DOUT0_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT0_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUT0_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT0_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DOUT1_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT1_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUT1_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT1_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DOUT2_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT2_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUT2_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT2_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DOUT3_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT3_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUT3_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT3_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DOUT4_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUT4_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUT4_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUT4_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DOUT5_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUT5_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUT5_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUT5_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DOUT6_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUT6_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUT6_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUT6_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DOUT7_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUT7_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUT7_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUT7_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DOUTS_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUTS_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DOUTS_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type SPI_MEM_DOUTS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout0_mode(&self) -> SPI_MEM_DOUT0_MODE_R {
        SPI_MEM_DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout1_mode(&self) -> SPI_MEM_DOUT1_MODE_R {
        SPI_MEM_DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout2_mode(&self) -> SPI_MEM_DOUT2_MODE_R {
        SPI_MEM_DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout3_mode(&self) -> SPI_MEM_DOUT3_MODE_R {
        SPI_MEM_DOUT3_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_dout4_mode(&self) -> SPI_MEM_DOUT4_MODE_R {
        SPI_MEM_DOUT4_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_dout5_mode(&self) -> SPI_MEM_DOUT5_MODE_R {
        SPI_MEM_DOUT5_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_dout6_mode(&self) -> SPI_MEM_DOUT6_MODE_R {
        SPI_MEM_DOUT6_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_dout7_mode(&self) -> SPI_MEM_DOUT7_MODE_R {
        SPI_MEM_DOUT7_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_douts_mode(&self) -> SPI_MEM_DOUTS_MODE_R {
        SPI_MEM_DOUTS_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_DOUT_MODE")
            .field(
                "spi_mem_dout0_mode",
                &format_args!("{}", self.spi_mem_dout0_mode().bit()),
            )
            .field(
                "spi_mem_dout1_mode",
                &format_args!("{}", self.spi_mem_dout1_mode().bit()),
            )
            .field(
                "spi_mem_dout2_mode",
                &format_args!("{}", self.spi_mem_dout2_mode().bit()),
            )
            .field(
                "spi_mem_dout3_mode",
                &format_args!("{}", self.spi_mem_dout3_mode().bit()),
            )
            .field(
                "spi_mem_dout4_mode",
                &format_args!("{}", self.spi_mem_dout4_mode().bit()),
            )
            .field(
                "spi_mem_dout5_mode",
                &format_args!("{}", self.spi_mem_dout5_mode().bit()),
            )
            .field(
                "spi_mem_dout6_mode",
                &format_args!("{}", self.spi_mem_dout6_mode().bit()),
            )
            .field(
                "spi_mem_dout7_mode",
                &format_args!("{}", self.spi_mem_dout7_mode().bit()),
            )
            .field(
                "spi_mem_douts_mode",
                &format_args!("{}", self.spi_mem_douts_mode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_DOUT_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dout0_mode(&mut self) -> SPI_MEM_DOUT0_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUT0_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dout1_mode(&mut self) -> SPI_MEM_DOUT1_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUT1_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dout2_mode(&mut self) -> SPI_MEM_DOUT2_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUT2_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dout3_mode(&mut self) -> SPI_MEM_DOUT3_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUT3_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dout4_mode(&mut self) -> SPI_MEM_DOUT4_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUT4_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dout5_mode(&mut self) -> SPI_MEM_DOUT5_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUT5_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dout6_mode(&mut self) -> SPI_MEM_DOUT6_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUT6_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dout7_mode(&mut self) -> SPI_MEM_DOUT7_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUT7_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_douts_mode(&mut self) -> SPI_MEM_DOUTS_MODE_W<SPI_MEM_DOUT_MODE_SPEC> {
        SPI_MEM_DOUTS_MODE_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MSPI flash output timing adjustment control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_dout_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_dout_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_DOUT_MODE_SPEC;
impl crate::RegisterSpec for SPI_MEM_DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_dout_mode::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_DOUT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_dout_mode::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_DOUT_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_DOUT_MODE to value 0"]
impl crate::Resettable for SPI_MEM_DOUT_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
