#[doc = "Register `T2QNR` writer"]
pub type W = crate::W<T2QNR_SPEC>;
#[doc = "Field `T2_QNR_VAL` writer - write this reg to configure 64 quantization coefficient in t2 table"]
pub type T2_QNR_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T2QNR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - write this reg to configure 64 quantization coefficient in t2 table"]
    #[inline(always)]
    pub fn t2_qnr_val(&mut self) -> T2_QNR_VAL_W<'_, T2QNR_SPEC> {
        T2_QNR_VAL_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t2qnr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T2QNR_SPEC;
impl crate::RegisterSpec for T2QNR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`t2qnr::W`](W) writer structure"]
impl crate::Writable for T2QNR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T2QNR to value 0"]
impl crate::Resettable for T2QNR_SPEC {}
