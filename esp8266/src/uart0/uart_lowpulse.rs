#[doc = "Register `UART_LOWPULSE` reader"]
pub struct R(crate::R<UART_LOWPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LOWPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_LOWPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_LOWPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `lowpulse_min_cnt` reader - used in baudrate detect"]
pub type LOWPULSE_MIN_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - used in baudrate detect"]
    #[inline(always)]
    pub fn lowpulse_min_cnt(&self) -> LOWPULSE_MIN_CNT_R {
        LOWPULSE_MIN_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "UART_LOWPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_lowpulse](index.html) module"]
pub struct UART_LOWPULSE_SPEC;
impl crate::RegisterSpec for UART_LOWPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_lowpulse::R](R) reader structure"]
impl crate::Readable for UART_LOWPULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_LOWPULSE to value 0"]
impl crate::Resettable for UART_LOWPULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
