#[doc = "Register `GSNPSID` reader"]
pub type R = crate::R<GSNPSID_SPEC>;
#[doc = "Field `SYNOPSYSID` reader - "]
pub type SYNOPSYSID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn synopsysid(&self) -> SYNOPSYSID_R {
        SYNOPSYSID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GSNPSID")
            .field("synopsysid", &self.synopsysid().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GSNPSID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GSNPSID_SPEC;
impl crate::RegisterSpec for GSNPSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsnpsid::R`](R) reader structure"]
impl crate::Readable for GSNPSID_SPEC {}
#[doc = "`reset()` method sets GSNPSID to value 0x4f54_400a"]
impl crate::Resettable for GSNPSID_SPEC {
    const RESET_VALUE: u32 = 0x4f54_400a;
}
