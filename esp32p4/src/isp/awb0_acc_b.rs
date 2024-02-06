#[doc = "Register `AWB0_ACC_B` reader"]
pub type R = crate::R<AWB0_ACC_B_SPEC>;
#[doc = "Field `AWB0_ACC_B` reader - this field represents accumulate of channel b of all white point of algo0"]
pub type AWB0_ACC_B_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this field represents accumulate of channel b of all white point of algo0"]
    #[inline(always)]
    pub fn awb0_acc_b(&self) -> AWB0_ACC_B_R {
        AWB0_ACC_B_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB0_ACC_B")
            .field("awb0_acc_b", &format_args!("{}", self.awb0_acc_b().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AWB0_ACC_B_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "result of accumulate of b channel of all white points\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb0_acc_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB0_ACC_B_SPEC;
impl crate::RegisterSpec for AWB0_ACC_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb0_acc_b::R`](R) reader structure"]
impl crate::Readable for AWB0_ACC_B_SPEC {}
#[doc = "`reset()` method sets AWB0_ACC_B to value 0"]
impl crate::Resettable for AWB0_ACC_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
