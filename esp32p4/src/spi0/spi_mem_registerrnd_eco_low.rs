#[doc = "Register `SPI_MEM_REGISTERRND_ECO_LOW` reader"]
pub type R = crate::R<SPI_MEM_REGISTERRND_ECO_LOW_SPEC>;
#[doc = "Register `SPI_MEM_REGISTERRND_ECO_LOW` writer"]
pub type W = crate::W<SPI_MEM_REGISTERRND_ECO_LOW_SPEC>;
#[doc = "Field `SPI_MEM_REGISTERRND_ECO_LOW` reader - ECO low register"]
pub type SPI_MEM_REGISTERRND_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MEM_REGISTERRND_ECO_LOW` writer - ECO low register"]
pub type SPI_MEM_REGISTERRND_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECO low register"]
    #[inline(always)]
    pub fn spi_mem_registerrnd_eco_low(&self) -> SPI_MEM_REGISTERRND_ECO_LOW_R {
        SPI_MEM_REGISTERRND_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_REGISTERRND_ECO_LOW")
            .field(
                "spi_mem_registerrnd_eco_low",
                &format_args!("{}", self.spi_mem_registerrnd_eco_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_REGISTERRND_ECO_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECO low register"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_registerrnd_eco_low(
        &mut self,
    ) -> SPI_MEM_REGISTERRND_ECO_LOW_W<SPI_MEM_REGISTERRND_ECO_LOW_SPEC> {
        SPI_MEM_REGISTERRND_ECO_LOW_W::new(self, 0)
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
#[doc = "MSPI ECO low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_registerrnd_eco_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_registerrnd_eco_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_REGISTERRND_ECO_LOW_SPEC;
impl crate::RegisterSpec for SPI_MEM_REGISTERRND_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_registerrnd_eco_low::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_REGISTERRND_ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_registerrnd_eco_low::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_REGISTERRND_ECO_LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_REGISTERRND_ECO_LOW to value 0x037c"]
impl crate::Resettable for SPI_MEM_REGISTERRND_ECO_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0x037c;
}
