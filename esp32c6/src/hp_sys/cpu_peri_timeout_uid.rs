#[doc = "Register `CPU_PERI_TIMEOUT_UID` reader"]
pub struct R(crate::R<CPU_PERI_TIMEOUT_UID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERI_TIMEOUT_UID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERI_TIMEOUT_UID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERI_TIMEOUT_UID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_PERI_TIMEOUT_UID` reader - Record master id\\[4:0\\] &amp; master permission\\[6:5\\] when trigger timeout. This register will be cleared after the interrupt is cleared."]
pub type CPU_PERI_TIMEOUT_UID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Record master id\\[4:0\\] &amp; master permission\\[6:5\\] when trigger timeout. This register will be cleared after the interrupt is cleared."]
    #[inline(always)]
    pub fn cpu_peri_timeout_uid(&self) -> CPU_PERI_TIMEOUT_UID_R {
        CPU_PERI_TIMEOUT_UID_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERI_TIMEOUT_UID")
            .field(
                "cpu_peri_timeout_uid",
                &format_args!("{}", self.cpu_peri_timeout_uid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_PERI_TIMEOUT_UID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CPU_PERI_TIMEOUT_UID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_peri_timeout_uid](index.html) module"]
pub struct CPU_PERI_TIMEOUT_UID_SPEC;
impl crate::RegisterSpec for CPU_PERI_TIMEOUT_UID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_peri_timeout_uid::R](R) reader structure"]
impl crate::Readable for CPU_PERI_TIMEOUT_UID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPU_PERI_TIMEOUT_UID to value 0"]
impl crate::Resettable for CPU_PERI_TIMEOUT_UID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
