#[doc = "Register `SPI_MEM_XTS_PHYSICAL_ADDRESS` reader"]
pub type R = crate::R<SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC>;
#[doc = "Register `SPI_MEM_XTS_PHYSICAL_ADDRESS` writer"]
pub type W = crate::W<SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC>;
#[doc = "Field `SPI_XTS_PHYSICAL_ADDRESS` reader - This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter."]
pub type SPI_XTS_PHYSICAL_ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_XTS_PHYSICAL_ADDRESS` writer - This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter."]
pub type SPI_XTS_PHYSICAL_ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter."]
    #[inline(always)]
    pub fn spi_xts_physical_address(&self) -> SPI_XTS_PHYSICAL_ADDRESS_R {
        SPI_XTS_PHYSICAL_ADDRESS_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_XTS_PHYSICAL_ADDRESS")
            .field(
                "spi_xts_physical_address",
                &format_args!("{}", self.spi_xts_physical_address().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:25 - This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter."]
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_physical_address(
        &mut self,
    ) -> SPI_XTS_PHYSICAL_ADDRESS_W<SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC> {
        SPI_XTS_PHYSICAL_ADDRESS_W::new(self, 0)
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
#[doc = "Manual Encryption physical address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_physical_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_physical_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC;
impl crate::RegisterSpec for SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_xts_physical_address::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_xts_physical_address::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_XTS_PHYSICAL_ADDRESS to value 0"]
impl crate::Resettable for SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
