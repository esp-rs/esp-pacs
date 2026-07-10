#[doc = "Register `CROP_ERR_ST` reader"]
pub type R = crate::R<CROP_ERR_ST_SPEC>;
#[doc = "Field `CROP_Y_MISMATCH` reader - Represents isp_corp row end index over image size"]
pub type CROP_Y_MISMATCH_R = crate::BitReader;
#[doc = "Field `CROP_X_MISMATCH` reader - Represents isp_corp col end index over image size"]
pub type CROP_X_MISMATCH_R = crate::BitReader;
#[doc = "Field `CROP_Y_END_EVEN` reader - Represents isp_corp row end index is an even number"]
pub type CROP_Y_END_EVEN_R = crate::BitReader;
#[doc = "Field `CROP_X_END_EVEN` reader - Represents isp_corp col end index is an even number"]
pub type CROP_X_END_EVEN_R = crate::BitReader;
#[doc = "Field `CROP_Y_START_ODD` reader - Represents isp_corp row start index is an odd number"]
pub type CROP_Y_START_ODD_R = crate::BitReader;
#[doc = "Field `CROP_X_START_ODD` reader - Represents isp_corp col start index is an odd number"]
pub type CROP_X_START_ODD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents isp_corp row end index over image size"]
    #[inline(always)]
    pub fn crop_y_mismatch(&self) -> CROP_Y_MISMATCH_R {
        CROP_Y_MISMATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents isp_corp col end index over image size"]
    #[inline(always)]
    pub fn crop_x_mismatch(&self) -> CROP_X_MISMATCH_R {
        CROP_X_MISMATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents isp_corp row end index is an even number"]
    #[inline(always)]
    pub fn crop_y_end_even(&self) -> CROP_Y_END_EVEN_R {
        CROP_Y_END_EVEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents isp_corp col end index is an even number"]
    #[inline(always)]
    pub fn crop_x_end_even(&self) -> CROP_X_END_EVEN_R {
        CROP_X_END_EVEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents isp_corp row start index is an odd number"]
    #[inline(always)]
    pub fn crop_y_start_odd(&self) -> CROP_Y_START_ODD_R {
        CROP_Y_START_ODD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents isp_corp col start index is an odd number"]
    #[inline(always)]
    pub fn crop_x_start_odd(&self) -> CROP_X_START_ODD_R {
        CROP_X_START_ODD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CROP_ERR_ST")
            .field("crop_y_mismatch", &self.crop_y_mismatch())
            .field("crop_x_mismatch", &self.crop_x_mismatch())
            .field("crop_y_end_even", &self.crop_y_end_even())
            .field("crop_x_end_even", &self.crop_x_end_even())
            .field("crop_y_start_odd", &self.crop_y_start_odd())
            .field("crop_x_start_odd", &self.crop_x_start_odd())
            .finish()
    }
}
#[doc = "crop error state register\n\nYou can [`read`](crate::Reg::read) this register and get [`crop_err_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CROP_ERR_ST_SPEC;
impl crate::RegisterSpec for CROP_ERR_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crop_err_st::R`](R) reader structure"]
impl crate::Readable for CROP_ERR_ST_SPEC {}
#[doc = "`reset()` method sets CROP_ERR_ST to value 0"]
impl crate::Resettable for CROP_ERR_ST_SPEC {}
