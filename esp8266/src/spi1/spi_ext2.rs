#[doc = "Register `SPI_EXT2` reader"]
pub type R = crate::R<SPI_EXT2_SPEC>;
#[doc = "Register `SPI_EXT2` writer"]
pub type W = crate::W<SPI_EXT2_SPEC>;
#[doc = "Field `st` reader - "]
pub type ST_R = crate::FieldReader;
#[doc = "Field `st` writer - "]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_EXT2")
            .field("st", &format_args!("{}", self.st().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_EXT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<SPI_EXT2_SPEC> {
        ST_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ext2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ext2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_EXT2_SPEC;
impl crate::RegisterSpec for SPI_EXT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ext2::R`](R) reader structure"]
impl crate::Readable for SPI_EXT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ext2::W`](W) writer structure"]
impl crate::Writable for SPI_EXT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_EXT2 to value 0"]
impl crate::Resettable for SPI_EXT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
