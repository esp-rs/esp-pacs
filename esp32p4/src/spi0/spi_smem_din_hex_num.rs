#[doc = "Register `SPI_SMEM_DIN_HEX_NUM` reader"]
pub type R = crate::R<SPI_SMEM_DIN_HEX_NUM_SPEC>;
#[doc = "Register `SPI_SMEM_DIN_HEX_NUM` writer"]
pub type W = crate::W<SPI_SMEM_DIN_HEX_NUM_SPEC>;
#[doc = "Field `SPI_SMEM_DIN08_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN08_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN08_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN08_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN09_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN09_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN09_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN09_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN10_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN10_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN10_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN10_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN11_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN11_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN11_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN11_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN12_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN12_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN12_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN12_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN13_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN13_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN13_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN13_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN14_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN14_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN14_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN14_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN15_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN15_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN15_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN15_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DINS_HEX_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DINS_HEX_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DINS_HEX_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DINS_HEX_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din08_num(&self) -> SPI_SMEM_DIN08_NUM_R {
        SPI_SMEM_DIN08_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din09_num(&self) -> SPI_SMEM_DIN09_NUM_R {
        SPI_SMEM_DIN09_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din10_num(&self) -> SPI_SMEM_DIN10_NUM_R {
        SPI_SMEM_DIN10_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din11_num(&self) -> SPI_SMEM_DIN11_NUM_R {
        SPI_SMEM_DIN11_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din12_num(&self) -> SPI_SMEM_DIN12_NUM_R {
        SPI_SMEM_DIN12_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din13_num(&self) -> SPI_SMEM_DIN13_NUM_R {
        SPI_SMEM_DIN13_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din14_num(&self) -> SPI_SMEM_DIN14_NUM_R {
        SPI_SMEM_DIN14_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din15_num(&self) -> SPI_SMEM_DIN15_NUM_R {
        SPI_SMEM_DIN15_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_dins_hex_num(&self) -> SPI_SMEM_DINS_HEX_NUM_R {
        SPI_SMEM_DINS_HEX_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_DIN_HEX_NUM")
            .field("spi_smem_din08_num", &self.spi_smem_din08_num().bits())
            .field("spi_smem_din09_num", &self.spi_smem_din09_num().bits())
            .field("spi_smem_din10_num", &self.spi_smem_din10_num().bits())
            .field("spi_smem_din11_num", &self.spi_smem_din11_num().bits())
            .field("spi_smem_din12_num", &self.spi_smem_din12_num().bits())
            .field("spi_smem_din13_num", &self.spi_smem_din13_num().bits())
            .field("spi_smem_din14_num", &self.spi_smem_din14_num().bits())
            .field("spi_smem_din15_num", &self.spi_smem_din15_num().bits())
            .field(
                "spi_smem_dins_hex_num",
                &self.spi_smem_dins_hex_num().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SMEM_DIN_HEX_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din08_num(&mut self) -> SPI_SMEM_DIN08_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DIN08_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din09_num(&mut self) -> SPI_SMEM_DIN09_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DIN09_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din10_num(&mut self) -> SPI_SMEM_DIN10_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DIN10_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din11_num(&mut self) -> SPI_SMEM_DIN11_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DIN11_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din12_num(&mut self) -> SPI_SMEM_DIN12_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DIN12_NUM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din13_num(&mut self) -> SPI_SMEM_DIN13_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DIN13_NUM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din14_num(&mut self) -> SPI_SMEM_DIN14_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DIN14_NUM_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din15_num(&mut self) -> SPI_SMEM_DIN15_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DIN15_NUM_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_dins_hex_num(&mut self) -> SPI_SMEM_DINS_HEX_NUM_W<SPI_SMEM_DIN_HEX_NUM_SPEC> {
        SPI_SMEM_DINS_HEX_NUM_W::new(self, 16)
    }
}
#[doc = "MSPI 16x external RAM input timing delay number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_hex_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_din_hex_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_DIN_HEX_NUM_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DIN_HEX_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_din_hex_num::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_DIN_HEX_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_din_hex_num::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_DIN_HEX_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_SMEM_DIN_HEX_NUM to value 0"]
impl crate::Resettable for SPI_SMEM_DIN_HEX_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
