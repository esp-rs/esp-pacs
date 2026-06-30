#[doc = "Register `REGISTER11_AHBORAXISTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER11_AHBORAXISTATUSREGISTER_SPEC>;
#[doc = "Field `AXWHSTS` reader - AXI Master Write Channel or AHB Master Status When high, it indicates that AXI master's write channel is active and transferring data in the GMACAXI configuration In the GMACAHB configuration, it indicates that the AHB master interface FSMs are in the nonidle state"]
pub type AXWHSTS_R = crate::BitReader;
#[doc = "Field `AXIRDSTS` reader - AXI Master Read Channel Status When high, it indicates that AXI master's read channel is active and transferring data"]
pub type AXIRDSTS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AXI Master Write Channel or AHB Master Status When high, it indicates that AXI master's write channel is active and transferring data in the GMACAXI configuration In the GMACAHB configuration, it indicates that the AHB master interface FSMs are in the nonidle state"]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXI Master Read Channel Status When high, it indicates that AXI master's read channel is active and transferring data"]
    #[inline(always)]
    pub fn axirdsts(&self) -> AXIRDSTS_R {
        AXIRDSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER11_AHBORAXISTATUSREGISTER")
            .field("axwhsts", &self.axwhsts())
            .field("axirdsts", &self.axirdsts())
            .finish()
    }
}
#[doc = "Gives the idle status of the AHB master interface in the GMACAHB configuration Gives the idle status of the AXI master's read or write channel in the GMACAXI configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`register11_ahboraxistatusregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER11_AHBORAXISTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER11_AHBORAXISTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register11_ahboraxistatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER11_AHBORAXISTATUSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER11_AHBORAXISTATUSREGISTER to value 0"]
impl crate::Resettable for REGISTER11_AHBORAXISTATUSREGISTER_SPEC {}
