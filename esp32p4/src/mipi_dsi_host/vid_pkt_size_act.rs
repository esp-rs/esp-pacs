#[doc = "Register `VID_PKT_SIZE_ACT` reader"]
pub type R = crate::R<VID_PKT_SIZE_ACT_SPEC>;
#[doc = "Field `VID_PKT_SIZE_ACT` reader - NA"]
pub type VID_PKT_SIZE_ACT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - NA"]
    #[inline(always)]
    pub fn vid_pkt_size_act(&self) -> VID_PKT_SIZE_ACT_R {
        VID_PKT_SIZE_ACT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_PKT_SIZE_ACT")
            .field(
                "vid_pkt_size_act",
                &format_args!("{}", self.vid_pkt_size_act().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_PKT_SIZE_ACT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_pkt_size_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_PKT_SIZE_ACT_SPEC;
impl crate::RegisterSpec for VID_PKT_SIZE_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_pkt_size_act::R`](R) reader structure"]
impl crate::Readable for VID_PKT_SIZE_ACT_SPEC {}
#[doc = "`reset()` method sets VID_PKT_SIZE_ACT to value 0"]
impl crate::Resettable for VID_PKT_SIZE_ACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
