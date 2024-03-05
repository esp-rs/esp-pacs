#[doc = "Register `SPI_MEM_XTS_PLAIN_BASE` reader"]
pub type R = crate::R<SPI_MEM_XTS_PLAIN_BASE_SPEC>;
#[doc = "Register `SPI_MEM_XTS_PLAIN_BASE` writer"]
pub type W = crate::W<SPI_MEM_XTS_PLAIN_BASE_SPEC>;
#[doc = "Field `SPI_XTS_PLAIN` reader - This field is only used to generate include file in c case. This field is useless. Please do not use this field."]
pub type SPI_XTS_PLAIN_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_XTS_PLAIN` writer - This field is only used to generate include file in c case. This field is useless. Please do not use this field."]
pub type SPI_XTS_PLAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field is only used to generate include file in c case. This field is useless. Please do not use this field."]
    #[inline(always)]
    pub fn spi_xts_plain(&self) -> SPI_XTS_PLAIN_R {
        SPI_XTS_PLAIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_XTS_PLAIN_BASE")
            .field(
                "spi_xts_plain",
                &format_args!("{}", self.spi_xts_plain().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_XTS_PLAIN_BASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is only used to generate include file in c case. This field is useless. Please do not use this field."]
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_plain(&mut self) -> SPI_XTS_PLAIN_W<SPI_MEM_XTS_PLAIN_BASE_SPEC> {
        SPI_XTS_PLAIN_W::new(self, 0)
    }
}
#[doc = "The base address of the memory that stores plaintext in Manual Encryption\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_plain_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_plain_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_XTS_PLAIN_BASE_SPEC;
impl crate::RegisterSpec for SPI_MEM_XTS_PLAIN_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_xts_plain_base::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_XTS_PLAIN_BASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_xts_plain_base::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_XTS_PLAIN_BASE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_XTS_PLAIN_BASE to value 0"]
impl crate::Resettable for SPI_MEM_XTS_PLAIN_BASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
