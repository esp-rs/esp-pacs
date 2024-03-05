#[doc = "Register `SPI_INTR_2_MAP` reader"]
pub type R = crate::R<SPI_INTR_2_MAP_SPEC>;
#[doc = "Register `SPI_INTR_2_MAP` writer"]
pub type W = crate::W<SPI_INTR_2_MAP_SPEC>;
#[doc = "Field `SPI_INTR_2_MAP` reader - reg_core0_spi_intr_2_map"]
pub type SPI_INTR_2_MAP_R = crate::FieldReader;
#[doc = "Field `SPI_INTR_2_MAP` writer - reg_core0_spi_intr_2_map"]
pub type SPI_INTR_2_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_spi_intr_2_map"]
    #[inline(always)]
    pub fn spi_intr_2_map(&self) -> SPI_INTR_2_MAP_R {
        SPI_INTR_2_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_INTR_2_MAP")
            .field(
                "spi_intr_2_map",
                &format_args!("{}", self.spi_intr_2_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_INTR_2_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_spi_intr_2_map"]
    #[inline(always)]
    #[must_use]
    pub fn spi_intr_2_map(&mut self) -> SPI_INTR_2_MAP_W<SPI_INTR_2_MAP_SPEC> {
        SPI_INTR_2_MAP_W::new(self, 0)
    }
}
#[doc = "spi1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_2_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_2_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_INTR_2_MAP_SPEC;
impl crate::RegisterSpec for SPI_INTR_2_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_intr_2_map::R`](R) reader structure"]
impl crate::Readable for SPI_INTR_2_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_intr_2_map::W`](W) writer structure"]
impl crate::Writable for SPI_INTR_2_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_INTR_2_MAP to value 0"]
impl crate::Resettable for SPI_INTR_2_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
