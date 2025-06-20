#[doc = "Register `DEBUG` reader"]
pub type R = crate::R<DEBUG_SPEC>;
#[doc = "Field `STUFF_COUNT` reader - Actual stuff count modulo 8 as definned in ISO FD protocol. Stuff count is erased in the beginning of CAN frame and increased by one with each stuff bit until Stuff count field in ISO FD frame. Then it stays fixed until the beginning of next frame. In non-ISO FD or normal CAN stuff bits are counted until the end of a frame. Note that this field is NOT gray encoded as defined in ISO FD standard. Stuff count is calculated only as long as controller is transceiving on the bus. During the reception this value remains fixed!"]
pub type STUFF_COUNT_R = crate::FieldReader;
#[doc = "Field `DESTUFF_COUNT` reader - Actual de-stuff count modulo 8 as defined in ISO FD protocol. De-Stuff count is erased in the beginning of the frame and increased by one with each de-stuffed bit until Stuff count field in ISO FD Frame. Then it stays fixed until beginning of next frame. In non-ISO FD or normal CAN de-stuff bits are counted until the end of the frame. Note that this field is NOT grey encoded as defined in ISO FD standard. De-stuff count is calculated in both. Transceiver as well as receiver."]
pub type DESTUFF_COUNT_R = crate::FieldReader;
#[doc = "Field `PC_ARB` reader - Protocol control state machine is in Arbitration field."]
pub type PC_ARB_R = crate::BitReader;
#[doc = "Field `PC_CON` reader - Protocol control state machine is in Control field."]
pub type PC_CON_R = crate::BitReader;
#[doc = "Field `PC_DAT` reader - Protocol control state machine is in Data field."]
pub type PC_DAT_R = crate::BitReader;
#[doc = "Field `PC_STC` reader - Protocol control state machine is in Stuff Count field."]
pub type PC_STC_R = crate::BitReader;
#[doc = "Field `PC_CRC` reader - Protocol control state machine is in CRC field."]
pub type PC_CRC_R = crate::BitReader;
#[doc = "Field `PC_CRCD` reader - Protocol control state machine is in CRC Delimiter field."]
pub type PC_CRCD_R = crate::BitReader;
#[doc = "Field `PC_ACK` reader - Protocol control state machine is in ACK field."]
pub type PC_ACK_R = crate::BitReader;
#[doc = "Field `PC_ACKD` reader - Protocol control state machine is in ACK Delimiter field."]
pub type PC_ACKD_R = crate::BitReader;
#[doc = "Field `PC_EOF` reader - Protocol control state machine is in End of file field."]
pub type PC_EOF_R = crate::BitReader;
#[doc = "Field `PC_INT` reader - Protocol control state machine is in Intermission field."]
pub type PC_INT_R = crate::BitReader;
#[doc = "Field `PC_SUSP` reader - Protocol control state machine is in Suspend transmission field."]
pub type PC_SUSP_R = crate::BitReader;
#[doc = "Field `PC_OVR` reader - Protocol control state machine is in Overload field."]
pub type PC_OVR_R = crate::BitReader;
#[doc = "Field `PC_SOF` reader - Protocol control state machine is in Start of frame field."]
pub type PC_SOF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Actual stuff count modulo 8 as definned in ISO FD protocol. Stuff count is erased in the beginning of CAN frame and increased by one with each stuff bit until Stuff count field in ISO FD frame. Then it stays fixed until the beginning of next frame. In non-ISO FD or normal CAN stuff bits are counted until the end of a frame. Note that this field is NOT gray encoded as defined in ISO FD standard. Stuff count is calculated only as long as controller is transceiving on the bus. During the reception this value remains fixed!"]
    #[inline(always)]
    pub fn stuff_count(&self) -> STUFF_COUNT_R {
        STUFF_COUNT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Actual de-stuff count modulo 8 as defined in ISO FD protocol. De-Stuff count is erased in the beginning of the frame and increased by one with each de-stuffed bit until Stuff count field in ISO FD Frame. Then it stays fixed until beginning of next frame. In non-ISO FD or normal CAN de-stuff bits are counted until the end of the frame. Note that this field is NOT grey encoded as defined in ISO FD standard. De-stuff count is calculated in both. Transceiver as well as receiver."]
    #[inline(always)]
    pub fn destuff_count(&self) -> DESTUFF_COUNT_R {
        DESTUFF_COUNT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Protocol control state machine is in Arbitration field."]
    #[inline(always)]
    pub fn pc_arb(&self) -> PC_ARB_R {
        PC_ARB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protocol control state machine is in Control field."]
    #[inline(always)]
    pub fn pc_con(&self) -> PC_CON_R {
        PC_CON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol control state machine is in Data field."]
    #[inline(always)]
    pub fn pc_dat(&self) -> PC_DAT_R {
        PC_DAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol control state machine is in Stuff Count field."]
    #[inline(always)]
    pub fn pc_stc(&self) -> PC_STC_R {
        PC_STC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protocol control state machine is in CRC field."]
    #[inline(always)]
    pub fn pc_crc(&self) -> PC_CRC_R {
        PC_CRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protocol control state machine is in CRC Delimiter field."]
    #[inline(always)]
    pub fn pc_crcd(&self) -> PC_CRCD_R {
        PC_CRCD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol control state machine is in ACK field."]
    #[inline(always)]
    pub fn pc_ack(&self) -> PC_ACK_R {
        PC_ACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protocol control state machine is in ACK Delimiter field."]
    #[inline(always)]
    pub fn pc_ackd(&self) -> PC_ACKD_R {
        PC_ACKD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol control state machine is in End of file field."]
    #[inline(always)]
    pub fn pc_eof(&self) -> PC_EOF_R {
        PC_EOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protocol control state machine is in Intermission field."]
    #[inline(always)]
    pub fn pc_int(&self) -> PC_INT_R {
        PC_INT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protocol control state machine is in Suspend transmission field."]
    #[inline(always)]
    pub fn pc_susp(&self) -> PC_SUSP_R {
        PC_SUSP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protocol control state machine is in Overload field."]
    #[inline(always)]
    pub fn pc_ovr(&self) -> PC_OVR_R {
        PC_OVR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protocol control state machine is in Start of frame field."]
    #[inline(always)]
    pub fn pc_sof(&self) -> PC_SOF_R {
        PC_SOF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG")
            .field("stuff_count", &self.stuff_count())
            .field("destuff_count", &self.destuff_count())
            .field("pc_arb", &self.pc_arb())
            .field("pc_con", &self.pc_con())
            .field("pc_dat", &self.pc_dat())
            .field("pc_stc", &self.pc_stc())
            .field("pc_crc", &self.pc_crc())
            .field("pc_crcd", &self.pc_crcd())
            .field("pc_ack", &self.pc_ack())
            .field("pc_ackd", &self.pc_ackd())
            .field("pc_eof", &self.pc_eof())
            .field("pc_int", &self.pc_int())
            .field("pc_susp", &self.pc_susp())
            .field("pc_ovr", &self.pc_ovr())
            .field("pc_sof", &self.pc_sof())
            .finish()
    }
}
#[doc = "TWAI FD debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SPEC;
impl crate::RegisterSpec for DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DEBUG_SPEC {}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DEBUG_SPEC {
    const RESET_VALUE: u32 = 0;
}
