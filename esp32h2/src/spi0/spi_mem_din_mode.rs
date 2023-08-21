#[doc = "Register `SPI_MEM_DIN_MODE` reader"]
pub type R = crate::R<SPI_MEM_DIN_MODE_SPEC>;
#[doc = "Register `SPI_MEM_DIN_MODE` writer"]
pub type W = crate::W<SPI_MEM_DIN_MODE_SPEC>;
#[doc = "Field `SPI_MEM_DIN0_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN0_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN0_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN0_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DIN1_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN1_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN1_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN1_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DIN2_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN2_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN2_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN2_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DIN3_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN3_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN3_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN3_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DIN4_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DIN4_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN4_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DIN4_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DIN5_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DIN5_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN5_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DIN5_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DIN6_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DIN6_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN6_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DIN6_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DIN7_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DIN7_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN7_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DIN7_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DINS_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DINS_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DINS_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type SPI_MEM_DINS_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din0_mode(&self) -> SPI_MEM_DIN0_MODE_R {
        SPI_MEM_DIN0_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din1_mode(&self) -> SPI_MEM_DIN1_MODE_R {
        SPI_MEM_DIN1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din2_mode(&self) -> SPI_MEM_DIN2_MODE_R {
        SPI_MEM_DIN2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din3_mode(&self) -> SPI_MEM_DIN3_MODE_R {
        SPI_MEM_DIN3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_din4_mode(&self) -> SPI_MEM_DIN4_MODE_R {
        SPI_MEM_DIN4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_din5_mode(&self) -> SPI_MEM_DIN5_MODE_R {
        SPI_MEM_DIN5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_din6_mode(&self) -> SPI_MEM_DIN6_MODE_R {
        SPI_MEM_DIN6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_din7_mode(&self) -> SPI_MEM_DIN7_MODE_R {
        SPI_MEM_DIN7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn spi_mem_dins_mode(&self) -> SPI_MEM_DINS_MODE_R {
        SPI_MEM_DINS_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_DIN_MODE")
            .field(
                "spi_mem_din0_mode",
                &format_args!("{}", self.spi_mem_din0_mode().bits()),
            )
            .field(
                "spi_mem_din1_mode",
                &format_args!("{}", self.spi_mem_din1_mode().bits()),
            )
            .field(
                "spi_mem_din2_mode",
                &format_args!("{}", self.spi_mem_din2_mode().bits()),
            )
            .field(
                "spi_mem_din3_mode",
                &format_args!("{}", self.spi_mem_din3_mode().bits()),
            )
            .field(
                "spi_mem_din4_mode",
                &format_args!("{}", self.spi_mem_din4_mode().bits()),
            )
            .field(
                "spi_mem_din5_mode",
                &format_args!("{}", self.spi_mem_din5_mode().bits()),
            )
            .field(
                "spi_mem_din6_mode",
                &format_args!("{}", self.spi_mem_din6_mode().bits()),
            )
            .field(
                "spi_mem_din7_mode",
                &format_args!("{}", self.spi_mem_din7_mode().bits()),
            )
            .field(
                "spi_mem_dins_mode",
                &format_args!("{}", self.spi_mem_dins_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_DIN_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din0_mode(&mut self) -> SPI_MEM_DIN0_MODE_W<SPI_MEM_DIN_MODE_SPEC, 0> {
        SPI_MEM_DIN0_MODE_W::new(self)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din1_mode(&mut self) -> SPI_MEM_DIN1_MODE_W<SPI_MEM_DIN_MODE_SPEC, 3> {
        SPI_MEM_DIN1_MODE_W::new(self)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din2_mode(&mut self) -> SPI_MEM_DIN2_MODE_W<SPI_MEM_DIN_MODE_SPEC, 6> {
        SPI_MEM_DIN2_MODE_W::new(self)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din3_mode(&mut self) -> SPI_MEM_DIN3_MODE_W<SPI_MEM_DIN_MODE_SPEC, 9> {
        SPI_MEM_DIN3_MODE_W::new(self)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din4_mode(&mut self) -> SPI_MEM_DIN4_MODE_W<SPI_MEM_DIN_MODE_SPEC, 12> {
        SPI_MEM_DIN4_MODE_W::new(self)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din5_mode(&mut self) -> SPI_MEM_DIN5_MODE_W<SPI_MEM_DIN_MODE_SPEC, 15> {
        SPI_MEM_DIN5_MODE_W::new(self)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din6_mode(&mut self) -> SPI_MEM_DIN6_MODE_W<SPI_MEM_DIN_MODE_SPEC, 18> {
        SPI_MEM_DIN6_MODE_W::new(self)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din7_mode(&mut self) -> SPI_MEM_DIN7_MODE_W<SPI_MEM_DIN_MODE_SPEC, 21> {
        SPI_MEM_DIN7_MODE_W::new(self)
    }
    #[doc = "Bits 24:26 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dins_mode(&mut self) -> SPI_MEM_DINS_MODE_W<SPI_MEM_DIN_MODE_SPEC, 24> {
        SPI_MEM_DINS_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MSPI flash input timing delay mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_din_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_din_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_DIN_MODE_SPEC;
impl crate::RegisterSpec for SPI_MEM_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_din_mode::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_DIN_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_din_mode::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_DIN_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_DIN_MODE to value 0"]
impl crate::Resettable for SPI_MEM_DIN_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
