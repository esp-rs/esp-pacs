#[doc = "Register `FIFO_STATUS` reader"]
pub struct R(crate::R<FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_EMPTY` reader - 1 indicate that fifo is empty"]
pub type FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `WORK_STATUS` reader - mem_full interrupt status"]
pub type WORK_STATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1 indicate that fifo is empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - mem_full interrupt status"]
    #[inline(always)]
    pub fn work_status(&self) -> WORK_STATUS_R {
        WORK_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_STATUS")
            .field("fifo_empty", &format_args!("{}", self.fifo_empty().bit()))
            .field("work_status", &format_args!("{}", self.work_status().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "fifo status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_status](index.html) module"]
pub struct FIFO_STATUS_SPEC;
impl crate::RegisterSpec for FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_status::R](R) reader structure"]
impl crate::Readable for FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_STATUS to value 0x01"]
impl crate::Resettable for FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
