#[doc = "Register `DEV_CHAR_TABLE6_LOC2` reader"]
pub type R = crate::R<DEV_CHAR_TABLE6_LOC2_SPEC>;
#[doc = "Field `DCT_DEV6_LOC2` reader - NA"]
pub type DCT_DEV6_LOC2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dct_dev6_loc2(&self) -> DCT_DEV6_LOC2_R {
        DCT_DEV6_LOC2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEV_CHAR_TABLE6_LOC2")
            .field(
                "dct_dev6_loc2",
                &format_args!("{}", self.dct_dev6_loc2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEV_CHAR_TABLE6_LOC2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table6_loc2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEV_CHAR_TABLE6_LOC2_SPEC;
impl crate::RegisterSpec for DEV_CHAR_TABLE6_LOC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_char_table6_loc2::R`](R) reader structure"]
impl crate::Readable for DEV_CHAR_TABLE6_LOC2_SPEC {}
#[doc = "`reset()` method sets DEV_CHAR_TABLE6_LOC2 to value 0"]
impl crate::Resettable for DEV_CHAR_TABLE6_LOC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
