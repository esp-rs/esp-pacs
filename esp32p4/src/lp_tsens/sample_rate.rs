///Register `SAMPLE_RATE` reader
pub type R = crate::R<SAMPLE_RATE_SPEC>;
///Register `SAMPLE_RATE` writer
pub type W = crate::W<SAMPLE_RATE_SPEC>;
///Field `SAMPLE_RATE` reader - Hardware automatic sampling rate.
pub type SAMPLE_RATE_R = crate::FieldReader<u16>;
///Field `SAMPLE_RATE` writer - Hardware automatic sampling rate.
pub type SAMPLE_RATE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Hardware automatic sampling rate.
    #[inline(always)]
    pub fn sample_rate(&self) -> SAMPLE_RATE_R {
        SAMPLE_RATE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAMPLE_RATE")
            .field("sample_rate", &self.sample_rate())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Hardware automatic sampling rate.
    #[inline(always)]
    #[must_use]
    pub fn sample_rate(&mut self) -> SAMPLE_RATE_W<SAMPLE_RATE_SPEC> {
        SAMPLE_RATE_W::new(self, 0)
    }
}
/**Hardware automatic sampling control registers.

You can [`read`](crate::generic::Reg::read) this register and get [`sample_rate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_rate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAMPLE_RATE_SPEC;
impl crate::RegisterSpec for SAMPLE_RATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sample_rate::R`](R) reader structure
impl crate::Readable for SAMPLE_RATE_SPEC {}
///`write(|w| ..)` method takes [`sample_rate::W`](W) writer structure
impl crate::Writable for SAMPLE_RATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAMPLE_RATE to value 0x14
impl crate::Resettable for SAMPLE_RATE_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
