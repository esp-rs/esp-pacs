#[doc = "Register `REGISTER153_CHANNEL2CBSSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER153_CHANNEL2CBSSTATUSREGISTER_SPEC>;
#[doc = "Field `CH2_ABS` reader - Average Bits per Slot This field contains the average transmitted bits per slot This field is computed over programmed number of slots _SLC bits in the CBS Control Register_ for Channel 2 traffic The maximum value is 0x30D4 for 100 Mbps and 0x1E848 for 1000 Mbps"]
pub type CH2_ABS_R = crate::FieldReader<u32>;
#[doc = "Field `CH2_ABSU` reader - ABS Updated When set, this bit indicates that the MAC has updated the ABS value This bit is cleared when the application reads the ABS value"]
pub type CH2_ABSU_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:16 - Average Bits per Slot This field contains the average transmitted bits per slot This field is computed over programmed number of slots _SLC bits in the CBS Control Register_ for Channel 2 traffic The maximum value is 0x30D4 for 100 Mbps and 0x1E848 for 1000 Mbps"]
    #[inline(always)]
    pub fn ch2_abs(&self) -> CH2_ABS_R {
        CH2_ABS_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - ABS Updated When set, this bit indicates that the MAC has updated the ABS value This bit is cleared when the application reads the ABS value"]
    #[inline(always)]
    pub fn ch2_absu(&self) -> CH2_ABSU_R {
        CH2_ABSU_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER153_CHANNEL2CBSSTATUSREGISTER")
            .field("ch2_abs", &self.ch2_abs())
            .field("ch2_absu", &self.ch2_absu())
            .finish()
    }
}
#[doc = "Provides the average traffic transmitted in Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register153_channel2cbsstatusregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER153_CHANNEL2CBSSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER153_CHANNEL2CBSSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register153_channel2cbsstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER153_CHANNEL2CBSSTATUSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER153_CHANNEL2CBSSTATUSREGISTER to value 0"]
impl crate::Resettable for REGISTER153_CHANNEL2CBSSTATUSREGISTER_SPEC {}
