#[doc = "Register `SPI_SMEM_DIN_MODE` reader"]
pub type R = crate::R<SPI_SMEM_DIN_MODE_SPEC>;
#[doc = "Register `SPI_SMEM_DIN_MODE` writer"]
pub type W = crate::W<SPI_SMEM_DIN_MODE_SPEC>;
#[doc = "Field `SPI_SMEM_DIN0_MODE` reader - "]
pub type SPI_SMEM_DIN0_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN0_MODE` writer - "]
pub type SPI_SMEM_DIN0_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DIN1_MODE` reader - "]
pub type SPI_SMEM_DIN1_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN1_MODE` writer - "]
pub type SPI_SMEM_DIN1_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DIN2_MODE` reader - "]
pub type SPI_SMEM_DIN2_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN2_MODE` writer - "]
pub type SPI_SMEM_DIN2_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DIN3_MODE` reader - "]
pub type SPI_SMEM_DIN3_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN3_MODE` writer - "]
pub type SPI_SMEM_DIN3_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DIN4_MODE` reader - "]
pub type SPI_SMEM_DIN4_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN4_MODE` writer - "]
pub type SPI_SMEM_DIN4_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DIN5_MODE` reader - "]
pub type SPI_SMEM_DIN5_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN5_MODE` writer - "]
pub type SPI_SMEM_DIN5_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DIN6_MODE` reader - "]
pub type SPI_SMEM_DIN6_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN6_MODE` writer - "]
pub type SPI_SMEM_DIN6_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DIN7_MODE` reader - "]
pub type SPI_SMEM_DIN7_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN7_MODE` writer - "]
pub type SPI_SMEM_DIN7_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DINS_MODE` reader - "]
pub type SPI_SMEM_DINS_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DINS_MODE` writer - "]
pub type SPI_SMEM_DINS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn spi_smem_din0_mode(&self) -> SPI_SMEM_DIN0_MODE_R {
        SPI_SMEM_DIN0_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn spi_smem_din1_mode(&self) -> SPI_SMEM_DIN1_MODE_R {
        SPI_SMEM_DIN1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn spi_smem_din2_mode(&self) -> SPI_SMEM_DIN2_MODE_R {
        SPI_SMEM_DIN2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn spi_smem_din3_mode(&self) -> SPI_SMEM_DIN3_MODE_R {
        SPI_SMEM_DIN3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn spi_smem_din4_mode(&self) -> SPI_SMEM_DIN4_MODE_R {
        SPI_SMEM_DIN4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn spi_smem_din5_mode(&self) -> SPI_SMEM_DIN5_MODE_R {
        SPI_SMEM_DIN5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_smem_din6_mode(&self) -> SPI_SMEM_DIN6_MODE_R {
        SPI_SMEM_DIN6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn spi_smem_din7_mode(&self) -> SPI_SMEM_DIN7_MODE_R {
        SPI_SMEM_DIN7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn spi_smem_dins_mode(&self) -> SPI_SMEM_DINS_MODE_R {
        SPI_SMEM_DINS_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_DIN_MODE")
            .field("spi_smem_dins_mode", &self.spi_smem_dins_mode())
            .field("spi_smem_din7_mode", &self.spi_smem_din7_mode())
            .field("spi_smem_din6_mode", &self.spi_smem_din6_mode())
            .field("spi_smem_din5_mode", &self.spi_smem_din5_mode())
            .field("spi_smem_din4_mode", &self.spi_smem_din4_mode())
            .field("spi_smem_din3_mode", &self.spi_smem_din3_mode())
            .field("spi_smem_din2_mode", &self.spi_smem_din2_mode())
            .field("spi_smem_din1_mode", &self.spi_smem_din1_mode())
            .field("spi_smem_din0_mode", &self.spi_smem_din0_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn spi_smem_din0_mode(&mut self) -> SPI_SMEM_DIN0_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN0_MODE_W::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn spi_smem_din1_mode(&mut self) -> SPI_SMEM_DIN1_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN1_MODE_W::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn spi_smem_din2_mode(&mut self) -> SPI_SMEM_DIN2_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN2_MODE_W::new(self, 6)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn spi_smem_din3_mode(&mut self) -> SPI_SMEM_DIN3_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN3_MODE_W::new(self, 9)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn spi_smem_din4_mode(&mut self) -> SPI_SMEM_DIN4_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN4_MODE_W::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn spi_smem_din5_mode(&mut self) -> SPI_SMEM_DIN5_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN5_MODE_W::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_smem_din6_mode(&mut self) -> SPI_SMEM_DIN6_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN6_MODE_W::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn spi_smem_din7_mode(&mut self) -> SPI_SMEM_DIN7_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN7_MODE_W::new(self, 21)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn spi_smem_dins_mode(&mut self) -> SPI_SMEM_DINS_MODE_W<'_, SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DINS_MODE_W::new(self, 24)
    }
}
#[doc = "SPI Memory SRAM Data In Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_din_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_DIN_MODE_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_din_mode::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_DIN_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_din_mode::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_DIN_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_DIN_MODE to value 0"]
impl crate::Resettable for SPI_SMEM_DIN_MODE_SPEC {}
