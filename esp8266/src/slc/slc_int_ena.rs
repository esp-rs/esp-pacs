#[doc = "Register `SLC_INT_ENA` reader"]
pub struct R(crate::R<SLC_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_INT_ENA` writer"]
pub struct W(crate::W<SLC_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SLC_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_FRHOST_BIT0_INT_ENA` reader - "]
pub type SLC_FRHOST_BIT0_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_FRHOST_BIT0_INT_ENA` writer - "]
pub type SLC_FRHOST_BIT0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_FRHOST_BIT1_INT_ENA` reader - "]
pub type SLC_FRHOST_BIT1_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_FRHOST_BIT1_INT_ENA` writer - "]
pub type SLC_FRHOST_BIT1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_FRHOST_BIT2_INT_ENA` reader - "]
pub type SLC_FRHOST_BIT2_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_FRHOST_BIT2_INT_ENA` writer - "]
pub type SLC_FRHOST_BIT2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_FRHOST_BIT3_INT_ENA` reader - "]
pub type SLC_FRHOST_BIT3_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_FRHOST_BIT3_INT_ENA` writer - "]
pub type SLC_FRHOST_BIT3_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_FRHOST_BIT4_INT_ENA` reader - "]
pub type SLC_FRHOST_BIT4_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_FRHOST_BIT4_INT_ENA` writer - "]
pub type SLC_FRHOST_BIT4_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_FRHOST_BIT5_INT_ENA` reader - "]
pub type SLC_FRHOST_BIT5_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_FRHOST_BIT5_INT_ENA` writer - "]
pub type SLC_FRHOST_BIT5_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_FRHOST_BIT6_INT_ENA` reader - "]
pub type SLC_FRHOST_BIT6_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_FRHOST_BIT6_INT_ENA` writer - "]
pub type SLC_FRHOST_BIT6_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_FRHOST_BIT7_INT_ENA` reader - "]
pub type SLC_FRHOST_BIT7_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_FRHOST_BIT7_INT_ENA` writer - "]
pub type SLC_FRHOST_BIT7_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_RX_START_INT_ENA` reader - "]
pub type SLC_RX_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_RX_START_INT_ENA` writer - "]
pub type SLC_RX_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TX_START_INT_ENA` reader - "]
pub type SLC_TX_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TX_START_INT_ENA` writer - "]
pub type SLC_TX_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_RX_UDF_INT_ENA` reader - "]
pub type SLC_RX_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_RX_UDF_INT_ENA` writer - "]
pub type SLC_RX_UDF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TX_OVF_INT_ENA` reader - "]
pub type SLC_TX_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TX_OVF_INT_ENA` writer - "]
pub type SLC_TX_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TOKEN0_1TO0_INT_ENA` reader - "]
pub type SLC_TOKEN0_1TO0_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN0_1TO0_INT_ENA` writer - "]
pub type SLC_TOKEN0_1TO0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TOKEN1_1TO0_INT_ENA` reader - "]
pub type SLC_TOKEN1_1TO0_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN1_1TO0_INT_ENA` writer - "]
pub type SLC_TOKEN1_1TO0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TX_DONE_INT_ENA` reader - "]
pub type SLC_TX_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TX_DONE_INT_ENA` writer - "]
pub type SLC_TX_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TX_EOF_INT_ENA` reader - "]
pub type SLC_TX_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TX_EOF_INT_ENA` writer - "]
pub type SLC_TX_EOF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_RX_DONE_INT_ENA` reader - "]
pub type SLC_RX_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_RX_DONE_INT_ENA` writer - "]
pub type SLC_RX_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_RX_EOF_INT_ENA` reader - "]
pub type SLC_RX_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_RX_EOF_INT_ENA` writer - "]
pub type SLC_RX_EOF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TOHOST_INT_ENA` reader - "]
pub type SLC_TOHOST_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TOHOST_INT_ENA` writer - "]
pub type SLC_TOHOST_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TX_DSCR_ERR_INT_ENA` reader - "]
pub type SLC_TX_DSCR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TX_DSCR_ERR_INT_ENA` writer - "]
pub type SLC_TX_DSCR_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_RX_DSCR_ERR_INT_ENA` reader - "]
pub type SLC_RX_DSCR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_RX_DSCR_ERR_INT_ENA` writer - "]
pub type SLC_RX_DSCR_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
#[doc = "Field `SLC_TX_DSCR_EMPTY_INT_ENA` reader - "]
pub type SLC_TX_DSCR_EMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC_TX_DSCR_EMPTY_INT_ENA` writer - "]
pub type SLC_TX_DSCR_EMPTY_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC_INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_frhost_bit0_int_ena(&self) -> SLC_FRHOST_BIT0_INT_ENA_R {
        SLC_FRHOST_BIT0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_frhost_bit1_int_ena(&self) -> SLC_FRHOST_BIT1_INT_ENA_R {
        SLC_FRHOST_BIT1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_frhost_bit2_int_ena(&self) -> SLC_FRHOST_BIT2_INT_ENA_R {
        SLC_FRHOST_BIT2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_frhost_bit3_int_ena(&self) -> SLC_FRHOST_BIT3_INT_ENA_R {
        SLC_FRHOST_BIT3_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_frhost_bit4_int_ena(&self) -> SLC_FRHOST_BIT4_INT_ENA_R {
        SLC_FRHOST_BIT4_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_frhost_bit5_int_ena(&self) -> SLC_FRHOST_BIT5_INT_ENA_R {
        SLC_FRHOST_BIT5_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_frhost_bit6_int_ena(&self) -> SLC_FRHOST_BIT6_INT_ENA_R {
        SLC_FRHOST_BIT6_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_frhost_bit7_int_ena(&self) -> SLC_FRHOST_BIT7_INT_ENA_R {
        SLC_FRHOST_BIT7_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_rx_start_int_ena(&self) -> SLC_RX_START_INT_ENA_R {
        SLC_RX_START_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_tx_start_int_ena(&self) -> SLC_TX_START_INT_ENA_R {
        SLC_TX_START_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_rx_udf_int_ena(&self) -> SLC_RX_UDF_INT_ENA_R {
        SLC_RX_UDF_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_tx_ovf_int_ena(&self) -> SLC_TX_OVF_INT_ENA_R {
        SLC_TX_OVF_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_token0_1to0_int_ena(&self) -> SLC_TOKEN0_1TO0_INT_ENA_R {
        SLC_TOKEN0_1TO0_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_token1_1to0_int_ena(&self) -> SLC_TOKEN1_1TO0_INT_ENA_R {
        SLC_TOKEN1_1TO0_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_tx_done_int_ena(&self) -> SLC_TX_DONE_INT_ENA_R {
        SLC_TX_DONE_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_tx_eof_int_ena(&self) -> SLC_TX_EOF_INT_ENA_R {
        SLC_TX_EOF_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_rx_done_int_ena(&self) -> SLC_RX_DONE_INT_ENA_R {
        SLC_RX_DONE_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_rx_eof_int_ena(&self) -> SLC_RX_EOF_INT_ENA_R {
        SLC_RX_EOF_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_tohost_int_ena(&self) -> SLC_TOHOST_INT_ENA_R {
        SLC_TOHOST_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_tx_dscr_err_int_ena(&self) -> SLC_TX_DSCR_ERR_INT_ENA_R {
        SLC_TX_DSCR_ERR_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_rx_dscr_err_int_ena(&self) -> SLC_RX_DSCR_ERR_INT_ENA_R {
        SLC_RX_DSCR_ERR_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_tx_dscr_empty_int_ena(&self) -> SLC_TX_DSCR_EMPTY_INT_ENA_R {
        SLC_TX_DSCR_EMPTY_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_INT_ENA")
            .field(
                "slc_tx_dscr_empty_int_ena",
                &format_args!("{}", self.slc_tx_dscr_empty_int_ena().bit()),
            )
            .field(
                "slc_rx_dscr_err_int_ena",
                &format_args!("{}", self.slc_rx_dscr_err_int_ena().bit()),
            )
            .field(
                "slc_tx_dscr_err_int_ena",
                &format_args!("{}", self.slc_tx_dscr_err_int_ena().bit()),
            )
            .field(
                "slc_tohost_int_ena",
                &format_args!("{}", self.slc_tohost_int_ena().bit()),
            )
            .field(
                "slc_rx_eof_int_ena",
                &format_args!("{}", self.slc_rx_eof_int_ena().bit()),
            )
            .field(
                "slc_rx_done_int_ena",
                &format_args!("{}", self.slc_rx_done_int_ena().bit()),
            )
            .field(
                "slc_tx_eof_int_ena",
                &format_args!("{}", self.slc_tx_eof_int_ena().bit()),
            )
            .field(
                "slc_tx_done_int_ena",
                &format_args!("{}", self.slc_tx_done_int_ena().bit()),
            )
            .field(
                "slc_token1_1to0_int_ena",
                &format_args!("{}", self.slc_token1_1to0_int_ena().bit()),
            )
            .field(
                "slc_token0_1to0_int_ena",
                &format_args!("{}", self.slc_token0_1to0_int_ena().bit()),
            )
            .field(
                "slc_tx_ovf_int_ena",
                &format_args!("{}", self.slc_tx_ovf_int_ena().bit()),
            )
            .field(
                "slc_rx_udf_int_ena",
                &format_args!("{}", self.slc_rx_udf_int_ena().bit()),
            )
            .field(
                "slc_tx_start_int_ena",
                &format_args!("{}", self.slc_tx_start_int_ena().bit()),
            )
            .field(
                "slc_rx_start_int_ena",
                &format_args!("{}", self.slc_rx_start_int_ena().bit()),
            )
            .field(
                "slc_frhost_bit7_int_ena",
                &format_args!("{}", self.slc_frhost_bit7_int_ena().bit()),
            )
            .field(
                "slc_frhost_bit6_int_ena",
                &format_args!("{}", self.slc_frhost_bit6_int_ena().bit()),
            )
            .field(
                "slc_frhost_bit5_int_ena",
                &format_args!("{}", self.slc_frhost_bit5_int_ena().bit()),
            )
            .field(
                "slc_frhost_bit4_int_ena",
                &format_args!("{}", self.slc_frhost_bit4_int_ena().bit()),
            )
            .field(
                "slc_frhost_bit3_int_ena",
                &format_args!("{}", self.slc_frhost_bit3_int_ena().bit()),
            )
            .field(
                "slc_frhost_bit2_int_ena",
                &format_args!("{}", self.slc_frhost_bit2_int_ena().bit()),
            )
            .field(
                "slc_frhost_bit1_int_ena",
                &format_args!("{}", self.slc_frhost_bit1_int_ena().bit()),
            )
            .field(
                "slc_frhost_bit0_int_ena",
                &format_args!("{}", self.slc_frhost_bit0_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn slc_frhost_bit0_int_ena(&mut self) -> SLC_FRHOST_BIT0_INT_ENA_W<0> {
        SLC_FRHOST_BIT0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn slc_frhost_bit1_int_ena(&mut self) -> SLC_FRHOST_BIT1_INT_ENA_W<1> {
        SLC_FRHOST_BIT1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn slc_frhost_bit2_int_ena(&mut self) -> SLC_FRHOST_BIT2_INT_ENA_W<2> {
        SLC_FRHOST_BIT2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn slc_frhost_bit3_int_ena(&mut self) -> SLC_FRHOST_BIT3_INT_ENA_W<3> {
        SLC_FRHOST_BIT3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn slc_frhost_bit4_int_ena(&mut self) -> SLC_FRHOST_BIT4_INT_ENA_W<4> {
        SLC_FRHOST_BIT4_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn slc_frhost_bit5_int_ena(&mut self) -> SLC_FRHOST_BIT5_INT_ENA_W<5> {
        SLC_FRHOST_BIT5_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn slc_frhost_bit6_int_ena(&mut self) -> SLC_FRHOST_BIT6_INT_ENA_W<6> {
        SLC_FRHOST_BIT6_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn slc_frhost_bit7_int_ena(&mut self) -> SLC_FRHOST_BIT7_INT_ENA_W<7> {
        SLC_FRHOST_BIT7_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_start_int_ena(&mut self) -> SLC_RX_START_INT_ENA_W<8> {
        SLC_RX_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_start_int_ena(&mut self) -> SLC_TX_START_INT_ENA_W<9> {
        SLC_TX_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_udf_int_ena(&mut self) -> SLC_RX_UDF_INT_ENA_W<10> {
        SLC_RX_UDF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_ovf_int_ena(&mut self) -> SLC_TX_OVF_INT_ENA_W<11> {
        SLC_TX_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token0_1to0_int_ena(&mut self) -> SLC_TOKEN0_1TO0_INT_ENA_W<12> {
        SLC_TOKEN0_1TO0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token1_1to0_int_ena(&mut self) -> SLC_TOKEN1_1TO0_INT_ENA_W<13> {
        SLC_TOKEN1_1TO0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_done_int_ena(&mut self) -> SLC_TX_DONE_INT_ENA_W<14> {
        SLC_TX_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_eof_int_ena(&mut self) -> SLC_TX_EOF_INT_ENA_W<15> {
        SLC_TX_EOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_done_int_ena(&mut self) -> SLC_RX_DONE_INT_ENA_W<16> {
        SLC_RX_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_eof_int_ena(&mut self) -> SLC_RX_EOF_INT_ENA_W<17> {
        SLC_RX_EOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tohost_int_ena(&mut self) -> SLC_TOHOST_INT_ENA_W<18> {
        SLC_TOHOST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_dscr_err_int_ena(&mut self) -> SLC_TX_DSCR_ERR_INT_ENA_W<19> {
        SLC_TX_DSCR_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_dscr_err_int_ena(&mut self) -> SLC_RX_DSCR_ERR_INT_ENA_W<20> {
        SLC_RX_DSCR_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_dscr_empty_int_ena(&mut self) -> SLC_TX_DSCR_EMPTY_INT_ENA_W<21> {
        SLC_TX_DSCR_EMPTY_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_int_ena](index.html) module"]
pub struct SLC_INT_ENA_SPEC;
impl crate::RegisterSpec for SLC_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_int_ena::R](R) reader structure"]
impl crate::Readable for SLC_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_int_ena::W](W) writer structure"]
impl crate::Writable for SLC_INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_INT_ENA to value 0"]
impl crate::Resettable for SLC_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
