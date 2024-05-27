///Register `XTS_PHYSICAL_ADDRESS` reader
pub type R = crate::R<XTS_PHYSICAL_ADDRESS_SPEC>;
///Register `XTS_PHYSICAL_ADDRESS` writer
pub type W = crate::W<XTS_PHYSICAL_ADDRESS_SPEC>;
///Field `SPI_XTS_PHYSICAL_ADDRESS` reader - This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
pub type SPI_XTS_PHYSICAL_ADDRESS_R = crate::FieldReader<u32>;
///Field `SPI_XTS_PHYSICAL_ADDRESS` writer - This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
pub type SPI_XTS_PHYSICAL_ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
    #[inline(always)]
    pub fn spi_xts_physical_address(&self) -> SPI_XTS_PHYSICAL_ADDRESS_R {
        SPI_XTS_PHYSICAL_ADDRESS_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTS_PHYSICAL_ADDRESS")
            .field("spi_xts_physical_address", &self.spi_xts_physical_address())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_physical_address(
        &mut self,
    ) -> SPI_XTS_PHYSICAL_ADDRESS_W<XTS_PHYSICAL_ADDRESS_SPEC> {
        SPI_XTS_PHYSICAL_ADDRESS_W::new(self, 0)
    }
}
/**Manual Encryption physical address register

You can [`read`](crate::generic::Reg::read) this register and get [`xts_physical_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_physical_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XTS_PHYSICAL_ADDRESS_SPEC;
impl crate::RegisterSpec for XTS_PHYSICAL_ADDRESS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`xts_physical_address::R`](R) reader structure
impl crate::Readable for XTS_PHYSICAL_ADDRESS_SPEC {}
///`write(|w| ..)` method takes [`xts_physical_address::W`](W) writer structure
impl crate::Writable for XTS_PHYSICAL_ADDRESS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets XTS_PHYSICAL_ADDRESS to value 0
impl crate::Resettable for XTS_PHYSICAL_ADDRESS_SPEC {
    const RESET_VALUE: u32 = 0;
}
