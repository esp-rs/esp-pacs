#[doc = "Register `SPI_CACHE` reader"]
pub type R = crate::R<SPI_CACHE_SPEC>;
#[doc = "Register `SPI_CACHE` writer"]
pub type W = crate::W<SPI_CACHE_SPEC>;
#[doc = "Field `cache_flush_start` reader - Flush cache"]
pub type CACHE_FLUSH_START_R = crate::BitReader;
#[doc = "Field `cache_flush_start` writer - Flush cache"]
pub type CACHE_FLUSH_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cache_empty` reader - Cache is empty"]
pub type CACHE_EMPTY_R = crate::BitReader;
#[doc = "Field `cache_empty` writer - Cache is empty"]
pub type CACHE_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cache_enable` reader - Cache enable"]
pub type CACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `cache_enable` writer - Cache enable"]
pub type CACHE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `busy` reader - SPI busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `busy` writer - SPI busy"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `block` reader - Flash memory block to map, in 2mb blocks"]
pub type BLOCK_R = crate::FieldReader;
#[doc = "Field `block` writer - Flash memory block to map, in 2mb blocks"]
pub type BLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `offset` reader - Offset within block to map, in megabytes"]
pub type OFFSET_R = crate::FieldReader;
#[doc = "Field `offset` writer - Offset within block to map, in megabytes"]
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `target` reader - Controls where the spi flash is mapped (unconfirmed)"]
pub type TARGET_R = crate::BitReader;
#[doc = "Field `target` writer - Controls where the spi flash is mapped (unconfirmed)"]
pub type TARGET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flush cache"]
    #[inline(always)]
    pub fn cache_flush_start(&self) -> CACHE_FLUSH_START_R {
        CACHE_FLUSH_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cache is empty"]
    #[inline(always)]
    pub fn cache_empty(&self) -> CACHE_EMPTY_R {
        CACHE_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Cache enable"]
    #[inline(always)]
    pub fn cache_enable(&self) -> CACHE_ENABLE_R {
        CACHE_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Flash memory block to map, in 2mb blocks"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Offset within block to map, in megabytes"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Controls where the spi flash is mapped (unconfirmed)"]
    #[inline(always)]
    pub fn target(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CACHE")
            .field(
                "cache_flush_start",
                &format_args!("{}", self.cache_flush_start().bit()),
            )
            .field("cache_empty", &format_args!("{}", self.cache_empty().bit()))
            .field(
                "cache_enable",
                &format_args!("{}", self.cache_enable().bit()),
            )
            .field("busy", &format_args!("{}", self.busy().bit()))
            .field("block", &format_args!("{}", self.block().bits()))
            .field("offset", &format_args!("{}", self.offset().bits()))
            .field("target", &format_args!("{}", self.target().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CACHE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Flush cache"]
    #[inline(always)]
    #[must_use]
    pub fn cache_flush_start(&mut self) -> CACHE_FLUSH_START_W<SPI_CACHE_SPEC> {
        CACHE_FLUSH_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Cache is empty"]
    #[inline(always)]
    #[must_use]
    pub fn cache_empty(&mut self) -> CACHE_EMPTY_W<SPI_CACHE_SPEC> {
        CACHE_EMPTY_W::new(self, 1)
    }
    #[doc = "Bit 8 - Cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn cache_enable(&mut self) -> CACHE_ENABLE_W<SPI_CACHE_SPEC> {
        CACHE_ENABLE_W::new(self, 8)
    }
    #[doc = "Bit 9 - SPI busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<SPI_CACHE_SPEC> {
        BUSY_W::new(self, 9)
    }
    #[doc = "Bits 16:18 - Flash memory block to map, in 2mb blocks"]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<SPI_CACHE_SPEC> {
        BLOCK_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Offset within block to map, in megabytes"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<SPI_CACHE_SPEC> {
        OFFSET_W::new(self, 24)
    }
    #[doc = "Bit 26 - Controls where the spi flash is mapped (unconfirmed)"]
    #[inline(always)]
    #[must_use]
    pub fn target(&mut self) -> TARGET_W<SPI_CACHE_SPEC> {
        TARGET_W::new(self, 26)
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
#[doc = "Controls SPI memory-mapped caching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cache::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cache::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CACHE_SPEC;
impl crate::RegisterSpec for SPI_CACHE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cache::R`](R) reader structure"]
impl crate::Readable for SPI_CACHE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_cache::W`](W) writer structure"]
impl crate::Writable for SPI_CACHE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CACHE to value 0"]
impl crate::Resettable for SPI_CACHE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
