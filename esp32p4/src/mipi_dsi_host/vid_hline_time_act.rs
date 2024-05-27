#[doc = "Register `VID_HLINE_TIME_ACT` reader"]
pub type R = crate::R<VID_HLINE_TIME_ACT_SPEC>;
#[doc = "Field `VID_HLINE_TIME_ACT` reader - NA"]
pub type VID_HLINE_TIME_ACT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn vid_hline_time_act(&self) -> VID_HLINE_TIME_ACT_R {
        VID_HLINE_TIME_ACT_R::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_HLINE_TIME_ACT")
            .field("vid_hline_time_act", &self.vid_hline_time_act())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_hline_time_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_HLINE_TIME_ACT_SPEC;
impl crate::RegisterSpec for VID_HLINE_TIME_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hline_time_act::R`](R) reader structure"]
impl crate::Readable for VID_HLINE_TIME_ACT_SPEC {}
#[doc = "`reset()` method sets VID_HLINE_TIME_ACT to value 0"]
impl crate::Resettable for VID_HLINE_TIME_ACT_SPEC {
    const RESET_VALUE: u32 = 0;
}
