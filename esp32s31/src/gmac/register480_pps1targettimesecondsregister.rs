#[doc = "Register `REGISTER480_PPS1TARGETTIMESECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC>;
#[doc = "Register `REGISTER480_PPS1TARGETTIMESECONDSREGISTER` writer"]
pub type W = crate::W<REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC>;
#[doc = "Field `TSTRH1` reader - PPS1 Target Time Seconds Register This register stores the time in seconds When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_"]
pub type TSTRH1_R = crate::FieldReader<u32>;
#[doc = "Field `TSTRH1` writer - PPS1 Target Time Seconds Register This register stores the time in seconds When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_"]
pub type TSTRH1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPS1 Target Time Seconds Register This register stores the time in seconds When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_"]
    #[inline(always)]
    pub fn tstrh1(&self) -> TSTRH1_R {
        TSTRH1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER480_PPS1TARGETTIMESECONDSREGISTER")
            .field("tstrh1", &self.tstrh1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS1 Target Time Seconds Register This register stores the time in seconds When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_"]
    #[inline(always)]
    pub fn tstrh1(&mut self) -> TSTRH1_W<'_, REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC> {
        TSTRH1_W::new(self, 0)
    }
}
#[doc = "Contains the higher 32 bits of time to be compared with the system time to generate the interrupt event or to start generating the PPS1 output signal This register is present only when IEEE1588 timestamping is enabled without an external timestamp input and at least one additional PPS output is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register480_pps1targettimesecondsregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register480_pps1targettimesecondsregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register480_pps1targettimesecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register480_pps1targettimesecondsregister::W`](W) writer structure"]
impl crate::Writable for REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER480_PPS1TARGETTIMESECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC {}
