#[doc = "Register `SPI_CACHE_TARGET` reader"]
pub type R = crate::R<SPI_CACHE_TARGET_SPEC>;
#[doc = "Register `SPI_CACHE_TARGET` writer"]
pub type W = crate::W<SPI_CACHE_TARGET_SPEC>;
#[doc = "Field `target1` reader - "]
pub type TARGET1_R = crate::BitReader;
#[doc = "Field `target1` writer - "]
pub type TARGET1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `target2` reader - "]
pub type TARGET2_R = crate::BitReader;
#[doc = "Field `target2` writer - "]
pub type TARGET2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn target1(&self) -> TARGET1_R {
        TARGET1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn target2(&self) -> TARGET2_R {
        TARGET2_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CACHE_TARGET")
            .field("target1", &format_args!("{}", self.target1().bit()))
            .field("target2", &format_args!("{}", self.target2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CACHE_TARGET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn target1(&mut self) -> TARGET1_W<SPI_CACHE_TARGET_SPEC, 3> {
        TARGET1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn target2(&mut self) -> TARGET2_W<SPI_CACHE_TARGET_SPEC, 4> {
        TARGET2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control where the cache is mapped (unconfirmed)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cache_target::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cache_target::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CACHE_TARGET_SPEC;
impl crate::RegisterSpec for SPI_CACHE_TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cache_target::R`](R) reader structure"]
impl crate::Readable for SPI_CACHE_TARGET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_cache_target::W`](W) writer structure"]
impl crate::Writable for SPI_CACHE_TARGET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CACHE_TARGET to value 0"]
impl crate::Resettable for SPI_CACHE_TARGET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
