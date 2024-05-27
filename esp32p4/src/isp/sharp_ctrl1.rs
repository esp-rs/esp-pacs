///Register `SHARP_CTRL1` reader
pub type R = crate::R<SHARP_CTRL1_SPEC>;
///Field `SHARP_GRADIENT_MAX` reader - this field configures sharp max gradient, refresh at the end of each frame end
pub type SHARP_GRADIENT_MAX_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - this field configures sharp max gradient, refresh at the end of each frame end
    #[inline(always)]
    pub fn sharp_gradient_max(&self) -> SHARP_GRADIENT_MAX_R {
        SHARP_GRADIENT_MAX_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHARP_CTRL1")
            .field("sharp_gradient_max", &self.sharp_gradient_max())
            .finish()
    }
}
/**sharp control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_ctrl1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SHARP_CTRL1_SPEC;
impl crate::RegisterSpec for SHARP_CTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sharp_ctrl1::R`](R) reader structure
impl crate::Readable for SHARP_CTRL1_SPEC {}
///`reset()` method sets SHARP_CTRL1 to value 0
impl crate::Resettable for SHARP_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
