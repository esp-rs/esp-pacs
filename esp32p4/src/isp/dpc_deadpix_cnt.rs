#[doc = "Register `DPC_DEADPIX_CNT` reader"]
pub type R = crate::R<DPC_DEADPIX_CNT_SPEC>;
#[doc = "Field `DPC_DEADPIX_CNT` reader - this field represents the dead pixel count"]
pub type DPC_DEADPIX_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - this field represents the dead pixel count"]
    #[inline(always)]
    pub fn dpc_deadpix_cnt(&self) -> DPC_DEADPIX_CNT_R {
        DPC_DEADPIX_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPC_DEADPIX_CNT")
            .field("dpc_deadpix_cnt", &self.dpc_deadpix_cnt().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPC_DEADPIX_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "DPC dead-pix number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpc_deadpix_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPC_DEADPIX_CNT_SPEC;
impl crate::RegisterSpec for DPC_DEADPIX_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpc_deadpix_cnt::R`](R) reader structure"]
impl crate::Readable for DPC_DEADPIX_CNT_SPEC {}
#[doc = "`reset()` method sets DPC_DEADPIX_CNT to value 0"]
impl crate::Resettable for DPC_DEADPIX_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
