#[doc = "Register `REGISTER456_TARGETTIMENANOSECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC>;
#[doc = "Register `REGISTER456_TARGETTIMENANOSECONDSREGISTER` writer"]
pub type W = crate::W<REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC>;
#[doc = "Field `TTSLO` reader - Target Timestamp Low Register This register stores the time in _signed_ nanoseconds When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field _Bits \\[6:5\\]_ in Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_ This value should not exceed 0x3B9A_C9FF when Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_ The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value"]
pub type TTSLO_R = crate::FieldReader<u32>;
#[doc = "Field `TTSLO` writer - Target Timestamp Low Register This register stores the time in _signed_ nanoseconds When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field _Bits \\[6:5\\]_ in Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_ This value should not exceed 0x3B9A_C9FF when Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_ The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value"]
pub type TTSLO_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TRGTBUSY` reader - Target Time Register Busy The MAC sets this bit when the PPSCMD field _Bit \\[3:0\\]_ in Register 459 _PPS Control Register_ is programmed to 010 or 011 Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1 Otherwise, the synchronization of the previous programmed time gets corrupted This bit is reserved when the Enable Flexible PulsePerSecond Output feature is not selected"]
pub type TRGTBUSY_R = crate::BitReader;
#[doc = "Field `TRGTBUSY` writer - Target Time Register Busy The MAC sets this bit when the PPSCMD field _Bit \\[3:0\\]_ in Register 459 _PPS Control Register_ is programmed to 010 or 011 Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1 Otherwise, the synchronization of the previous programmed time gets corrupted This bit is reserved when the Enable Flexible PulsePerSecond Output feature is not selected"]
pub type TRGTBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register This register stores the time in _signed_ nanoseconds When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field _Bits \\[6:5\\]_ in Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_ This value should not exceed 0x3B9A_C9FF when Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_ The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value"]
    #[inline(always)]
    pub fn ttslo(&self) -> TTSLO_R {
        TTSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Target Time Register Busy The MAC sets this bit when the PPSCMD field _Bit \\[3:0\\]_ in Register 459 _PPS Control Register_ is programmed to 010 or 011 Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1 Otherwise, the synchronization of the previous programmed time gets corrupted This bit is reserved when the Enable Flexible PulsePerSecond Output feature is not selected"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TRGTBUSY_R {
        TRGTBUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER456_TARGETTIMENANOSECONDSREGISTER")
            .field("ttslo", &self.ttslo())
            .field("trgtbusy", &self.trgtbusy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register This register stores the time in _signed_ nanoseconds When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field _Bits \\[6:5\\]_ in Register 459 _PPS Control Register_, the MAC starts or stops the PPS signal output and generates an interrupt _if enabled_ This value should not exceed 0x3B9A_C9FF when Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_ The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value"]
    #[inline(always)]
    pub fn ttslo(&mut self) -> TTSLO_W<'_, REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC> {
        TTSLO_W::new(self, 0)
    }
    #[doc = "Bit 31 - Target Time Register Busy The MAC sets this bit when the PPSCMD field _Bit \\[3:0\\]_ in Register 459 _PPS Control Register_ is programmed to 010 or 011 Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1 Otherwise, the synchronization of the previous programmed time gets corrupted This bit is reserved when the Enable Flexible PulsePerSecond Output feature is not selected"]
    #[inline(always)]
    pub fn trgtbusy(&mut self) -> TRGTBUSY_W<'_, REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC> {
        TRGTBUSY_W::new(self, 31)
    }
}
#[doc = "Contains the lower 32 bits of time to be compared with the system time for interrupt event generation or to start the PPS signal output generation This register is present only when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register456_targettimenanosecondsregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register456_targettimenanosecondsregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register456_targettimenanosecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register456_targettimenanosecondsregister::W`](W) writer structure"]
impl crate::Writable for REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER456_TARGETTIMENANOSECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC {}
