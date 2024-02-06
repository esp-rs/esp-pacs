#[doc = "Register `RECIVE_SEQ` reader"]
pub type R = crate::R<RECIVE_SEQ_SPEC>;
#[doc = "Field `RECIVE_SEQ` reader - High speed sdio pad bist recive sequence"]
pub type RECIVE_SEQ_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - High speed sdio pad bist recive sequence"]
    #[inline(always)]
    pub fn recive_seq(&self) -> RECIVE_SEQ_R {
        RECIVE_SEQ_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RECIVE_SEQ")
            .field("recive_seq", &format_args!("{}", self.recive_seq().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RECIVE_SEQ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "High speed sdio pad bist recive sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`recive_seq::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECIVE_SEQ_SPEC;
impl crate::RegisterSpec for RECIVE_SEQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`recive_seq::R`](R) reader structure"]
impl crate::Readable for RECIVE_SEQ_SPEC {}
#[doc = "`reset()` method sets RECIVE_SEQ to value 0"]
impl crate::Resettable for RECIVE_SEQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
