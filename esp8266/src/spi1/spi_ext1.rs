#[doc = "Register `SPI_EXT1` reader"]
pub type R = crate::R<SPI_EXT1_SPEC>;
#[doc = "Register `SPI_EXT1` writer"]
pub type W = crate::W<SPI_EXT1_SPEC>;
#[doc = "Field `erase_time` reader - "]
pub type ERASE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `erase_time` writer - "]
pub type ERASE_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `erase_shift` reader - "]
pub type ERASE_SHIFT_R = crate::FieldReader;
#[doc = "Field `erase_shift` writer - "]
pub type ERASE_SHIFT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `erase_enable` reader - "]
pub type ERASE_ENABLE_R = crate::BitReader;
#[doc = "Field `erase_enable` writer - "]
pub type ERASE_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn erase_time(&self) -> ERASE_TIME_R {
        ERASE_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn erase_shift(&self) -> ERASE_SHIFT_R {
        ERASE_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn erase_enable(&self) -> ERASE_ENABLE_R {
        ERASE_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_EXT1")
            .field(
                "erase_enable",
                &format_args!("{}", self.erase_enable().bit()),
            )
            .field(
                "erase_shift",
                &format_args!("{}", self.erase_shift().bits()),
            )
            .field("erase_time", &format_args!("{}", self.erase_time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_EXT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn erase_time(&mut self) -> ERASE_TIME_W<SPI_EXT1_SPEC, 0> {
        ERASE_TIME_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn erase_shift(&mut self) -> ERASE_SHIFT_W<SPI_EXT1_SPEC, 16> {
        ERASE_SHIFT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn erase_enable(&mut self) -> ERASE_ENABLE_W<SPI_EXT1_SPEC, 31> {
        ERASE_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ext1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ext1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_EXT1_SPEC;
impl crate::RegisterSpec for SPI_EXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ext1::R`](R) reader structure"]
impl crate::Readable for SPI_EXT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ext1::W`](W) writer structure"]
impl crate::Writable for SPI_EXT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_EXT1 to value 0"]
impl crate::Resettable for SPI_EXT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
