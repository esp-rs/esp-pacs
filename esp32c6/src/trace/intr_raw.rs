#[doc = "Register `INTR_RAW` reader"]
pub struct R(crate::R<INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_OVERFLOW_INTR_RAW` reader - fifo_overflow interrupt status"]
pub type FIFO_OVERFLOW_INTR_RAW_R = crate::BitReader;
#[doc = "Field `MEM_FULL_INTR_RAW` reader - mem_full interrupt status"]
pub type MEM_FULL_INTR_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - fifo_overflow interrupt status"]
    #[inline(always)]
    pub fn fifo_overflow_intr_raw(&self) -> FIFO_OVERFLOW_INTR_RAW_R {
        FIFO_OVERFLOW_INTR_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - mem_full interrupt status"]
    #[inline(always)]
    pub fn mem_full_intr_raw(&self) -> MEM_FULL_INTR_RAW_R {
        MEM_FULL_INTR_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_RAW")
            .field(
                "fifo_overflow_intr_raw",
                &format_args!("{}", self.fifo_overflow_intr_raw().bit()),
            )
            .field(
                "mem_full_intr_raw",
                &format_args!("{}", self.mem_full_intr_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_raw](index.html) module"]
pub struct INTR_RAW_SPEC;
impl crate::RegisterSpec for INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_raw::R](R) reader structure"]
impl crate::Readable for INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_RAW to value 0"]
impl crate::Resettable for INTR_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
