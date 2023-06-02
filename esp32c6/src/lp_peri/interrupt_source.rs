#[doc = "Register `INTERRUPT_SOURCE` reader"]
pub struct R(crate::R<INTERRUPT_SOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_SOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_SOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_SOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LP_INTERRUPT_SOURCE` reader - BIT5~BIT0: pmu_lp_int, modem_lp_int, lp_timer_lp_int, lp_uart_int, lp_i2c_int, lp_io_int"]
pub type LP_INTERRUPT_SOURCE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - BIT5~BIT0: pmu_lp_int, modem_lp_int, lp_timer_lp_int, lp_uart_int, lp_i2c_int, lp_io_int"]
    #[inline(always)]
    pub fn lp_interrupt_source(&self) -> LP_INTERRUPT_SOURCE_R {
        LP_INTERRUPT_SOURCE_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_SOURCE")
            .field(
                "lp_interrupt_source",
                &format_args!("{}", self.lp_interrupt_source().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRUPT_SOURCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_source](index.html) module"]
pub struct INTERRUPT_SOURCE_SPEC;
impl crate::RegisterSpec for INTERRUPT_SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_source::R](R) reader structure"]
impl crate::Readable for INTERRUPT_SOURCE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERRUPT_SOURCE to value 0"]
impl crate::Resettable for INTERRUPT_SOURCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
