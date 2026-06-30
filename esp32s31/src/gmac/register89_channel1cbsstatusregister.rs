#[doc = "Register `REGISTER89_CHANNEL1CBSSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER89_CHANNEL1CBSSTATUSREGISTER_SPEC>;
#[doc = "Field `CH1_ABS` reader - Average Bits per Slot This field contains the average transmitted bits per slot This field is computed over programmed number of slots _SLC bits in the CBS Control Register_ for Channel 1 traffic The maximum value is 0x30D4 for 100 Mbps and 0x1E848 for 1000 Mbps"]
pub type CH1_ABS_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_ABSU` reader - ABS Updated When set, this bit indicates that the MAC has updated the ABS value This bit is cleared when the application reads the ABS value"]
pub type CH1_ABSU_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:16 - Average Bits per Slot This field contains the average transmitted bits per slot This field is computed over programmed number of slots _SLC bits in the CBS Control Register_ for Channel 1 traffic The maximum value is 0x30D4 for 100 Mbps and 0x1E848 for 1000 Mbps"]
    #[inline(always)]
    pub fn ch1_abs(&self) -> CH1_ABS_R {
        CH1_ABS_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - ABS Updated When set, this bit indicates that the MAC has updated the ABS value This bit is cleared when the application reads the ABS value"]
    #[inline(always)]
    pub fn ch1_absu(&self) -> CH1_ABSU_R {
        CH1_ABSU_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER89_CHANNEL1CBSSTATUSREGISTER")
            .field("ch1_abs", &self.ch1_abs())
            .field("ch1_absu", &self.ch1_absu())
            .finish()
    }
}
#[doc = "Provides the average traffic transmitted in Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register89_channel1cbsstatusregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER89_CHANNEL1CBSSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER89_CHANNEL1CBSSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register89_channel1cbsstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER89_CHANNEL1CBSSTATUSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER89_CHANNEL1CBSSTATUSREGISTER to value 0"]
impl crate::Resettable for REGISTER89_CHANNEL1CBSSTATUSREGISTER_SPEC {}
