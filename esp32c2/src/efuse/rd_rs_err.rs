#[doc = "Register `RD_RS_ERR` reader"]
pub struct R(crate::R<RD_RS_ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_RS_ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_RS_ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_RS_ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK1_ERR_NUM` reader - The value of this signal means the number of error bytes in block1."]
pub type BLK1_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `BLK1_FAIL` reader - 0: Means no failure and that the data of block1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
pub type BLK1_FAIL_R = crate::BitReader;
#[doc = "Field `BLK2_ERR_NUM` reader - The value of this signal means the number of error bytes in block2."]
pub type BLK2_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `BLK2_FAIL` reader - 0: Means no failure and that the data of block2 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
pub type BLK2_FAIL_R = crate::BitReader;
#[doc = "Field `BLK3_ERR_NUM` reader - The value of this signal means the number of error bytes in block3."]
pub type BLK3_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `BLK3_FAIL` reader - 0: Means no failure and that the block3 data is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
pub type BLK3_FAIL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - The value of this signal means the number of error bytes in block1."]
    #[inline(always)]
    pub fn blk1_err_num(&self) -> BLK1_ERR_NUM_R {
        BLK1_ERR_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 0: Means no failure and that the data of block1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn blk1_fail(&self) -> BLK1_FAIL_R {
        BLK1_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - The value of this signal means the number of error bytes in block2."]
    #[inline(always)]
    pub fn blk2_err_num(&self) -> BLK2_ERR_NUM_R {
        BLK2_ERR_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 0: Means no failure and that the data of block2 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn blk2_fail(&self) -> BLK2_FAIL_R {
        BLK2_FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - The value of this signal means the number of error bytes in block3."]
    #[inline(always)]
    pub fn blk3_err_num(&self) -> BLK3_ERR_NUM_R {
        BLK3_ERR_NUM_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 0: Means no failure and that the block3 data is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn blk3_fail(&self) -> BLK3_FAIL_R {
        BLK3_FAIL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_RS_ERR")
            .field(
                "blk1_err_num",
                &format_args!("{}", self.blk1_err_num().bits()),
            )
            .field("blk1_fail", &format_args!("{}", self.blk1_fail().bit()))
            .field(
                "blk2_err_num",
                &format_args!("{}", self.blk2_err_num().bits()),
            )
            .field("blk2_fail", &format_args!("{}", self.blk2_fail().bit()))
            .field(
                "blk3_err_num",
                &format_args!("{}", self.blk3_err_num().bits()),
            )
            .field("blk3_fail", &format_args!("{}", self.blk3_fail().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_RS_ERR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 0 of BLOCK1-10.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_rs_err](index.html) module"]
pub struct RD_RS_ERR_SPEC;
impl crate::RegisterSpec for RD_RS_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_rs_err::R](R) reader structure"]
impl crate::Readable for RD_RS_ERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_RS_ERR to value 0"]
impl crate::Resettable for RD_RS_ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
