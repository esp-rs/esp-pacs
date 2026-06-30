#[doc = "Register `YOLO` reader"]
pub type R = crate::R<YOLO_SPEC>;
#[doc = "Field `VAL` reader - What else could be in this register??"]
pub type VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - What else could be in this register??"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YOLO").field("val", &self.val()).finish()
    }
}
#[doc = "TWAI FD transmitted frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`yolo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YOLO_SPEC;
impl crate::RegisterSpec for YOLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`yolo::R`](R) reader structure"]
impl crate::Readable for YOLO_SPEC {}
#[doc = "`reset()` method sets YOLO to value 0xdead_beef"]
impl crate::Resettable for YOLO_SPEC {
    const RESET_VALUE: u32 = 0xdead_beef;
}
