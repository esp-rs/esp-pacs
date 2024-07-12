#[doc = "Register `AWB0_WHITE_CNT` reader"]
pub type R = crate::R<AWB0_WHITE_CNT_SPEC>;
#[doc = "Field `AWB0_WHITE_CNT` reader - this field configures number of white point detected of algo0"]
pub type AWB0_WHITE_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - this field configures number of white point detected of algo0"]
    #[inline(always)]
    pub fn awb0_white_cnt(&self) -> AWB0_WHITE_CNT_R {
        AWB0_WHITE_CNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB0_WHITE_CNT")
            .field("awb0_white_cnt", &self.awb0_white_cnt())
            .finish()
    }
}
#[doc = "result of awb white point number\n\nYou can [`read`](crate::Reg::read) this register and get [`awb0_white_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB0_WHITE_CNT_SPEC;
impl crate::RegisterSpec for AWB0_WHITE_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb0_white_cnt::R`](R) reader structure"]
impl crate::Readable for AWB0_WHITE_CNT_SPEC {}
#[doc = "`reset()` method sets AWB0_WHITE_CNT to value 0"]
impl crate::Resettable for AWB0_WHITE_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
