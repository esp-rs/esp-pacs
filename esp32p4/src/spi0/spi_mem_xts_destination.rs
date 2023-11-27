#[doc = "Register `SPI_MEM_XTS_DESTINATION` reader"]
pub type R = crate::R<SPI_MEM_XTS_DESTINATION_SPEC>;
#[doc = "Register `SPI_MEM_XTS_DESTINATION` writer"]
pub type W = crate::W<SPI_MEM_XTS_DESTINATION_SPEC>;
#[doc = "Field `SPI_XTS_DESTINATION` reader - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
pub type SPI_XTS_DESTINATION_R = crate::BitReader;
#[doc = "Field `SPI_XTS_DESTINATION` writer - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
pub type SPI_XTS_DESTINATION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
    #[inline(always)]
    pub fn spi_xts_destination(&self) -> SPI_XTS_DESTINATION_R {
        SPI_XTS_DESTINATION_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_XTS_DESTINATION")
            .field(
                "spi_xts_destination",
                &format_args!("{}", self.spi_xts_destination().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_XTS_DESTINATION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_destination(&mut self) -> SPI_XTS_DESTINATION_W<SPI_MEM_XTS_DESTINATION_SPEC> {
        SPI_XTS_DESTINATION_W::new(self, 0)
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
#[doc = "Manual Encryption destination register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_destination::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_destination::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_XTS_DESTINATION_SPEC;
impl crate::RegisterSpec for SPI_MEM_XTS_DESTINATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_xts_destination::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_XTS_DESTINATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_xts_destination::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_XTS_DESTINATION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_XTS_DESTINATION to value 0"]
impl crate::Resettable for SPI_MEM_XTS_DESTINATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
