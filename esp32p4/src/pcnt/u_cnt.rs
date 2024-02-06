#[doc = "Register `U%s_CNT` reader"]
pub type R = crate::R<U_CNT_SPEC>;
#[doc = "Field `PULSE_CNT_U` reader - This register stores the current pulse count value for unit %s."]
pub type PULSE_CNT_U_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit %s."]
    #[inline(always)]
    pub fn pulse_cnt_u(&self) -> PULSE_CNT_U_R {
        PULSE_CNT_U_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_CNT")
            .field(
                "pulse_cnt_u",
                &format_args!("{}", self.pulse_cnt_u().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<U_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Counter value for unit %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U_CNT_SPEC;
impl crate::RegisterSpec for U_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_cnt::R`](R) reader structure"]
impl crate::Readable for U_CNT_SPEC {}
#[doc = "`reset()` method sets U%s_CNT to value 0"]
impl crate::Resettable for U_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
