///Register `BLC_MEAN` reader
pub type R = crate::R<BLC_MEAN_SPEC>;
///Field `BLC_R3_MEAN` reader - this field represents the average black value of bottom right channel
pub type BLC_R3_MEAN_R = crate::FieldReader;
///Field `BLC_R2_MEAN` reader - this field represents the average black value of bottom left channel
pub type BLC_R2_MEAN_R = crate::FieldReader;
///Field `BLC_R1_MEAN` reader - this field represents the average black value of top right channel
pub type BLC_R1_MEAN_R = crate::FieldReader;
///Field `BLC_R0_MEAN` reader - this field represents the average black value of top left channel
pub type BLC_R0_MEAN_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - this field represents the average black value of bottom right channel
    #[inline(always)]
    pub fn blc_r3_mean(&self) -> BLC_R3_MEAN_R {
        BLC_R3_MEAN_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - this field represents the average black value of bottom left channel
    #[inline(always)]
    pub fn blc_r2_mean(&self) -> BLC_R2_MEAN_R {
        BLC_R2_MEAN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - this field represents the average black value of top right channel
    #[inline(always)]
    pub fn blc_r1_mean(&self) -> BLC_R1_MEAN_R {
        BLC_R1_MEAN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - this field represents the average black value of top left channel
    #[inline(always)]
    pub fn blc_r0_mean(&self) -> BLC_R0_MEAN_R {
        BLC_R0_MEAN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLC_MEAN")
            .field("blc_r3_mean", &self.blc_r3_mean())
            .field("blc_r2_mean", &self.blc_r2_mean())
            .field("blc_r1_mean", &self.blc_r1_mean())
            .field("blc_r0_mean", &self.blc_r0_mean())
            .finish()
    }
}
/**results of the average of black window

You can [`read`](crate::generic::Reg::read) this register and get [`blc_mean::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLC_MEAN_SPEC;
impl crate::RegisterSpec for BLC_MEAN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blc_mean::R`](R) reader structure
impl crate::Readable for BLC_MEAN_SPEC {}
///`reset()` method sets BLC_MEAN to value 0
impl crate::Resettable for BLC_MEAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
