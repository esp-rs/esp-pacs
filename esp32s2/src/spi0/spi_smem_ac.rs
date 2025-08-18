#[doc = "Register `SPI_SMEM_AC` reader"]
pub type R = crate::R<SPI_SMEM_AC_SPEC>;
#[doc = "Register `SPI_SMEM_AC` writer"]
pub type W = crate::W<SPI_SMEM_AC_SPEC>;
#[doc = "Field `SPI_SMEM_CS_SETUP` reader - "]
pub type SPI_SMEM_CS_SETUP_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_CS_SETUP` writer - "]
pub type SPI_SMEM_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_CS_HOLD` reader - "]
pub type SPI_SMEM_CS_HOLD_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_CS_HOLD` writer - "]
pub type SPI_SMEM_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_CS_SETUP_TIME` reader - "]
pub type SPI_SMEM_CS_SETUP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_SMEM_CS_SETUP_TIME` writer - "]
pub type SPI_SMEM_CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SPI_SMEM_CS_HOLD_TIME` reader - "]
pub type SPI_SMEM_CS_HOLD_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_SMEM_CS_HOLD_TIME` writer - "]
pub type SPI_SMEM_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_smem_cs_setup(&self) -> SPI_SMEM_CS_SETUP_R {
        SPI_SMEM_CS_SETUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_smem_cs_hold(&self) -> SPI_SMEM_CS_HOLD_R {
        SPI_SMEM_CS_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:14"]
    #[inline(always)]
    pub fn spi_smem_cs_setup_time(&self) -> SPI_SMEM_CS_SETUP_TIME_R {
        SPI_SMEM_CS_SETUP_TIME_R::new(((self.bits >> 2) & 0x1fff) as u16)
    }
    #[doc = "Bits 15:27"]
    #[inline(always)]
    pub fn spi_smem_cs_hold_time(&self) -> SPI_SMEM_CS_HOLD_TIME_R {
        SPI_SMEM_CS_HOLD_TIME_R::new(((self.bits >> 15) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_AC")
            .field("spi_smem_cs_hold_time", &self.spi_smem_cs_hold_time())
            .field("spi_smem_cs_setup_time", &self.spi_smem_cs_setup_time())
            .field("spi_smem_cs_hold", &self.spi_smem_cs_hold())
            .field("spi_smem_cs_setup", &self.spi_smem_cs_setup())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_smem_cs_setup(&mut self) -> SPI_SMEM_CS_SETUP_W<'_, SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_SETUP_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_smem_cs_hold(&mut self) -> SPI_SMEM_CS_HOLD_W<'_, SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_HOLD_W::new(self, 1)
    }
    #[doc = "Bits 2:14"]
    #[inline(always)]
    pub fn spi_smem_cs_setup_time(&mut self) -> SPI_SMEM_CS_SETUP_TIME_W<'_, SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_SETUP_TIME_W::new(self, 2)
    }
    #[doc = "Bits 15:27"]
    #[inline(always)]
    pub fn spi_smem_cs_hold_time(&mut self) -> SPI_SMEM_CS_HOLD_TIME_W<'_, SPI_SMEM_AC_SPEC> {
        SPI_SMEM_CS_HOLD_TIME_W::new(self, 15)
    }
}
#[doc = "SPI Memory SRAM Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_AC_SPEC;
impl crate::RegisterSpec for SPI_SMEM_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_ac::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_AC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_ac::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_AC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_AC to value 0"]
impl crate::Resettable for SPI_SMEM_AC_SPEC {}
