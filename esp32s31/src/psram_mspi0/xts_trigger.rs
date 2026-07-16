#[doc = "Register `XTS_TRIGGER` writer"]
pub type W = crate::W<XTS_TRIGGER_SPEC>;
#[doc = "Field `SPI_XTS_TRIGGER` writer - "]
pub type SPI_XTS_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTS_TRIGGER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_xts_trigger(&mut self) -> SPI_XTS_TRIGGER_W<'_, XTS_TRIGGER_SPEC> {
        SPI_XTS_TRIGGER_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTS_TRIGGER_SPEC;
impl crate::RegisterSpec for XTS_TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`xts_trigger::W`](W) writer structure"]
impl crate::Writable for XTS_TRIGGER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTS_TRIGGER to value 0"]
impl crate::Resettable for XTS_TRIGGER_SPEC {}
