///Register `DECODER_STATUS0` reader
pub type R = crate::R<DECODER_STATUS0_SPEC>;
///Field `DECODE_BYTE_CNT` reader - Reserved
pub type DECODE_BYTE_CNT_R = crate::FieldReader<u32>;
///Field `HEADER_DEC_ST` reader - Reserved
pub type HEADER_DEC_ST_R = crate::FieldReader;
///Field `DECODE_SAMPLE_SEL` reader - Reserved
pub type DECODE_SAMPLE_SEL_R = crate::FieldReader;
impl R {
    ///Bits 0:25 - Reserved
    #[inline(always)]
    pub fn decode_byte_cnt(&self) -> DECODE_BYTE_CNT_R {
        DECODE_BYTE_CNT_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:29 - Reserved
    #[inline(always)]
    pub fn header_dec_st(&self) -> HEADER_DEC_ST_R {
        HEADER_DEC_ST_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    ///Bits 30:31 - Reserved
    #[inline(always)]
    pub fn decode_sample_sel(&self) -> DECODE_SAMPLE_SEL_R {
        DECODE_SAMPLE_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECODER_STATUS0")
            .field("decode_byte_cnt", &self.decode_byte_cnt())
            .field("header_dec_st", &self.header_dec_st())
            .field("decode_sample_sel", &self.decode_sample_sel())
            .finish()
    }
}
/**Trace and Debug registers

You can [`read`](crate::generic::Reg::read) this register and get [`decoder_status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DECODER_STATUS0_SPEC;
impl crate::RegisterSpec for DECODER_STATUS0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`decoder_status0::R`](R) reader structure
impl crate::Readable for DECODER_STATUS0_SPEC {}
///`reset()` method sets DECODER_STATUS0 to value 0
impl crate::Resettable for DECODER_STATUS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
