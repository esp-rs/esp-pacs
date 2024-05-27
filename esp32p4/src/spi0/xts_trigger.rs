///Register `XTS_TRIGGER` writer
pub type W = crate::W<XTS_TRIGGER_SPEC>;
///Field `SPI_XTS_TRIGGER` writer - Set this bit to trigger the process of manual encryption calculation. This action should only be asserted when manual encryption status is 0. After this action, manual encryption status becomes 1. After calculation is done, manual encryption status becomes 2.
pub type SPI_XTS_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTS_TRIGGER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to trigger the process of manual encryption calculation. This action should only be asserted when manual encryption status is 0. After this action, manual encryption status becomes 1. After calculation is done, manual encryption status becomes 2.
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_trigger(&mut self) -> SPI_XTS_TRIGGER_W<XTS_TRIGGER_SPEC> {
        SPI_XTS_TRIGGER_W::new(self, 0)
    }
}
/**Manual Encryption physical address register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XTS_TRIGGER_SPEC;
impl crate::RegisterSpec for XTS_TRIGGER_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`xts_trigger::W`](W) writer structure
impl crate::Writable for XTS_TRIGGER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets XTS_TRIGGER to value 0
impl crate::Resettable for XTS_TRIGGER_SPEC {
    const RESET_VALUE: u32 = 0;
}
