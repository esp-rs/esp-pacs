#[doc = "Register `ERR_CAPT_RETR_CTR_ALC_TS_INFO` reader"]
pub type R = crate::R<ERR_CAPT_RETR_CTR_ALC_TS_INFO_SPEC>;
#[doc = "Field `ERR_POS` reader - 0b00000 - ERC_POS_SOF - Error in Start of Frame 0b00001 - ERC_POS_ARB - Error in Arbitration Filed 0b00010 - ERC_POS_CTRL - Error in Control field 0b00011 - ERC_POS_DATA - Error in Data Field 0b00100 - ERC_POS_CRC - Error in CRC Field 0b00101 - ERC_POS_ACK - Error in CRC delimiter, ACK field or ACK delimiter 0b00110 - ERC_POS_EOF - Error in End of frame field 0b00111 - ERC_POS_ERR - Error during Error frame 0b01000 - ERC_POS_OVRL - Error in Overload frame 0b11111 - ERC_POS_OTHER - Other position of error"]
pub type ERR_POS_R = crate::FieldReader;
#[doc = "Field `ERR_TYPE` reader - Type of last error. 0b000 - ERC_BIT_ERR - Bit Error 0b001 - ERC_CRC_ERR - CRC Error 0b010 - ERC_FRM_ERR - Form Error 0b011 - ERC_ACK_ERR - Acknowledge Error 0b100 - ERC_STUF_ERR - Stuff Error"]
pub type ERR_TYPE_R = crate::FieldReader;
#[doc = "Field `RETR_CTR_VAL` reader - Current value of retransmitt counter."]
pub type RETR_CTR_VAL_R = crate::FieldReader;
#[doc = "Field `ALC_BIT` reader - Arbitration lost capture bit position. If ALC_ID_FIELD = ALC_BASE_ID then bit index of BASE identifier in which arbitration was lost is given as: 11 - ALC_VAL. If ALC_ID_FIELD = ALC_EXTENSION then bit index of EXTENDED identifier in which arbitration was lost is given as: 18 - ALC_VAL. For other values of ALC_ID_FIELD, this value is undefined."]
pub type ALC_BIT_R = crate::FieldReader;
#[doc = "Field `ALC_ID_FIELD` reader - Part of CAN Identifier in which arbitration was lost. 0b000 - ALC_RSVD - Unit did not loose arbitration since last reset. 0b001 - ALC_BASE_ID - Arbitration was lost during base identifier. 0b010 - ALC_SRR_RTR - Arbitration was lost during first bit after base identifier (SRR of Extended Frame, RTR bit of CAN 2.0 Base Frame) 0b011 - ALC_IDE - Arbitration was lost during IDE bit. 0b100 - ALC_EXTENSION - Arbitration was lost during Identifier extension. 0b101 - ALC_RTR - Arbitration was lost during RTR bit after Identifier extension!"]
pub type ALC_ID_FIELD_R = crate::FieldReader;
#[doc = "Field `TS_BITS` reader - Number of active bits of CTU CAN FD time-base minus 1 (0x3F = 64 bit time-base)."]
pub type TS_BITS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - 0b00000 - ERC_POS_SOF - Error in Start of Frame 0b00001 - ERC_POS_ARB - Error in Arbitration Filed 0b00010 - ERC_POS_CTRL - Error in Control field 0b00011 - ERC_POS_DATA - Error in Data Field 0b00100 - ERC_POS_CRC - Error in CRC Field 0b00101 - ERC_POS_ACK - Error in CRC delimiter, ACK field or ACK delimiter 0b00110 - ERC_POS_EOF - Error in End of frame field 0b00111 - ERC_POS_ERR - Error during Error frame 0b01000 - ERC_POS_OVRL - Error in Overload frame 0b11111 - ERC_POS_OTHER - Other position of error"]
    #[inline(always)]
    pub fn err_pos(&self) -> ERR_POS_R {
        ERR_POS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Type of last error. 0b000 - ERC_BIT_ERR - Bit Error 0b001 - ERC_CRC_ERR - CRC Error 0b010 - ERC_FRM_ERR - Form Error 0b011 - ERC_ACK_ERR - Acknowledge Error 0b100 - ERC_STUF_ERR - Stuff Error"]
    #[inline(always)]
    pub fn err_type(&self) -> ERR_TYPE_R {
        ERR_TYPE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Current value of retransmitt counter."]
    #[inline(always)]
    pub fn retr_ctr_val(&self) -> RETR_CTR_VAL_R {
        RETR_CTR_VAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Arbitration lost capture bit position. If ALC_ID_FIELD = ALC_BASE_ID then bit index of BASE identifier in which arbitration was lost is given as: 11 - ALC_VAL. If ALC_ID_FIELD = ALC_EXTENSION then bit index of EXTENDED identifier in which arbitration was lost is given as: 18 - ALC_VAL. For other values of ALC_ID_FIELD, this value is undefined."]
    #[inline(always)]
    pub fn alc_bit(&self) -> ALC_BIT_R {
        ALC_BIT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Part of CAN Identifier in which arbitration was lost. 0b000 - ALC_RSVD - Unit did not loose arbitration since last reset. 0b001 - ALC_BASE_ID - Arbitration was lost during base identifier. 0b010 - ALC_SRR_RTR - Arbitration was lost during first bit after base identifier (SRR of Extended Frame, RTR bit of CAN 2.0 Base Frame) 0b011 - ALC_IDE - Arbitration was lost during IDE bit. 0b100 - ALC_EXTENSION - Arbitration was lost during Identifier extension. 0b101 - ALC_RTR - Arbitration was lost during RTR bit after Identifier extension!"]
    #[inline(always)]
    pub fn alc_id_field(&self) -> ALC_ID_FIELD_R {
        ALC_ID_FIELD_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:29 - Number of active bits of CTU CAN FD time-base minus 1 (0x3F = 64 bit time-base)."]
    #[inline(always)]
    pub fn ts_bits(&self) -> TS_BITS_R {
        TS_BITS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_CAPT_RETR_CTR_ALC_TS_INFO")
            .field("err_pos", &self.err_pos())
            .field("err_type", &self.err_type())
            .field("retr_ctr_val", &self.retr_ctr_val())
            .field("alc_bit", &self.alc_bit())
            .field("alc_id_field", &self.alc_id_field())
            .field("ts_bits", &self.ts_bits())
            .finish()
    }
}
#[doc = "TWAI FD error capture & retransmit counter & arbitration lost & timestamp integration information register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_capt_retr_ctr_alc_ts_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_CAPT_RETR_CTR_ALC_TS_INFO_SPEC;
impl crate::RegisterSpec for ERR_CAPT_RETR_CTR_ALC_TS_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_capt_retr_ctr_alc_ts_info::R`](R) reader structure"]
impl crate::Readable for ERR_CAPT_RETR_CTR_ALC_TS_INFO_SPEC {}
#[doc = "`reset()` method sets ERR_CAPT_RETR_CTR_ALC_TS_INFO to value 0x1f"]
impl crate::Resettable for ERR_CAPT_RETR_CTR_ALC_TS_INFO_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
