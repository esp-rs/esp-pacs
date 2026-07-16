#[doc = "Register `SPI_SMEM_S_DOUT_HEX_MODE` reader"]
pub type R = crate::R<SPI_SMEM_S_DOUT_HEX_MODE_SPEC>;
#[doc = "Register `SPI_SMEM_S_DOUT_HEX_MODE` writer"]
pub type W = crate::W<SPI_SMEM_S_DOUT_HEX_MODE_SPEC>;
#[doc = "Field `SPI_SMEM_S_DOUT08_MODE` reader - "]
pub type SPI_SMEM_S_DOUT08_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUT08_MODE` writer - "]
pub type SPI_SMEM_S_DOUT08_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_DOUT09_MODE` reader - "]
pub type SPI_SMEM_S_DOUT09_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUT09_MODE` writer - "]
pub type SPI_SMEM_S_DOUT09_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_DOUT10_MODE` reader - "]
pub type SPI_SMEM_S_DOUT10_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUT10_MODE` writer - "]
pub type SPI_SMEM_S_DOUT10_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_DOUT11_MODE` reader - "]
pub type SPI_SMEM_S_DOUT11_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUT11_MODE` writer - "]
pub type SPI_SMEM_S_DOUT11_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_DOUT12_MODE` reader - "]
pub type SPI_SMEM_S_DOUT12_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUT12_MODE` writer - "]
pub type SPI_SMEM_S_DOUT12_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_DOUT13_MODE` reader - "]
pub type SPI_SMEM_S_DOUT13_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUT13_MODE` writer - "]
pub type SPI_SMEM_S_DOUT13_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_DOUT14_MODE` reader - "]
pub type SPI_SMEM_S_DOUT14_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUT14_MODE` writer - "]
pub type SPI_SMEM_S_DOUT14_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_DOUT15_MODE` reader - "]
pub type SPI_SMEM_S_DOUT15_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUT15_MODE` writer - "]
pub type SPI_SMEM_S_DOUT15_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_DOUTS_HEX_MODE` reader - "]
pub type SPI_SMEM_S_DOUTS_HEX_MODE_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_DOUTS_HEX_MODE` writer - "]
pub type SPI_SMEM_S_DOUTS_HEX_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_smem_s_dout08_mode(&self) -> SPI_SMEM_S_DOUT08_MODE_R {
        SPI_SMEM_S_DOUT08_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_smem_s_dout09_mode(&self) -> SPI_SMEM_S_DOUT09_MODE_R {
        SPI_SMEM_S_DOUT09_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_smem_s_dout10_mode(&self) -> SPI_SMEM_S_DOUT10_MODE_R {
        SPI_SMEM_S_DOUT10_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_smem_s_dout11_mode(&self) -> SPI_SMEM_S_DOUT11_MODE_R {
        SPI_SMEM_S_DOUT11_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_smem_s_dout12_mode(&self) -> SPI_SMEM_S_DOUT12_MODE_R {
        SPI_SMEM_S_DOUT12_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_smem_s_dout13_mode(&self) -> SPI_SMEM_S_DOUT13_MODE_R {
        SPI_SMEM_S_DOUT13_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_smem_s_dout14_mode(&self) -> SPI_SMEM_S_DOUT14_MODE_R {
        SPI_SMEM_S_DOUT14_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_smem_s_dout15_mode(&self) -> SPI_SMEM_S_DOUT15_MODE_R {
        SPI_SMEM_S_DOUT15_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_smem_s_douts_hex_mode(&self) -> SPI_SMEM_S_DOUTS_HEX_MODE_R {
        SPI_SMEM_S_DOUTS_HEX_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_S_DOUT_HEX_MODE")
            .field("spi_smem_s_dout08_mode", &self.spi_smem_s_dout08_mode())
            .field("spi_smem_s_dout09_mode", &self.spi_smem_s_dout09_mode())
            .field("spi_smem_s_dout10_mode", &self.spi_smem_s_dout10_mode())
            .field("spi_smem_s_dout11_mode", &self.spi_smem_s_dout11_mode())
            .field("spi_smem_s_dout12_mode", &self.spi_smem_s_dout12_mode())
            .field("spi_smem_s_dout13_mode", &self.spi_smem_s_dout13_mode())
            .field("spi_smem_s_dout14_mode", &self.spi_smem_s_dout14_mode())
            .field("spi_smem_s_dout15_mode", &self.spi_smem_s_dout15_mode())
            .field(
                "spi_smem_s_douts_hex_mode",
                &self.spi_smem_s_douts_hex_mode(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_smem_s_dout08_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUT08_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUT08_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_smem_s_dout09_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUT09_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUT09_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_smem_s_dout10_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUT10_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUT10_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_smem_s_dout11_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUT11_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUT11_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_smem_s_dout12_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUT12_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUT12_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_smem_s_dout13_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUT13_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUT13_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_smem_s_dout14_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUT14_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUT14_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_smem_s_dout15_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUT15_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUT15_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_smem_s_douts_hex_mode(
        &mut self,
    ) -> SPI_SMEM_S_DOUTS_HEX_MODE_W<'_, SPI_SMEM_S_DOUT_HEX_MODE_SPEC> {
        SPI_SMEM_S_DOUTS_HEX_MODE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_dout_hex_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_dout_hex_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_S_DOUT_HEX_MODE_SPEC;
impl crate::RegisterSpec for SPI_SMEM_S_DOUT_HEX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_s_dout_hex_mode::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_S_DOUT_HEX_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_s_dout_hex_mode::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_S_DOUT_HEX_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_S_DOUT_HEX_MODE to value 0"]
impl crate::Resettable for SPI_SMEM_S_DOUT_HEX_MODE_SPEC {}
