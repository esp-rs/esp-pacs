#[doc = "Register `UART_HIGHPULSE` reader"]
pub struct R(crate::R<UART_HIGHPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_HIGHPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_HIGHPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_HIGHPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `highpulse_min_cnt` reader - used in baudrate detect"]
pub type HIGHPULSE_MIN_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - used in baudrate detect"]
    #[inline(always)]
    pub fn highpulse_min_cnt(&self) -> HIGHPULSE_MIN_CNT_R {
        HIGHPULSE_MIN_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_HIGHPULSE")
            .field(
                "highpulse_min_cnt",
                &format_args!("{}", self.highpulse_min_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_HIGHPULSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "UART_HIGHPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_highpulse](index.html) module"]
pub struct UART_HIGHPULSE_SPEC;
impl crate::RegisterSpec for UART_HIGHPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_highpulse::R](R) reader structure"]
impl crate::Readable for UART_HIGHPULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_HIGHPULSE to value 0"]
impl crate::Resettable for UART_HIGHPULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
