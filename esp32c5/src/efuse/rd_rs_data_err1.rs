#[doc = "Register `RD_RS_DATA_ERR1` reader"]
pub type R = crate::R<RD_RS_DATA_ERR1_SPEC>;
#[doc = "Field `RD_KEY5_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key5_data"]
pub type RD_KEY5_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_KEY5_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_key5_data is reliable\\\\ 1: Means that programming rd_key5_data failed and the number of error bytes is over 6."]
pub type RD_KEY5_DATA_FAIL_R = crate::BitReader;
#[doc = "Field `RD_SYS_PART2_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_sys_part2_data"]
pub type RD_SYS_PART2_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_SYS_PART2_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_sys_part2_data is reliable\\\\ 1: Means that programming rd_sys_part2_data failed and the number of error bytes is over 6."]
pub type RD_SYS_PART2_DATA_FAIL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key5_data"]
    #[inline(always)]
    pub fn rd_key5_data_err_num(&self) -> RD_KEY5_DATA_ERR_NUM_R {
        RD_KEY5_DATA_ERR_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Represents error status of register.\\\\0: Means no failure and that the data of rd_key5_data is reliable\\\\ 1: Means that programming rd_key5_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_key5_data_fail(&self) -> RD_KEY5_DATA_FAIL_R {
        RD_KEY5_DATA_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_sys_part2_data"]
    #[inline(always)]
    pub fn rd_sys_part2_data_err_num(&self) -> RD_SYS_PART2_DATA_ERR_NUM_R {
        RD_SYS_PART2_DATA_ERR_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Represents error status of register.\\\\0: Means no failure and that the data of rd_sys_part2_data is reliable\\\\ 1: Means that programming rd_sys_part2_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_sys_part2_data_fail(&self) -> RD_SYS_PART2_DATA_FAIL_R {
        RD_SYS_PART2_DATA_FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_RS_DATA_ERR1")
            .field("rd_key5_data_err_num", &self.rd_key5_data_err_num())
            .field("rd_key5_data_fail", &self.rd_key5_data_fail())
            .field(
                "rd_sys_part2_data_err_num",
                &self.rd_sys_part2_data_err_num(),
            )
            .field("rd_sys_part2_data_fail", &self.rd_sys_part2_data_fail())
            .finish()
    }
}
#[doc = "Represents rd_rs_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_rs_data_err1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_RS_DATA_ERR1_SPEC;
impl crate::RegisterSpec for RD_RS_DATA_ERR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_rs_data_err1::R`](R) reader structure"]
impl crate::Readable for RD_RS_DATA_ERR1_SPEC {}
#[doc = "`reset()` method sets RD_RS_DATA_ERR1 to value 0"]
impl crate::Resettable for RD_RS_DATA_ERR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
