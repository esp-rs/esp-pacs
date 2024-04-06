#[doc = "Register `DTXFSTS` reader"]
pub type R = crate::R<DTXFSTS_SPEC>;
#[doc = "Field `INEPTXFSPCAVAIL` reader - "]
pub type INEPTXFSPCAVAIL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ineptxfspcavail(&self) -> INEPTXFSPCAVAIL_R {
        INEPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS")
            .field(
                "ineptxfspcavail",
                &format_args!("{}", self.ineptxfspcavail().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DTXFSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS_SPEC;
impl crate::RegisterSpec for DTXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts::R`](R) reader structure"]
impl crate::Readable for DTXFSTS_SPEC {}
#[doc = "`reset()` method sets DTXFSTS to value 0"]
impl crate::Resettable for DTXFSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
