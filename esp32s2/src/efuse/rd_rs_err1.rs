#[doc = "Register `RD_RS_ERR1` reader"]
pub struct R(crate::R<RD_RS_ERR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_RS_ERR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_RS_ERR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_RS_ERR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY5_ERR_NUM` reader - The value of this signal means the number of error bytes in KEY5."]
pub type KEY5_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `KEY5_FAIL` reader - 0: Means no failure and that the data of KEY5 is reliable. 1: Means that programming user data failed and the number of error bytes is over 5."]
pub type KEY5_FAIL_R = crate::BitReader;
#[doc = "Field `SYS_PART2_ERR_NUM` reader - The value of this signal means the number of error bytes in BLOCK10."]
pub type SYS_PART2_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `SYS_PART2_FAIL` reader - 0: Means no failure and that the data of BLOCK10 is reliable. 1: Means that programming BLOCK10 data failed and the number of error bytes is over 5."]
pub type SYS_PART2_FAIL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - The value of this signal means the number of error bytes in KEY5."]
    #[inline(always)]
    pub fn key5_err_num(&self) -> KEY5_ERR_NUM_R {
        KEY5_ERR_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 0: Means no failure and that the data of KEY5 is reliable. 1: Means that programming user data failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn key5_fail(&self) -> KEY5_FAIL_R {
        KEY5_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - The value of this signal means the number of error bytes in BLOCK10."]
    #[inline(always)]
    pub fn sys_part2_err_num(&self) -> SYS_PART2_ERR_NUM_R {
        SYS_PART2_ERR_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 0: Means no failure and that the data of BLOCK10 is reliable. 1: Means that programming BLOCK10 data failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn sys_part2_fail(&self) -> SYS_PART2_FAIL_R {
        SYS_PART2_FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_RS_ERR1")
            .field(
                "key5_err_num",
                &format_args!("{}", self.key5_err_num().bits()),
            )
            .field("key5_fail", &format_args!("{}", self.key5_fail().bit()))
            .field(
                "sys_part2_err_num",
                &format_args!("{}", self.sys_part2_err_num().bits()),
            )
            .field(
                "sys_part2_fail",
                &format_args!("{}", self.sys_part2_fail().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_RS_ERR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 1 of BLOCK1-10.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_rs_err1](index.html) module"]
pub struct RD_RS_ERR1_SPEC;
impl crate::RegisterSpec for RD_RS_ERR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_rs_err1::R](R) reader structure"]
impl crate::Readable for RD_RS_ERR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_RS_ERR1 to value 0"]
impl crate::Resettable for RD_RS_ERR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
