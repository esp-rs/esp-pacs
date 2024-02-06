#[doc = "Register `POSPULSE` reader"]
pub type R = crate::R<POSPULSE_SPEC>;
#[doc = "Field `POSEDGE_MIN_CNT` reader - This register stores the minimal input clock count between two positive edges. It is used in boudrate-detect process."]
pub type POSEDGE_MIN_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - This register stores the minimal input clock count between two positive edges. It is used in boudrate-detect process."]
    #[inline(always)]
    pub fn posedge_min_cnt(&self) -> POSEDGE_MIN_CNT_R {
        POSEDGE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POSPULSE")
            .field(
                "posedge_min_cnt",
                &format_args!("{}", self.posedge_min_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POSPULSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Autobaud high pulse register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pospulse::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POSPULSE_SPEC;
impl crate::RegisterSpec for POSPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pospulse::R`](R) reader structure"]
impl crate::Readable for POSPULSE_SPEC {}
#[doc = "`reset()` method sets POSPULSE to value 0x0fff"]
impl crate::Resettable for POSPULSE_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
