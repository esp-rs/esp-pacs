#[doc = "Register `DTXFSTS1` reader"]
pub type R = crate::R<DTXFSTS1_SPEC>;
#[doc = "Field `D_INEPTXFSPCAVAIL1` reader - "]
pub type D_INEPTXFSPCAVAIL1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfspcavail1(&self) -> D_INEPTXFSPCAVAIL1_R {
        D_INEPTXFSPCAVAIL1_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS1")
            .field(
                "d_ineptxfspcavail1",
                &format_args!("{}", self.d_ineptxfspcavail1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DTXFSTS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS1_SPEC;
impl crate::RegisterSpec for DTXFSTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts1::R`](R) reader structure"]
impl crate::Readable for DTXFSTS1_SPEC {}
#[doc = "`reset()` method sets DTXFSTS1 to value 0"]
impl crate::Resettable for DTXFSTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
