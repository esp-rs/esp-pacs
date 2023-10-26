#[doc = "Register `APP_SPI_INTR_3_MAP` reader"]
pub type R = crate::R<APP_SPI_INTR_3_MAP_SPEC>;
#[doc = "Register `APP_SPI_INTR_3_MAP` writer"]
pub type W = crate::W<APP_SPI_INTR_3_MAP_SPEC>;
#[doc = "Field `APP_SPI_INTR_3_MAP` reader - "]
pub type APP_SPI_INTR_3_MAP_R = crate::FieldReader;
#[doc = "Field `APP_SPI_INTR_3_MAP` writer - "]
pub type APP_SPI_INTR_3_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_spi_intr_3_map(&self) -> APP_SPI_INTR_3_MAP_R {
        APP_SPI_INTR_3_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_SPI_INTR_3_MAP")
            .field(
                "app_spi_intr_3_map",
                &format_args!("{}", self.app_spi_intr_3_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_SPI_INTR_3_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn app_spi_intr_3_map(&mut self) -> APP_SPI_INTR_3_MAP_W<APP_SPI_INTR_3_MAP_SPEC, 0> {
        APP_SPI_INTR_3_MAP_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_spi_intr_3_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_spi_intr_3_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_SPI_INTR_3_MAP_SPEC;
impl crate::RegisterSpec for APP_SPI_INTR_3_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_spi_intr_3_map::R`](R) reader structure"]
impl crate::Readable for APP_SPI_INTR_3_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_spi_intr_3_map::W`](W) writer structure"]
impl crate::Writable for APP_SPI_INTR_3_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APP_SPI_INTR_3_MAP to value 0x10"]
impl crate::Resettable for APP_SPI_INTR_3_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
