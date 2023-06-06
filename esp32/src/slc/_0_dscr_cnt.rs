#[doc = "Register `_0_DSCR_CNT` reader"]
pub struct R(crate::R<_0_DSCR_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_DSCR_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_DSCR_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_DSCR_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_RX_DSCR_CNT_LAT` reader - "]
pub type SLC0_RX_DSCR_CNT_LAT_R = crate::FieldReader<u16>;
#[doc = "Field `SLC0_RX_GET_EOF_OCC` reader - "]
pub type SLC0_RX_GET_EOF_OCC_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn slc0_rx_dscr_cnt_lat(&self) -> SLC0_RX_DSCR_CNT_LAT_R {
        SLC0_RX_DSCR_CNT_LAT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rx_get_eof_occ(&self) -> SLC0_RX_GET_EOF_OCC_R {
        SLC0_RX_GET_EOF_OCC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_DSCR_CNT")
            .field(
                "slc0_rx_dscr_cnt_lat",
                &format_args!("{}", self.slc0_rx_dscr_cnt_lat().bits()),
            )
            .field(
                "slc0_rx_get_eof_occ",
                &format_args!("{}", self.slc0_rx_get_eof_occ().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0_DSCR_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_dscr_cnt](index.html) module"]
pub struct _0_DSCR_CNT_SPEC;
impl crate::RegisterSpec for _0_DSCR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_dscr_cnt::R](R) reader structure"]
impl crate::Readable for _0_DSCR_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _0_DSCR_CNT to value 0"]
impl crate::Resettable for _0_DSCR_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
