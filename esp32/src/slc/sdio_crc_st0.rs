#[doc = "Register `SDIO_CRC_ST0` reader"]
pub struct R(crate::R<SDIO_CRC_ST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CRC_ST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_CRC_ST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_CRC_ST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAT0_CRC_ERR_CNT` reader - "]
pub type DAT0_CRC_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `DAT1_CRC_ERR_CNT` reader - "]
pub type DAT1_CRC_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `DAT2_CRC_ERR_CNT` reader - "]
pub type DAT2_CRC_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `DAT3_CRC_ERR_CNT` reader - "]
pub type DAT3_CRC_ERR_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dat0_crc_err_cnt(&self) -> DAT0_CRC_ERR_CNT_R {
        DAT0_CRC_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn dat1_crc_err_cnt(&self) -> DAT1_CRC_ERR_CNT_R {
        DAT1_CRC_ERR_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn dat2_crc_err_cnt(&self) -> DAT2_CRC_ERR_CNT_R {
        DAT2_CRC_ERR_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dat3_crc_err_cnt(&self) -> DAT3_CRC_ERR_CNT_R {
        DAT3_CRC_ERR_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_CRC_ST0")
            .field(
                "dat0_crc_err_cnt",
                &format_args!("{}", self.dat0_crc_err_cnt().bits()),
            )
            .field(
                "dat1_crc_err_cnt",
                &format_args!("{}", self.dat1_crc_err_cnt().bits()),
            )
            .field(
                "dat2_crc_err_cnt",
                &format_args!("{}", self.dat2_crc_err_cnt().bits()),
            )
            .field(
                "dat3_crc_err_cnt",
                &format_args!("{}", self.dat3_crc_err_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_CRC_ST0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_crc_st0](index.html) module"]
pub struct SDIO_CRC_ST0_SPEC;
impl crate::RegisterSpec for SDIO_CRC_ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_crc_st0::R](R) reader structure"]
impl crate::Readable for SDIO_CRC_ST0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDIO_CRC_ST0 to value 0"]
impl crate::Resettable for SDIO_CRC_ST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
