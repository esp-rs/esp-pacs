#[doc = "Register `AWB0_ACC_R` reader"]
pub type R = crate::R<AWB0_ACC_R_SPEC>;
#[doc = "Field `AWB0_ACC_R` reader - this field represents accumulate of channel r of all white point of algo0"]
pub type AWB0_ACC_R_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this field represents accumulate of channel r of all white point of algo0"]
    #[inline(always)]
    pub fn awb0_acc_r(&self) -> AWB0_ACC_R_R {
        AWB0_ACC_R_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB0_ACC_R")
            .field("awb0_acc_r", &format_args!("{}", self.awb0_acc_r().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AWB0_ACC_R_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "result of accumulate of r channel of all white points\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb0_acc_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB0_ACC_R_SPEC;
impl crate::RegisterSpec for AWB0_ACC_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb0_acc_r::R`](R) reader structure"]
impl crate::Readable for AWB0_ACC_R_SPEC {}
#[doc = "`reset()` method sets AWB0_ACC_R to value 0"]
impl crate::Resettable for AWB0_ACC_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
