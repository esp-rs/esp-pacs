///Register `SIGMADELTA_MISC` reader
pub type R = crate::R<SIGMADELTA_MISC_SPEC>;
///Register `SIGMADELTA_MISC` writer
pub type W = crate::W<SIGMADELTA_MISC_SPEC>;
///Field `SPI_SWAP` reader -
pub type SPI_SWAP_R = crate::BitReader;
///Field `SPI_SWAP` writer -
pub type SPI_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31
    #[inline(always)]
    pub fn spi_swap(&self) -> SPI_SWAP_R {
        SPI_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA_MISC")
            .field("spi_swap", &self.spi_swap())
            .finish()
    }
}
impl W {
    ///Bit 31
    #[inline(always)]
    #[must_use]
    pub fn spi_swap(&mut self) -> SPI_SWAP_W<SIGMADELTA_MISC_SPEC> {
        SPI_SWAP_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SIGMADELTA_MISC_SPEC;
impl crate::RegisterSpec for SIGMADELTA_MISC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sigmadelta_misc::R`](R) reader structure
impl crate::Readable for SIGMADELTA_MISC_SPEC {}
///`write(|w| ..)` method takes [`sigmadelta_misc::W`](W) writer structure
impl crate::Writable for SIGMADELTA_MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SIGMADELTA_MISC to value 0
impl crate::Resettable for SIGMADELTA_MISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
