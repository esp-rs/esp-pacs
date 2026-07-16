#[doc = "Register `SPI_SMEM_S_DIN_HEX_MODE` reader"]
pub type R = crate::R<SPI_SMEM_S_DIN_HEX_MODE_SPEC>;
#[doc = "Register `SPI_SMEM_S_DIN_HEX_MODE` writer"]
pub type W = crate::W<SPI_SMEM_S_DIN_HEX_MODE_SPEC>;
#[doc = "Field `SPI_SMEM_S_DIN08_MODE` reader - "]
pub type SPI_SMEM_S_DIN08_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DIN08_MODE` writer - "]
pub type SPI_SMEM_S_DIN08_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_DIN09_MODE` reader - "]
pub type SPI_SMEM_S_DIN09_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DIN09_MODE` writer - "]
pub type SPI_SMEM_S_DIN09_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_DIN10_MODE` reader - "]
pub type SPI_SMEM_S_DIN10_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DIN10_MODE` writer - "]
pub type SPI_SMEM_S_DIN10_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_DIN11_MODE` reader - "]
pub type SPI_SMEM_S_DIN11_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DIN11_MODE` writer - "]
pub type SPI_SMEM_S_DIN11_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_DIN12_MODE` reader - "]
pub type SPI_SMEM_S_DIN12_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DIN12_MODE` writer - "]
pub type SPI_SMEM_S_DIN12_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_DIN13_MODE` reader - "]
pub type SPI_SMEM_S_DIN13_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DIN13_MODE` writer - "]
pub type SPI_SMEM_S_DIN13_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_DIN14_MODE` reader - "]
pub type SPI_SMEM_S_DIN14_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DIN14_MODE` writer - "]
pub type SPI_SMEM_S_DIN14_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_DIN15_MODE` reader - "]
pub type SPI_SMEM_S_DIN15_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DIN15_MODE` writer - "]
pub type SPI_SMEM_S_DIN15_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_DINS_HEX_MODE` reader - "]
pub type SPI_SMEM_S_DINS_HEX_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_DINS_HEX_MODE` writer - "]
pub type SPI_SMEM_S_DINS_HEX_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn spi_smem_s_din08_mode(&self) -> SPI_SMEM_S_DIN08_MODE_R {
        SPI_SMEM_S_DIN08_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn spi_smem_s_din09_mode(&self) -> SPI_SMEM_S_DIN09_MODE_R {
        SPI_SMEM_S_DIN09_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn spi_smem_s_din10_mode(&self) -> SPI_SMEM_S_DIN10_MODE_R {
        SPI_SMEM_S_DIN10_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn spi_smem_s_din11_mode(&self) -> SPI_SMEM_S_DIN11_MODE_R {
        SPI_SMEM_S_DIN11_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn spi_smem_s_din12_mode(&self) -> SPI_SMEM_S_DIN12_MODE_R {
        SPI_SMEM_S_DIN12_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn spi_smem_s_din13_mode(&self) -> SPI_SMEM_S_DIN13_MODE_R {
        SPI_SMEM_S_DIN13_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_smem_s_din14_mode(&self) -> SPI_SMEM_S_DIN14_MODE_R {
        SPI_SMEM_S_DIN14_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn spi_smem_s_din15_mode(&self) -> SPI_SMEM_S_DIN15_MODE_R {
        SPI_SMEM_S_DIN15_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn spi_smem_s_dins_hex_mode(&self) -> SPI_SMEM_S_DINS_HEX_MODE_R {
        SPI_SMEM_S_DINS_HEX_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_S_DIN_HEX_MODE")
            .field("spi_smem_s_din08_mode", &self.spi_smem_s_din08_mode())
            .field("spi_smem_s_din09_mode", &self.spi_smem_s_din09_mode())
            .field("spi_smem_s_din10_mode", &self.spi_smem_s_din10_mode())
            .field("spi_smem_s_din11_mode", &self.spi_smem_s_din11_mode())
            .field("spi_smem_s_din12_mode", &self.spi_smem_s_din12_mode())
            .field("spi_smem_s_din13_mode", &self.spi_smem_s_din13_mode())
            .field("spi_smem_s_din14_mode", &self.spi_smem_s_din14_mode())
            .field("spi_smem_s_din15_mode", &self.spi_smem_s_din15_mode())
            .field("spi_smem_s_dins_hex_mode", &self.spi_smem_s_dins_hex_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn spi_smem_s_din08_mode(
        &mut self,
    ) -> SPI_SMEM_S_DIN08_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DIN08_MODE_W::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn spi_smem_s_din09_mode(
        &mut self,
    ) -> SPI_SMEM_S_DIN09_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DIN09_MODE_W::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn spi_smem_s_din10_mode(
        &mut self,
    ) -> SPI_SMEM_S_DIN10_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DIN10_MODE_W::new(self, 6)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn spi_smem_s_din11_mode(
        &mut self,
    ) -> SPI_SMEM_S_DIN11_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DIN11_MODE_W::new(self, 9)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn spi_smem_s_din12_mode(
        &mut self,
    ) -> SPI_SMEM_S_DIN12_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DIN12_MODE_W::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn spi_smem_s_din13_mode(
        &mut self,
    ) -> SPI_SMEM_S_DIN13_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DIN13_MODE_W::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_smem_s_din14_mode(
        &mut self,
    ) -> SPI_SMEM_S_DIN14_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DIN14_MODE_W::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn spi_smem_s_din15_mode(
        &mut self,
    ) -> SPI_SMEM_S_DIN15_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DIN15_MODE_W::new(self, 21)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn spi_smem_s_dins_hex_mode(
        &mut self,
    ) -> SPI_SMEM_S_DINS_HEX_MODE_W<'_, SPI_SMEM_S_DIN_HEX_MODE_SPEC> {
        SPI_SMEM_S_DINS_HEX_MODE_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_din_hex_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_din_hex_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_S_DIN_HEX_MODE_SPEC;
impl crate::RegisterSpec for SPI_SMEM_S_DIN_HEX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_s_din_hex_mode::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_S_DIN_HEX_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_s_din_hex_mode::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_S_DIN_HEX_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_S_DIN_HEX_MODE to value 0"]
impl crate::Resettable for SPI_SMEM_S_DIN_HEX_MODE_SPEC {}
