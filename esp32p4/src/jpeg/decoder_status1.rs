///Register `DECODER_STATUS1` reader
pub type R = crate::R<DECODER_STATUS1_SPEC>;
///Field `ENCODE_DATA` reader - Reserved
pub type ENCODE_DATA_R = crate::FieldReader<u16>;
///Field `COUNT_Q` reader - Reserved
pub type COUNT_Q_R = crate::FieldReader;
///Field `MCU_FSM_READY` reader - Reserved
pub type MCU_FSM_READY_R = crate::BitReader;
///Field `DECODE_DATA` reader - Reserved
pub type DECODE_DATA_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Reserved
    #[inline(always)]
    pub fn encode_data(&self) -> ENCODE_DATA_R {
        ENCODE_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:22 - Reserved
    #[inline(always)]
    pub fn count_q(&self) -> COUNT_Q_R {
        COUNT_Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - Reserved
    #[inline(always)]
    pub fn mcu_fsm_ready(&self) -> MCU_FSM_READY_R {
        MCU_FSM_READY_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - Reserved
    #[inline(always)]
    pub fn decode_data(&self) -> DECODE_DATA_R {
        DECODE_DATA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECODER_STATUS1")
            .field("encode_data", &self.encode_data())
            .field("count_q", &self.count_q())
            .field("mcu_fsm_ready", &self.mcu_fsm_ready())
            .field("decode_data", &self.decode_data())
            .finish()
    }
}
/**Trace and Debug registers

You can [`read`](crate::generic::Reg::read) this register and get [`decoder_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DECODER_STATUS1_SPEC;
impl crate::RegisterSpec for DECODER_STATUS1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`decoder_status1::R`](R) reader structure
impl crate::Readable for DECODER_STATUS1_SPEC {}
///`reset()` method sets DECODER_STATUS1 to value 0
impl crate::Resettable for DECODER_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
