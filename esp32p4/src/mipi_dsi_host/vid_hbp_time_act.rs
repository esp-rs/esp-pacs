#[doc = "Register `VID_HBP_TIME_ACT` reader"]
pub type R = crate::R<VID_HBP_TIME_ACT_SPEC>;
#[doc = "Field `VID_HBP_TIME_ACT` reader - NA"]
pub type VID_HBP_TIME_ACT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - NA"]
    #[inline(always)]
    pub fn vid_hbp_time_act(&self) -> VID_HBP_TIME_ACT_R {
        VID_HBP_TIME_ACT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_HBP_TIME_ACT")
            .field("vid_hbp_time_act", &self.vid_hbp_time_act())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_hbp_time_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_HBP_TIME_ACT_SPEC;
impl crate::RegisterSpec for VID_HBP_TIME_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hbp_time_act::R`](R) reader structure"]
impl crate::Readable for VID_HBP_TIME_ACT_SPEC {}
#[doc = "`reset()` method sets VID_HBP_TIME_ACT to value 0"]
impl crate::Resettable for VID_HBP_TIME_ACT_SPEC {
    const RESET_VALUE: u32 = 0;
}
