#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `TARGET0` reader - Interrupt raw bit of system timer target 0."]
pub type TARGET0_R = crate::BitReader;
#[doc = "Field `TARGET1` reader - Interrupt raw bit of system timer target 1."]
pub type TARGET1_R = crate::BitReader;
#[doc = "Field `TARGET2` reader - Interrupt raw bit of system timer target 2."]
pub type TARGET2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt raw bit of system timer target 0."]
    #[inline(always)]
    pub fn target0(&self) -> TARGET0_R {
        TARGET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt raw bit of system timer target 1."]
    #[inline(always)]
    pub fn target1(&self) -> TARGET1_R {
        TARGET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt raw bit of system timer target 2."]
    #[inline(always)]
    pub fn target2(&self) -> TARGET2_R {
        TARGET2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("target0", &format_args!("{}", self.target0().bit()))
            .field("target1", &format_args!("{}", self.target1().bit()))
            .field("target2", &format_args!("{}", self.target2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "System timer interrupt raw\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
