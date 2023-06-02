#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INT0_RAW` reader - Interrupt raw bit of system timer target 0."]
pub type INT0_RAW_R = crate::BitReader;
#[doc = "Field `INT1_RAW` reader - Interrupt raw bit of system timer target 1."]
pub type INT1_RAW_R = crate::BitReader;
#[doc = "Field `INT2_RAW` reader - Interrupt raw bit of system timer target 2."]
pub type INT2_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt raw bit of system timer target 0."]
    #[inline(always)]
    pub fn int0_raw(&self) -> INT0_RAW_R {
        INT0_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt raw bit of system timer target 1."]
    #[inline(always)]
    pub fn int1_raw(&self) -> INT1_RAW_R {
        INT1_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt raw bit of system timer target 2."]
    #[inline(always)]
    pub fn int2_raw(&self) -> INT2_RAW_R {
        INT2_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("int0_raw", &format_args!("{}", self.int0_raw().bit()))
            .field("int1_raw", &format_args!("{}", self.int1_raw().bit()))
            .field("int2_raw", &format_args!("{}", self.int2_raw().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "System timer interrupt raw\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
