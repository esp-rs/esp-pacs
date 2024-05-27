///Register `XTS_DATE` reader
pub type R = crate::R<XTS_DATE_SPEC>;
///Register `XTS_DATE` writer
pub type W = crate::W<XTS_DATE_SPEC>;
///Field `SPI_XTS_DATE` reader - This bits stores the last modified-time of manual encryption feature.
pub type SPI_XTS_DATE_R = crate::FieldReader<u32>;
///Field `SPI_XTS_DATE` writer - This bits stores the last modified-time of manual encryption feature.
pub type SPI_XTS_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - This bits stores the last modified-time of manual encryption feature.
    #[inline(always)]
    pub fn spi_xts_date(&self) -> SPI_XTS_DATE_R {
        SPI_XTS_DATE_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTS_DATE")
            .field("spi_xts_date", &self.spi_xts_date())
            .finish()
    }
}
impl W {
    ///Bits 0:29 - This bits stores the last modified-time of manual encryption feature.
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_date(&mut self) -> SPI_XTS_DATE_W<XTS_DATE_SPEC> {
        SPI_XTS_DATE_W::new(self, 0)
    }
}
/**Manual Encryption version register

You can [`read`](crate::generic::Reg::read) this register and get [`xts_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XTS_DATE_SPEC;
impl crate::RegisterSpec for XTS_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`xts_date::R`](R) reader structure
impl crate::Readable for XTS_DATE_SPEC {}
///`write(|w| ..)` method takes [`xts_date::W`](W) writer structure
impl crate::Writable for XTS_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets XTS_DATE to value 0x2020_1010
impl crate::Resettable for XTS_DATE_SPEC {
    const RESET_VALUE: u32 = 0x2020_1010;
}
