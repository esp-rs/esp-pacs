#[doc = "Register `STATUS0` reader"]
pub type R = crate::R<STATUS0_SPEC>;
#[doc = "Field `BITSTREAM_EOF_VLD_CNT` reader - the valid bit count for last bitstream"]
pub type BITSTREAM_EOF_VLD_CNT_R = crate::FieldReader;
#[doc = "Field `DCTOUT_ZZSCAN_ADDR` reader - the zig-zag read addr from dctout_ram"]
pub type DCTOUT_ZZSCAN_ADDR_R = crate::FieldReader;
#[doc = "Field `QNRVAL_ZZSCAN_ADDR` reader - the zig-zag read addr from qnrval_ram"]
pub type QNRVAL_ZZSCAN_ADDR_R = crate::FieldReader;
#[doc = "Field `REG_STATE_YUV` reader - the state of jpeg fsm"]
pub type REG_STATE_YUV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 11:16 - the valid bit count for last bitstream"]
    #[inline(always)]
    pub fn bitstream_eof_vld_cnt(&self) -> BITSTREAM_EOF_VLD_CNT_R {
        BITSTREAM_EOF_VLD_CNT_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:22 - the zig-zag read addr from dctout_ram"]
    #[inline(always)]
    pub fn dctout_zzscan_addr(&self) -> DCTOUT_ZZSCAN_ADDR_R {
        DCTOUT_ZZSCAN_ADDR_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - the zig-zag read addr from qnrval_ram"]
    #[inline(always)]
    pub fn qnrval_zzscan_addr(&self) -> QNRVAL_ZZSCAN_ADDR_R {
        QNRVAL_ZZSCAN_ADDR_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:31 - the state of jpeg fsm"]
    #[inline(always)]
    pub fn reg_state_yuv(&self) -> REG_STATE_YUV_R {
        REG_STATE_YUV_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS0")
            .field(
                "bitstream_eof_vld_cnt",
                &format_args!("{}", self.bitstream_eof_vld_cnt().bits()),
            )
            .field(
                "dctout_zzscan_addr",
                &format_args!("{}", self.dctout_zzscan_addr().bits()),
            )
            .field(
                "qnrval_zzscan_addr",
                &format_args!("{}", self.qnrval_zzscan_addr().bits()),
            )
            .field(
                "reg_state_yuv",
                &format_args!("{}", self.reg_state_yuv().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS0_SPEC;
impl crate::RegisterSpec for STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status0::R`](R) reader structure"]
impl crate::Readable for STATUS0_SPEC {}
#[doc = "`reset()` method sets STATUS0 to value 0"]
impl crate::Resettable for STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
