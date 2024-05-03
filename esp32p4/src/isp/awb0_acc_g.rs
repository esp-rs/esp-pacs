#[doc = "Register `AWB0_ACC_G` reader"]
pub type R = crate::R<AWB0_ACC_G_SPEC>;
#[doc = "Field `AWB0_ACC_G` reader - this field represents accumulate of channel g of all white point of algo0"]
pub type AWB0_ACC_G_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this field represents accumulate of channel g of all white point of algo0"]
    #[inline(always)]
    pub fn awb0_acc_g(&self) -> AWB0_ACC_G_R {
        AWB0_ACC_G_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB0_ACC_G")
            .field("awb0_acc_g", &self.awb0_acc_g().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AWB0_ACC_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "result of accumulate of g channel of all white points\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb0_acc_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB0_ACC_G_SPEC;
impl crate::RegisterSpec for AWB0_ACC_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb0_acc_g::R`](R) reader structure"]
impl crate::Readable for AWB0_ACC_G_SPEC {}
#[doc = "`reset()` method sets AWB0_ACC_G to value 0"]
impl crate::Resettable for AWB0_ACC_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
