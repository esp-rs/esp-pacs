#[doc = "Register `ID0` reader"]
pub type R = crate::R<ID0_SPEC>;
#[doc = "Field `DMAC_ID` reader - NA"]
pub type DMAC_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dmac_id(&self) -> DMAC_ID_R {
        DMAC_ID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID0")
            .field("dmac_id", &self.dmac_id().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ID0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID0_SPEC;
impl crate::RegisterSpec for ID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id0::R`](R) reader structure"]
impl crate::Readable for ID0_SPEC {}
#[doc = "`reset()` method sets ID0 to value 0"]
impl crate::Resettable for ID0_SPEC {
    const RESET_VALUE: u32 = 0;
}
