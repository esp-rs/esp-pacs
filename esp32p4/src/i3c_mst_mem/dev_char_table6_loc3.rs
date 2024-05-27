///Register `DEV_CHAR_TABLE6_LOC3` reader
pub type R = crate::R<DEV_CHAR_TABLE6_LOC3_SPEC>;
///Field `DCT_DEV6_LOC3` reader - NA
pub type DCT_DEV6_LOC3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - NA
    #[inline(always)]
    pub fn dct_dev6_loc3(&self) -> DCT_DEV6_LOC3_R {
        DCT_DEV6_LOC3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEV_CHAR_TABLE6_LOC3")
            .field("dct_dev6_loc3", &self.dct_dev6_loc3())
            .finish()
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table6_loc3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEV_CHAR_TABLE6_LOC3_SPEC;
impl crate::RegisterSpec for DEV_CHAR_TABLE6_LOC3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dev_char_table6_loc3::R`](R) reader structure
impl crate::Readable for DEV_CHAR_TABLE6_LOC3_SPEC {}
///`reset()` method sets DEV_CHAR_TABLE6_LOC3 to value 0
impl crate::Resettable for DEV_CHAR_TABLE6_LOC3_SPEC {
    const RESET_VALUE: u32 = 0;
}
