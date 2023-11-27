#[doc = "Register `VID_VBP_LINES_ACT` reader"]
pub type R = crate::R<VID_VBP_LINES_ACT_SPEC>;
#[doc = "Field `VBP_LINES_ACT` reader - NA"]
pub type VBP_LINES_ACT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn vbp_lines_act(&self) -> VBP_LINES_ACT_R {
        VBP_LINES_ACT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_VBP_LINES_ACT")
            .field(
                "vbp_lines_act",
                &format_args!("{}", self.vbp_lines_act().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_VBP_LINES_ACT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vbp_lines_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_VBP_LINES_ACT_SPEC;
impl crate::RegisterSpec for VID_VBP_LINES_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vbp_lines_act::R`](R) reader structure"]
impl crate::Readable for VID_VBP_LINES_ACT_SPEC {}
#[doc = "`reset()` method sets VID_VBP_LINES_ACT to value 0"]
impl crate::Resettable for VID_VBP_LINES_ACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
