#[doc = "Register `MEM_DOUT_MODE` reader"]
pub type R = crate::R<MEM_DOUT_MODE_SPEC>;
#[doc = "Register `MEM_DOUT_MODE` writer"]
pub type W = crate::W<MEM_DOUT_MODE_SPEC>;
#[doc = "Field `MEM_DOUT0_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type MEM_DOUT0_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUT0_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type MEM_DOUT0_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DOUT1_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type MEM_DOUT1_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUT1_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type MEM_DOUT1_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DOUT2_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type MEM_DOUT2_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUT2_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type MEM_DOUT2_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DOUT3_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type MEM_DOUT3_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUT3_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type MEM_DOUT3_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DOUT4_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUT4_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUT4_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUT4_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DOUT5_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUT5_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUT5_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUT5_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DOUT6_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUT6_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUT6_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUT6_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DOUT7_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUT7_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUT7_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUT7_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DOUTS_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUTS_MODE_R = crate::BitReader;
#[doc = "Field `MEM_DOUTS_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type MEM_DOUTS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_dout0_mode(&self) -> MEM_DOUT0_MODE_R {
        MEM_DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_dout1_mode(&self) -> MEM_DOUT1_MODE_R {
        MEM_DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_dout2_mode(&self) -> MEM_DOUT2_MODE_R {
        MEM_DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_dout3_mode(&self) -> MEM_DOUT3_MODE_R {
        MEM_DOUT3_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_dout4_mode(&self) -> MEM_DOUT4_MODE_R {
        MEM_DOUT4_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_dout5_mode(&self) -> MEM_DOUT5_MODE_R {
        MEM_DOUT5_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_dout6_mode(&self) -> MEM_DOUT6_MODE_R {
        MEM_DOUT6_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_dout7_mode(&self) -> MEM_DOUT7_MODE_R {
        MEM_DOUT7_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_douts_mode(&self) -> MEM_DOUTS_MODE_R {
        MEM_DOUTS_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_DOUT_MODE")
            .field("mem_dout0_mode", &self.mem_dout0_mode())
            .field("mem_dout1_mode", &self.mem_dout1_mode())
            .field("mem_dout2_mode", &self.mem_dout2_mode())
            .field("mem_dout3_mode", &self.mem_dout3_mode())
            .field("mem_dout4_mode", &self.mem_dout4_mode())
            .field("mem_dout5_mode", &self.mem_dout5_mode())
            .field("mem_dout6_mode", &self.mem_dout6_mode())
            .field("mem_dout7_mode", &self.mem_dout7_mode())
            .field("mem_douts_mode", &self.mem_douts_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_dout0_mode(&mut self) -> MEM_DOUT0_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUT0_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_dout1_mode(&mut self) -> MEM_DOUT1_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUT1_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_dout2_mode(&mut self) -> MEM_DOUT2_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUT2_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_dout3_mode(&mut self) -> MEM_DOUT3_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUT3_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_dout4_mode(&mut self) -> MEM_DOUT4_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUT4_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_dout5_mode(&mut self) -> MEM_DOUT5_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUT5_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_dout6_mode(&mut self) -> MEM_DOUT6_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUT6_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_dout7_mode(&mut self) -> MEM_DOUT7_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUT7_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn mem_douts_mode(&mut self) -> MEM_DOUTS_MODE_W<MEM_DOUT_MODE_SPEC> {
        MEM_DOUTS_MODE_W::new(self, 8)
    }
}
#[doc = "MSPI flash output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_dout_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_dout_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_DOUT_MODE_SPEC;
impl crate::RegisterSpec for MEM_DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_dout_mode::R`](R) reader structure"]
impl crate::Readable for MEM_DOUT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_dout_mode::W`](W) writer structure"]
impl crate::Writable for MEM_DOUT_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_DOUT_MODE to value 0"]
impl crate::Resettable for MEM_DOUT_MODE_SPEC {}
