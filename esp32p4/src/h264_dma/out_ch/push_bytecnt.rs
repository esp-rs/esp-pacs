#[doc = "Register `PUSH_BYTECNT` reader"]
pub type R = crate::R<PUSH_BYTECNT_SPEC>;
#[doc = "Field `OUT_CMDFIFO_PUSH_BYTECNT` reader - only for debug"]
pub type OUT_CMDFIFO_PUSH_BYTECNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - only for debug"]
    #[inline(always)]
    pub fn out_cmdfifo_push_bytecnt(&self) -> OUT_CMDFIFO_PUSH_BYTECNT_R {
        OUT_CMDFIFO_PUSH_BYTECNT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUSH_BYTECNT")
            .field(
                "out_cmdfifo_push_bytecnt",
                &self.out_cmdfifo_push_bytecnt().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PUSH_BYTECNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TX CHx push byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`push_bytecnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUSH_BYTECNT_SPEC;
impl crate::RegisterSpec for PUSH_BYTECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`push_bytecnt::R`](R) reader structure"]
impl crate::Readable for PUSH_BYTECNT_SPEC {}
#[doc = "`reset()` method sets PUSH_BYTECNT to value 0xff"]
impl crate::Resettable for PUSH_BYTECNT_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
