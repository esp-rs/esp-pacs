#[doc = "Register `RD_RS_ERR0` reader"]
pub struct R(crate::R<RD_RS_ERR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_RS_ERR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_RS_ERR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_RS_ERR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAC_SPI_8M_ERR_NUM` reader - The value of this signal means the number of error bytes in BLOCK1."]
pub type MAC_SPI_8M_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `MAC_SPI_8M_FAIL` reader - 0: Means no failure and that the data of BLOCK1 is reliable. 1: Means that programming BLOCK1 data failed and the number of error bytes is over 5."]
pub type MAC_SPI_8M_FAIL_R = crate::BitReader;
#[doc = "Field `SYS_PART1_NUM` reader - The value of this signal means the number of error bytes in BLOCK2."]
pub type SYS_PART1_NUM_R = crate::FieldReader;
#[doc = "Field `SYS_PART1_FAIL` reader - 0: Means no failure and that the data of BLOCK2 is reliable. 1: Means that programming BLOCK2 data failed and the number of error bytes is over 5."]
pub type SYS_PART1_FAIL_R = crate::BitReader;
#[doc = "Field `USR_DATA_ERR_NUM` reader - The value of this signal means the number of error bytes in BLOCK3."]
pub type USR_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `USR_DATA_FAIL` reader - 0: Means no failure and that the data of BLOCK3 is reliable. 1: Means that programming BLOCK3 data failed and the number of error bytes is over 5."]
pub type USR_DATA_FAIL_R = crate::BitReader;
#[doc = "Field `KEY0_ERR_NUM` reader - The value of this signal means the number of error bytes in KEY0."]
pub type KEY0_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `KEY0_FAIL` reader - 0: Means no failure and that the data of KEY0 is reliable. 1: Means that programming KEY0 failed and the number of error bytes is over 5."]
pub type KEY0_FAIL_R = crate::BitReader;
#[doc = "Field `KEY1_ERR_NUM` reader - The value of this signal means the number of error bytes in KEY1."]
pub type KEY1_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `KEY1_FAIL` reader - 0: Means no failure and that the data of KEY1 is reliable. 1: Means that programming KEY1 failed and the number of error bytes is over 5."]
pub type KEY1_FAIL_R = crate::BitReader;
#[doc = "Field `KEY2_ERR_NUM` reader - The value of this signal means the number of error bytes in KEY2."]
pub type KEY2_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `KEY2_FAIL` reader - 0: Means no failure and that the data of KEY2 is reliable. 1: Means that programming KEY2 failed and the number of error bytes is over 5."]
pub type KEY2_FAIL_R = crate::BitReader;
#[doc = "Field `KEY3_ERR_NUM` reader - The value of this signal means the number of error bytes in KEY3."]
pub type KEY3_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `KEY3_FAIL` reader - 0: Means no failure and that the data of KEY3 is reliable. 1: Means that programming KEY3 failed and the number of error bytes is over 5."]
pub type KEY3_FAIL_R = crate::BitReader;
#[doc = "Field `KEY4_ERR_NUM` reader - The value of this signal means the number of error bytes in KEY4."]
pub type KEY4_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `KEY4_FAIL` reader - 0: Means no failure and that the data of KEY4 is reliable. 1: Means that programming KEY4 failed and the number of error bytes is over 5."]
pub type KEY4_FAIL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - The value of this signal means the number of error bytes in BLOCK1."]
    #[inline(always)]
    pub fn mac_spi_8m_err_num(&self) -> MAC_SPI_8M_ERR_NUM_R {
        MAC_SPI_8M_ERR_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 0: Means no failure and that the data of BLOCK1 is reliable. 1: Means that programming BLOCK1 data failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn mac_spi_8m_fail(&self) -> MAC_SPI_8M_FAIL_R {
        MAC_SPI_8M_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - The value of this signal means the number of error bytes in BLOCK2."]
    #[inline(always)]
    pub fn sys_part1_num(&self) -> SYS_PART1_NUM_R {
        SYS_PART1_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 0: Means no failure and that the data of BLOCK2 is reliable. 1: Means that programming BLOCK2 data failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn sys_part1_fail(&self) -> SYS_PART1_FAIL_R {
        SYS_PART1_FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - The value of this signal means the number of error bytes in BLOCK3."]
    #[inline(always)]
    pub fn usr_data_err_num(&self) -> USR_DATA_ERR_NUM_R {
        USR_DATA_ERR_NUM_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 0: Means no failure and that the data of BLOCK3 is reliable. 1: Means that programming BLOCK3 data failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn usr_data_fail(&self) -> USR_DATA_FAIL_R {
        USR_DATA_FAIL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - The value of this signal means the number of error bytes in KEY0."]
    #[inline(always)]
    pub fn key0_err_num(&self) -> KEY0_ERR_NUM_R {
        KEY0_ERR_NUM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 0: Means no failure and that the data of KEY0 is reliable. 1: Means that programming KEY0 failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn key0_fail(&self) -> KEY0_FAIL_R {
        KEY0_FAIL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - The value of this signal means the number of error bytes in KEY1."]
    #[inline(always)]
    pub fn key1_err_num(&self) -> KEY1_ERR_NUM_R {
        KEY1_ERR_NUM_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 0: Means no failure and that the data of KEY1 is reliable. 1: Means that programming KEY1 failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn key1_fail(&self) -> KEY1_FAIL_R {
        KEY1_FAIL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - The value of this signal means the number of error bytes in KEY2."]
    #[inline(always)]
    pub fn key2_err_num(&self) -> KEY2_ERR_NUM_R {
        KEY2_ERR_NUM_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 0: Means no failure and that the data of KEY2 is reliable. 1: Means that programming KEY2 failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn key2_fail(&self) -> KEY2_FAIL_R {
        KEY2_FAIL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - The value of this signal means the number of error bytes in KEY3."]
    #[inline(always)]
    pub fn key3_err_num(&self) -> KEY3_ERR_NUM_R {
        KEY3_ERR_NUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 0: Means no failure and that the data of KEY3 is reliable. 1: Means that programming KEY3 failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn key3_fail(&self) -> KEY3_FAIL_R {
        KEY3_FAIL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - The value of this signal means the number of error bytes in KEY4."]
    #[inline(always)]
    pub fn key4_err_num(&self) -> KEY4_ERR_NUM_R {
        KEY4_ERR_NUM_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 0: Means no failure and that the data of KEY4 is reliable. 1: Means that programming KEY4 failed and the number of error bytes is over 5."]
    #[inline(always)]
    pub fn key4_fail(&self) -> KEY4_FAIL_R {
        KEY4_FAIL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_RS_ERR0")
            .field(
                "mac_spi_8m_err_num",
                &format_args!("{}", self.mac_spi_8m_err_num().bits()),
            )
            .field(
                "mac_spi_8m_fail",
                &format_args!("{}", self.mac_spi_8m_fail().bit()),
            )
            .field(
                "sys_part1_num",
                &format_args!("{}", self.sys_part1_num().bits()),
            )
            .field(
                "sys_part1_fail",
                &format_args!("{}", self.sys_part1_fail().bit()),
            )
            .field(
                "usr_data_err_num",
                &format_args!("{}", self.usr_data_err_num().bits()),
            )
            .field(
                "usr_data_fail",
                &format_args!("{}", self.usr_data_fail().bit()),
            )
            .field(
                "key0_err_num",
                &format_args!("{}", self.key0_err_num().bits()),
            )
            .field("key0_fail", &format_args!("{}", self.key0_fail().bit()))
            .field(
                "key1_err_num",
                &format_args!("{}", self.key1_err_num().bits()),
            )
            .field("key1_fail", &format_args!("{}", self.key1_fail().bit()))
            .field(
                "key2_err_num",
                &format_args!("{}", self.key2_err_num().bits()),
            )
            .field("key2_fail", &format_args!("{}", self.key2_fail().bit()))
            .field(
                "key3_err_num",
                &format_args!("{}", self.key3_err_num().bits()),
            )
            .field("key3_fail", &format_args!("{}", self.key3_fail().bit()))
            .field(
                "key4_err_num",
                &format_args!("{}", self.key4_err_num().bits()),
            )
            .field("key4_fail", &format_args!("{}", self.key4_fail().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_RS_ERR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 0 of BLOCK1-10.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_rs_err0](index.html) module"]
pub struct RD_RS_ERR0_SPEC;
impl crate::RegisterSpec for RD_RS_ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_rs_err0::R](R) reader structure"]
impl crate::Readable for RD_RS_ERR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_RS_ERR0 to value 0"]
impl crate::Resettable for RD_RS_ERR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
