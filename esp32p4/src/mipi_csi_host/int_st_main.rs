#[doc = "Register `INT_ST_MAIN` reader"]
pub type R = crate::R<INT_ST_MAIN_SPEC>;
#[doc = "Field `ST_STATUS_INT_PHY_FATAL` reader - NA"]
pub type ST_STATUS_INT_PHY_FATAL_R = crate::BitReader;
#[doc = "Field `ST_STATUS_INT_PKT_FATAL` reader - NA"]
pub type ST_STATUS_INT_PKT_FATAL_R = crate::BitReader;
#[doc = "Field `ST_STATUS_INT_BNDRY_FRAME_FATAL` reader - NA"]
pub type ST_STATUS_INT_BNDRY_FRAME_FATAL_R = crate::BitReader;
#[doc = "Field `ST_STATUS_INT_SEQ_FRAME_FATAL` reader - NA"]
pub type ST_STATUS_INT_SEQ_FRAME_FATAL_R = crate::BitReader;
#[doc = "Field `ST_STATUS_INT_CRC_FRAME_FATAL` reader - NA"]
pub type ST_STATUS_INT_CRC_FRAME_FATAL_R = crate::BitReader;
#[doc = "Field `ST_STATUS_INT_PLD_CRC_FATAL` reader - NA"]
pub type ST_STATUS_INT_PLD_CRC_FATAL_R = crate::BitReader;
#[doc = "Field `ST_STATUS_INT_DATA_ID` reader - NA"]
pub type ST_STATUS_INT_DATA_ID_R = crate::BitReader;
#[doc = "Field `ST_STATUS_INT_ECC_CORRECTED` reader - NA"]
pub type ST_STATUS_INT_ECC_CORRECTED_R = crate::BitReader;
#[doc = "Field `ST_STATUS_INT_PHY` reader - NA"]
pub type ST_STATUS_INT_PHY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_status_int_phy_fatal(&self) -> ST_STATUS_INT_PHY_FATAL_R {
        ST_STATUS_INT_PHY_FATAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_status_int_pkt_fatal(&self) -> ST_STATUS_INT_PKT_FATAL_R {
        ST_STATUS_INT_PKT_FATAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn st_status_int_bndry_frame_fatal(&self) -> ST_STATUS_INT_BNDRY_FRAME_FATAL_R {
        ST_STATUS_INT_BNDRY_FRAME_FATAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn st_status_int_seq_frame_fatal(&self) -> ST_STATUS_INT_SEQ_FRAME_FATAL_R {
        ST_STATUS_INT_SEQ_FRAME_FATAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn st_status_int_crc_frame_fatal(&self) -> ST_STATUS_INT_CRC_FRAME_FATAL_R {
        ST_STATUS_INT_CRC_FRAME_FATAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn st_status_int_pld_crc_fatal(&self) -> ST_STATUS_INT_PLD_CRC_FATAL_R {
        ST_STATUS_INT_PLD_CRC_FATAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn st_status_int_data_id(&self) -> ST_STATUS_INT_DATA_ID_R {
        ST_STATUS_INT_DATA_ID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn st_status_int_ecc_corrected(&self) -> ST_STATUS_INT_ECC_CORRECTED_R {
        ST_STATUS_INT_ECC_CORRECTED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn st_status_int_phy(&self) -> ST_STATUS_INT_PHY_R {
        ST_STATUS_INT_PHY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_MAIN")
            .field("st_status_int_phy_fatal", &self.st_status_int_phy_fatal())
            .field("st_status_int_pkt_fatal", &self.st_status_int_pkt_fatal())
            .field(
                "st_status_int_bndry_frame_fatal",
                &self.st_status_int_bndry_frame_fatal(),
            )
            .field(
                "st_status_int_seq_frame_fatal",
                &self.st_status_int_seq_frame_fatal(),
            )
            .field(
                "st_status_int_crc_frame_fatal",
                &self.st_status_int_crc_frame_fatal(),
            )
            .field(
                "st_status_int_pld_crc_fatal",
                &self.st_status_int_pld_crc_fatal(),
            )
            .field("st_status_int_data_id", &self.st_status_int_data_id())
            .field(
                "st_status_int_ecc_corrected",
                &self.st_status_int_ecc_corrected(),
            )
            .field("st_status_int_phy", &self.st_status_int_phy())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_main::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_MAIN_SPEC;
impl crate::RegisterSpec for INT_ST_MAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_main::R`](R) reader structure"]
impl crate::Readable for INT_ST_MAIN_SPEC {}
#[doc = "`reset()` method sets INT_ST_MAIN to value 0"]
impl crate::Resettable for INT_ST_MAIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
