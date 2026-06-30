#[doc = "Register `REGISTER454_TIMESTAMPADDENDREGISTER` reader"]
pub type R = crate::R<REGISTER454_TIMESTAMPADDENDREGISTER_SPEC>;
#[doc = "Register `REGISTER454_TIMESTAMPADDENDREGISTER` writer"]
pub type W = crate::W<REGISTER454_TIMESTAMPADDENDREGISTER_SPEC>;
#[doc = "Field `TSAR` reader - Timestamp Addend Register This field indicates the 32bit time value to be added to the Accumulator register to achieve time synchronization"]
pub type TSAR_R = crate::FieldReader<u32>;
#[doc = "Field `TSAR` writer - Timestamp Addend Register This field indicates the 32bit time value to be added to the Accumulator register to achieve time synchronization"]
pub type TSAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register This field indicates the 32bit time value to be added to the Accumulator register to achieve time synchronization"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER454_TIMESTAMPADDENDREGISTER")
            .field("tsar", &self.tsar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register This field indicates the 32bit time value to be added to the Accumulator register to achieve time synchronization"]
    #[inline(always)]
    pub fn tsar(&mut self) -> TSAR_W<'_, REGISTER454_TIMESTAMPADDENDREGISTER_SPEC> {
        TSAR_W::new(self, 0)
    }
}
#[doc = "This register is used by the software to readjust the clock frequency linearly to match the master clock frequency This register is only present when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register454_timestampaddendregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register454_timestampaddendregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER454_TIMESTAMPADDENDREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER454_TIMESTAMPADDENDREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register454_timestampaddendregister::R`](R) reader structure"]
impl crate::Readable for REGISTER454_TIMESTAMPADDENDREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register454_timestampaddendregister::W`](W) writer structure"]
impl crate::Writable for REGISTER454_TIMESTAMPADDENDREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER454_TIMESTAMPADDENDREGISTER to value 0"]
impl crate::Resettable for REGISTER454_TIMESTAMPADDENDREGISTER_SPEC {}
