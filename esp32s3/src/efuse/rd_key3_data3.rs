#[doc = "Register `RD_KEY3_DATA3` reader"]
pub type R = crate::R<RD_KEY3_DATA3_SPEC>;
#[doc = "Field `KEY3_DATA3` reader - Stores the third 32 bits of KEY3."]
pub type KEY3_DATA3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the third 32 bits of KEY3."]
    #[inline(always)]
    pub fn key3_data3(&self) -> KEY3_DATA3_R {
        KEY3_DATA3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY3_DATA3")
            .field("key3_data3", &format_args!("{}", self.key3_data3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY3_DATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register 3 of BLOCK7 (KEY3).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY3_DATA3_SPEC;
impl crate::RegisterSpec for RD_KEY3_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key3_data3::R`](R) reader structure"]
impl crate::Readable for RD_KEY3_DATA3_SPEC {}
#[doc = "`reset()` method sets RD_KEY3_DATA3 to value 0"]
impl crate::Resettable for RD_KEY3_DATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
