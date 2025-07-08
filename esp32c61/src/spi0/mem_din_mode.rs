#[doc = "Register `MEM_DIN_MODE` reader"]
pub type R = crate::R<MEM_DIN_MODE_SPEC>;
#[doc = "Register `MEM_DIN_MODE` writer"]
pub type W = crate::W<MEM_DIN_MODE_SPEC>;
#[doc = "Field `MEM_DIN0_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type MEM_DIN0_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DIN0_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type MEM_DIN0_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DIN1_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type MEM_DIN1_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DIN1_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type MEM_DIN1_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DIN2_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type MEM_DIN2_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DIN2_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type MEM_DIN2_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DIN3_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type MEM_DIN3_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DIN3_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type MEM_DIN3_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DIN4_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DIN4_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DIN4_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DIN4_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DIN5_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DIN5_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DIN5_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DIN5_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DIN6_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DIN6_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DIN6_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DIN6_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DIN7_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DIN7_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DIN7_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DIN7_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DINS_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DINS_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_DINS_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
pub type MEM_DINS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_din0_mode(&self) -> MEM_DIN0_MODE_R {
        MEM_DIN0_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_din1_mode(&self) -> MEM_DIN1_MODE_R {
        MEM_DIN1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_din2_mode(&self) -> MEM_DIN2_MODE_R {
        MEM_DIN2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_din3_mode(&self) -> MEM_DIN3_MODE_R {
        MEM_DIN3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_din4_mode(&self) -> MEM_DIN4_MODE_R {
        MEM_DIN4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_din5_mode(&self) -> MEM_DIN5_MODE_R {
        MEM_DIN5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_din6_mode(&self) -> MEM_DIN6_MODE_R {
        MEM_DIN6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_din7_mode(&self) -> MEM_DIN7_MODE_R {
        MEM_DIN7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_dins_mode(&self) -> MEM_DINS_MODE_R {
        MEM_DINS_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_DIN_MODE")
            .field("mem_din0_mode", &self.mem_din0_mode())
            .field("mem_din1_mode", &self.mem_din1_mode())
            .field("mem_din2_mode", &self.mem_din2_mode())
            .field("mem_din3_mode", &self.mem_din3_mode())
            .field("mem_din4_mode", &self.mem_din4_mode())
            .field("mem_din5_mode", &self.mem_din5_mode())
            .field("mem_din6_mode", &self.mem_din6_mode())
            .field("mem_din7_mode", &self.mem_din7_mode())
            .field("mem_dins_mode", &self.mem_dins_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_din0_mode(&mut self) -> MEM_DIN0_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DIN0_MODE_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_din1_mode(&mut self) -> MEM_DIN1_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DIN1_MODE_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_din2_mode(&mut self) -> MEM_DIN2_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DIN2_MODE_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn mem_din3_mode(&mut self) -> MEM_DIN3_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DIN3_MODE_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_din4_mode(&mut self) -> MEM_DIN4_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DIN4_MODE_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_din5_mode(&mut self) -> MEM_DIN5_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DIN5_MODE_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_din6_mode(&mut self) -> MEM_DIN6_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DIN6_MODE_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_din7_mode(&mut self) -> MEM_DIN7_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DIN7_MODE_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk"]
    #[inline(always)]
    pub fn mem_dins_mode(&mut self) -> MEM_DINS_MODE_W<MEM_DIN_MODE_SPEC> {
        MEM_DINS_MODE_W::new(self, 24)
    }
}
#[doc = "MSPI flash input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_din_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_din_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_DIN_MODE_SPEC;
impl crate::RegisterSpec for MEM_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_din_mode::R`](R) reader structure"]
impl crate::Readable for MEM_DIN_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_din_mode::W`](W) writer structure"]
impl crate::Writable for MEM_DIN_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_DIN_MODE to value 0"]
impl crate::Resettable for MEM_DIN_MODE_SPEC {}
