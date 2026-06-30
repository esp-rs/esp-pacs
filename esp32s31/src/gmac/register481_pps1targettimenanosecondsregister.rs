#[doc = "Register `REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC>;
#[doc = "Register `REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER` writer"]
pub type W = crate::W<REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC>;
#[doc = "Field `TTSL1` reader - Target Time Low for PPS1 Register This register stores the time in _signed_ nanoseconds When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field _Bits \\[14:13\\]_ in Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_ This value should not exceed 0x3B9A_C9FF when Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_ The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value"]
pub type TTSL1_R = crate::FieldReader<u32>;
#[doc = "Field `TTSL1` writer - Target Time Low for PPS1 Register This register stores the time in _signed_ nanoseconds When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field _Bits \\[14:13\\]_ in Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_ This value should not exceed 0x3B9A_C9FF when Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_ The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value"]
pub type TTSL1_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TRGTBUSY1` reader - PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field _Bits \\[10:8\\]_ in Register 459 _PPS Control Register_ is programmed to 010 or 011 Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1 Otherwise, the synchronization of the previous programmed time gets corrupted"]
pub type TRGTBUSY1_R = crate::BitReader;
#[doc = "Field `TRGTBUSY1` writer - PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field _Bits \\[10:8\\]_ in Register 459 _PPS Control Register_ is programmed to 010 or 011 Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1 Otherwise, the synchronization of the previous programmed time gets corrupted"]
pub type TRGTBUSY1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Target Time Low for PPS1 Register This register stores the time in _signed_ nanoseconds When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field _Bits \\[14:13\\]_ in Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_ This value should not exceed 0x3B9A_C9FF when Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_ The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value"]
    #[inline(always)]
    pub fn ttsl1(&self) -> TTSL1_R {
        TTSL1_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field _Bits \\[10:8\\]_ in Register 459 _PPS Control Register_ is programmed to 010 or 011 Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1 Otherwise, the synchronization of the previous programmed time gets corrupted"]
    #[inline(always)]
    pub fn trgtbusy1(&self) -> TRGTBUSY1_R {
        TRGTBUSY1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER")
            .field("ttsl1", &self.ttsl1())
            .field("trgtbusy1", &self.trgtbusy1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time Low for PPS1 Register This register stores the time in _signed_ nanoseconds When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field _Bits \\[14:13\\]_ in Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_ This value should not exceed 0x3B9A_C9FF when Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_ The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value"]
    #[inline(always)]
    pub fn ttsl1(&mut self) -> TTSL1_W<'_, REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC> {
        TTSL1_W::new(self, 0)
    }
    #[doc = "Bit 31 - PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field _Bits \\[10:8\\]_ in Register 459 _PPS Control Register_ is programmed to 010 or 011 Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1 Otherwise, the synchronization of the previous programmed time gets corrupted"]
    #[inline(always)]
    pub fn trgtbusy1(
        &mut self,
    ) -> TRGTBUSY1_W<'_, REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC> {
        TRGTBUSY1_W::new(self, 31)
    }
}
#[doc = "Contains the lower 32 bits of time to be compared with the system time to generate the interrupt event or to start generating the PPS1 output signal This register is present only when IEEE1588 timestamping is enabled without an external timestamp input and at least one additional PPS output is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register481_pps1targettimenanosecondsregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register481_pps1targettimenanosecondsregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register481_pps1targettimenanosecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register481_pps1targettimenanosecondsregister::W`](W) writer structure"]
impl crate::Writable for REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC {}
