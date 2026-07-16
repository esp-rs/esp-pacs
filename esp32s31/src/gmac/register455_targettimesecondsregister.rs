#[doc = "Register `REGISTER455_TARGETTIMESECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER455_TARGETTIMESECONDSREGISTER_SPEC>;
#[doc = "Register `REGISTER455_TARGETTIMESECONDSREGISTER` writer"]
pub type W = crate::W<REGISTER455_TARGETTIMESECONDSREGISTER_SPEC>;
#[doc = "Field `TSTR` reader - Target Time Seconds Register This register stores the time in seconds When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\] of Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_"]
pub type TSTR_R = crate::FieldReader<u32>;
#[doc = "Field `TSTR` writer - Target Time Seconds Register This register stores the time in seconds When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\] of Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_"]
pub type TSTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target Time Seconds Register This register stores the time in seconds When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\] of Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_"]
    #[inline(always)]
    pub fn tstr(&self) -> TSTR_R {
        TSTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER455_TARGETTIMESECONDSREGISTER")
            .field("tstr", &self.tstr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time Seconds Register This register stores the time in seconds When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\] of Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_"]
    #[inline(always)]
    pub fn tstr(&mut self) -> TSTR_W<'_, REGISTER455_TARGETTIMESECONDSREGISTER_SPEC> {
        TSTR_W::new(self, 0)
    }
}
#[doc = "Contains the higher 32 bits of time to be compared with the system time for interrupt event generation or to start the PPS signal output generation This register is present only when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register455_targettimesecondsregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register455_targettimesecondsregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER455_TARGETTIMESECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER455_TARGETTIMESECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register455_targettimesecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER455_TARGETTIMESECONDSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register455_targettimesecondsregister::W`](W) writer structure"]
impl crate::Writable for REGISTER455_TARGETTIMESECONDSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER455_TARGETTIMESECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER455_TARGETTIMESECONDSREGISTER_SPEC {}
