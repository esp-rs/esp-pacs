///Register `SDIO_CRC_ST0` reader
pub type R = crate::R<SDIO_CRC_ST0_SPEC>;
///Field `DAT0_CRC_ERR_CNT` reader -
pub type DAT0_CRC_ERR_CNT_R = crate::FieldReader;
///Field `DAT1_CRC_ERR_CNT` reader -
pub type DAT1_CRC_ERR_CNT_R = crate::FieldReader;
///Field `DAT2_CRC_ERR_CNT` reader -
pub type DAT2_CRC_ERR_CNT_R = crate::FieldReader;
///Field `DAT3_CRC_ERR_CNT` reader -
pub type DAT3_CRC_ERR_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn dat0_crc_err_cnt(&self) -> DAT0_CRC_ERR_CNT_R {
        DAT0_CRC_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn dat1_crc_err_cnt(&self) -> DAT1_CRC_ERR_CNT_R {
        DAT1_CRC_ERR_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn dat2_crc_err_cnt(&self) -> DAT2_CRC_ERR_CNT_R {
        DAT2_CRC_ERR_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn dat3_crc_err_cnt(&self) -> DAT3_CRC_ERR_CNT_R {
        DAT3_CRC_ERR_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_CRC_ST0")
            .field("dat0_crc_err_cnt", &self.dat0_crc_err_cnt())
            .field("dat1_crc_err_cnt", &self.dat1_crc_err_cnt())
            .field("dat2_crc_err_cnt", &self.dat2_crc_err_cnt())
            .field("dat3_crc_err_cnt", &self.dat3_crc_err_cnt())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_crc_st0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIO_CRC_ST0_SPEC;
impl crate::RegisterSpec for SDIO_CRC_ST0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdio_crc_st0::R`](R) reader structure
impl crate::Readable for SDIO_CRC_ST0_SPEC {}
///`reset()` method sets SDIO_CRC_ST0 to value 0
impl crate::Resettable for SDIO_CRC_ST0_SPEC {
    const RESET_VALUE: u32 = 0;
}
