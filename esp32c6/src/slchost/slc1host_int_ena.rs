#[doc = "Register `SLC1HOST_INT_ENA` reader"]
pub struct R(crate::R<SLC1HOST_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC1HOST_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC1HOST_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC1HOST_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC1HOST_INT_ENA` writer"]
pub struct W(crate::W<SLC1HOST_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC1HOST_INT_ENA_SPEC>;
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
impl From<crate::W<SLC1HOST_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC1HOST_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC1_TOHOST_BIT0_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOHOST_BIT0_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT0_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOHOST_BIT0_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOHOST_BIT1_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOHOST_BIT1_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT1_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOHOST_BIT1_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOHOST_BIT2_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOHOST_BIT2_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT2_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOHOST_BIT2_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOHOST_BIT3_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOHOST_BIT3_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT3_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOHOST_BIT3_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOHOST_BIT4_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOHOST_BIT4_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT4_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOHOST_BIT4_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOHOST_BIT5_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOHOST_BIT5_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT5_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOHOST_BIT5_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOHOST_BIT6_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOHOST_BIT6_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT6_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOHOST_BIT6_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOHOST_BIT7_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOHOST_BIT7_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT7_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOHOST_BIT7_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOKEN0_1TO0_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOKEN0_1TO0_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN0_1TO0_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOKEN0_1TO0_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOKEN1_1TO0_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOKEN1_1TO0_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN1_1TO0_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOKEN1_1TO0_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOKEN0_0TO1_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOKEN0_0TO1_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN0_0TO1_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOKEN0_0TO1_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TOKEN1_0TO1_INT_ENA` reader - *******Description***********"]
pub type SLC1_TOKEN1_0TO1_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN1_0TO1_INT_ENA` writer - *******Description***********"]
pub type SLC1_TOKEN1_0TO1_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1HOST_RX_SOF_INT_ENA` reader - *******Description***********"]
pub type SLC1HOST_RX_SOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1HOST_RX_SOF_INT_ENA` writer - *******Description***********"]
pub type SLC1HOST_RX_SOF_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1HOST_RX_EOF_INT_ENA` reader - *******Description***********"]
pub type SLC1HOST_RX_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1HOST_RX_EOF_INT_ENA` writer - *******Description***********"]
pub type SLC1HOST_RX_EOF_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1HOST_RX_START_INT_ENA` reader - *******Description***********"]
pub type SLC1HOST_RX_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1HOST_RX_START_INT_ENA` writer - *******Description***********"]
pub type SLC1HOST_RX_START_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1HOST_TX_START_INT_ENA` reader - *******Description***********"]
pub type SLC1HOST_TX_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1HOST_TX_START_INT_ENA` writer - *******Description***********"]
pub type SLC1HOST_TX_START_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_RX_UDF_INT_ENA` reader - *******Description***********"]
pub type SLC1_RX_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_RX_UDF_INT_ENA` writer - *******Description***********"]
pub type SLC1_RX_UDF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_TX_OVF_INT_ENA` reader - *******Description***********"]
pub type SLC1_TX_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_TX_OVF_INT_ENA` writer - *******Description***********"]
pub type SLC1_TX_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_RX_PF_VALID_INT_ENA` reader - *******Description***********"]
pub type SLC1_RX_PF_VALID_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_RX_PF_VALID_INT_ENA` writer - *******Description***********"]
pub type SLC1_RX_PF_VALID_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_EXT_BIT0_INT_ENA` reader - *******Description***********"]
pub type SLC1_EXT_BIT0_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_EXT_BIT0_INT_ENA` writer - *******Description***********"]
pub type SLC1_EXT_BIT0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_EXT_BIT1_INT_ENA` reader - *******Description***********"]
pub type SLC1_EXT_BIT1_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_EXT_BIT1_INT_ENA` writer - *******Description***********"]
pub type SLC1_EXT_BIT1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_EXT_BIT2_INT_ENA` reader - *******Description***********"]
pub type SLC1_EXT_BIT2_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_EXT_BIT2_INT_ENA` writer - *******Description***********"]
pub type SLC1_EXT_BIT2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_EXT_BIT3_INT_ENA` reader - *******Description***********"]
pub type SLC1_EXT_BIT3_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_EXT_BIT3_INT_ENA` writer - *******Description***********"]
pub type SLC1_EXT_BIT3_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_WIFI_RX_NEW_PACKET_INT_ENA` reader - *******Description***********"]
pub type SLC1_WIFI_RX_NEW_PACKET_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_WIFI_RX_NEW_PACKET_INT_ENA` writer - *******Description***********"]
pub type SLC1_WIFI_RX_NEW_PACKET_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_HOST_RD_RETRY_INT_ENA` reader - *******Description***********"]
pub type SLC1_HOST_RD_RETRY_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_HOST_RD_RETRY_INT_ENA` writer - *******Description***********"]
pub type SLC1_HOST_RD_RETRY_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
#[doc = "Field `SLC1_BT_RX_NEW_PACKET_INT_ENA` reader - *******Description***********"]
pub type SLC1_BT_RX_NEW_PACKET_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLC1_BT_RX_NEW_PACKET_INT_ENA` writer - *******Description***********"]
pub type SLC1_BT_RX_NEW_PACKET_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC1HOST_INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tohost_bit0_int_ena(&self) -> SLC1_TOHOST_BIT0_INT_ENA_R {
        SLC1_TOHOST_BIT0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tohost_bit1_int_ena(&self) -> SLC1_TOHOST_BIT1_INT_ENA_R {
        SLC1_TOHOST_BIT1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tohost_bit2_int_ena(&self) -> SLC1_TOHOST_BIT2_INT_ENA_R {
        SLC1_TOHOST_BIT2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tohost_bit3_int_ena(&self) -> SLC1_TOHOST_BIT3_INT_ENA_R {
        SLC1_TOHOST_BIT3_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tohost_bit4_int_ena(&self) -> SLC1_TOHOST_BIT4_INT_ENA_R {
        SLC1_TOHOST_BIT4_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tohost_bit5_int_ena(&self) -> SLC1_TOHOST_BIT5_INT_ENA_R {
        SLC1_TOHOST_BIT5_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tohost_bit6_int_ena(&self) -> SLC1_TOHOST_BIT6_INT_ENA_R {
        SLC1_TOHOST_BIT6_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tohost_bit7_int_ena(&self) -> SLC1_TOHOST_BIT7_INT_ENA_R {
        SLC1_TOHOST_BIT7_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_token0_1to0_int_ena(&self) -> SLC1_TOKEN0_1TO0_INT_ENA_R {
        SLC1_TOKEN0_1TO0_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_token1_1to0_int_ena(&self) -> SLC1_TOKEN1_1TO0_INT_ENA_R {
        SLC1_TOKEN1_1TO0_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_token0_0to1_int_ena(&self) -> SLC1_TOKEN0_0TO1_INT_ENA_R {
        SLC1_TOKEN0_0TO1_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_token1_0to1_int_ena(&self) -> SLC1_TOKEN1_0TO1_INT_ENA_R {
        SLC1_TOKEN1_0TO1_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - *******Description***********"]
    #[inline(always)]
    pub fn slc1host_rx_sof_int_ena(&self) -> SLC1HOST_RX_SOF_INT_ENA_R {
        SLC1HOST_RX_SOF_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - *******Description***********"]
    #[inline(always)]
    pub fn slc1host_rx_eof_int_ena(&self) -> SLC1HOST_RX_EOF_INT_ENA_R {
        SLC1HOST_RX_EOF_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - *******Description***********"]
    #[inline(always)]
    pub fn slc1host_rx_start_int_ena(&self) -> SLC1HOST_RX_START_INT_ENA_R {
        SLC1HOST_RX_START_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - *******Description***********"]
    #[inline(always)]
    pub fn slc1host_tx_start_int_ena(&self) -> SLC1HOST_TX_START_INT_ENA_R {
        SLC1HOST_TX_START_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_rx_udf_int_ena(&self) -> SLC1_RX_UDF_INT_ENA_R {
        SLC1_RX_UDF_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_tx_ovf_int_ena(&self) -> SLC1_TX_OVF_INT_ENA_R {
        SLC1_TX_OVF_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_rx_pf_valid_int_ena(&self) -> SLC1_RX_PF_VALID_INT_ENA_R {
        SLC1_RX_PF_VALID_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_ext_bit0_int_ena(&self) -> SLC1_EXT_BIT0_INT_ENA_R {
        SLC1_EXT_BIT0_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_ext_bit1_int_ena(&self) -> SLC1_EXT_BIT1_INT_ENA_R {
        SLC1_EXT_BIT1_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_ext_bit2_int_ena(&self) -> SLC1_EXT_BIT2_INT_ENA_R {
        SLC1_EXT_BIT2_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_ext_bit3_int_ena(&self) -> SLC1_EXT_BIT3_INT_ENA_R {
        SLC1_EXT_BIT3_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_wifi_rx_new_packet_int_ena(&self) -> SLC1_WIFI_RX_NEW_PACKET_INT_ENA_R {
        SLC1_WIFI_RX_NEW_PACKET_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_host_rd_retry_int_ena(&self) -> SLC1_HOST_RD_RETRY_INT_ENA_R {
        SLC1_HOST_RD_RETRY_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_bt_rx_new_packet_int_ena(&self) -> SLC1_BT_RX_NEW_PACKET_INT_ENA_R {
        SLC1_BT_RX_NEW_PACKET_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC1HOST_INT_ENA")
            .field(
                "slc1_tohost_bit0_int_ena",
                &format_args!("{}", self.slc1_tohost_bit0_int_ena().bit()),
            )
            .field(
                "slc1_tohost_bit1_int_ena",
                &format_args!("{}", self.slc1_tohost_bit1_int_ena().bit()),
            )
            .field(
                "slc1_tohost_bit2_int_ena",
                &format_args!("{}", self.slc1_tohost_bit2_int_ena().bit()),
            )
            .field(
                "slc1_tohost_bit3_int_ena",
                &format_args!("{}", self.slc1_tohost_bit3_int_ena().bit()),
            )
            .field(
                "slc1_tohost_bit4_int_ena",
                &format_args!("{}", self.slc1_tohost_bit4_int_ena().bit()),
            )
            .field(
                "slc1_tohost_bit5_int_ena",
                &format_args!("{}", self.slc1_tohost_bit5_int_ena().bit()),
            )
            .field(
                "slc1_tohost_bit6_int_ena",
                &format_args!("{}", self.slc1_tohost_bit6_int_ena().bit()),
            )
            .field(
                "slc1_tohost_bit7_int_ena",
                &format_args!("{}", self.slc1_tohost_bit7_int_ena().bit()),
            )
            .field(
                "slc1_token0_1to0_int_ena",
                &format_args!("{}", self.slc1_token0_1to0_int_ena().bit()),
            )
            .field(
                "slc1_token1_1to0_int_ena",
                &format_args!("{}", self.slc1_token1_1to0_int_ena().bit()),
            )
            .field(
                "slc1_token0_0to1_int_ena",
                &format_args!("{}", self.slc1_token0_0to1_int_ena().bit()),
            )
            .field(
                "slc1_token1_0to1_int_ena",
                &format_args!("{}", self.slc1_token1_0to1_int_ena().bit()),
            )
            .field(
                "slc1host_rx_sof_int_ena",
                &format_args!("{}", self.slc1host_rx_sof_int_ena().bit()),
            )
            .field(
                "slc1host_rx_eof_int_ena",
                &format_args!("{}", self.slc1host_rx_eof_int_ena().bit()),
            )
            .field(
                "slc1host_rx_start_int_ena",
                &format_args!("{}", self.slc1host_rx_start_int_ena().bit()),
            )
            .field(
                "slc1host_tx_start_int_ena",
                &format_args!("{}", self.slc1host_tx_start_int_ena().bit()),
            )
            .field(
                "slc1_rx_udf_int_ena",
                &format_args!("{}", self.slc1_rx_udf_int_ena().bit()),
            )
            .field(
                "slc1_tx_ovf_int_ena",
                &format_args!("{}", self.slc1_tx_ovf_int_ena().bit()),
            )
            .field(
                "slc1_rx_pf_valid_int_ena",
                &format_args!("{}", self.slc1_rx_pf_valid_int_ena().bit()),
            )
            .field(
                "slc1_ext_bit0_int_ena",
                &format_args!("{}", self.slc1_ext_bit0_int_ena().bit()),
            )
            .field(
                "slc1_ext_bit1_int_ena",
                &format_args!("{}", self.slc1_ext_bit1_int_ena().bit()),
            )
            .field(
                "slc1_ext_bit2_int_ena",
                &format_args!("{}", self.slc1_ext_bit2_int_ena().bit()),
            )
            .field(
                "slc1_ext_bit3_int_ena",
                &format_args!("{}", self.slc1_ext_bit3_int_ena().bit()),
            )
            .field(
                "slc1_wifi_rx_new_packet_int_ena",
                &format_args!("{}", self.slc1_wifi_rx_new_packet_int_ena().bit()),
            )
            .field(
                "slc1_host_rd_retry_int_ena",
                &format_args!("{}", self.slc1_host_rd_retry_int_ena().bit()),
            )
            .field(
                "slc1_bt_rx_new_packet_int_ena",
                &format_args!("{}", self.slc1_bt_rx_new_packet_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC1HOST_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_bit0_int_ena(&mut self) -> SLC1_TOHOST_BIT0_INT_ENA_W<0> {
        SLC1_TOHOST_BIT0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_bit1_int_ena(&mut self) -> SLC1_TOHOST_BIT1_INT_ENA_W<1> {
        SLC1_TOHOST_BIT1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_bit2_int_ena(&mut self) -> SLC1_TOHOST_BIT2_INT_ENA_W<2> {
        SLC1_TOHOST_BIT2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_bit3_int_ena(&mut self) -> SLC1_TOHOST_BIT3_INT_ENA_W<3> {
        SLC1_TOHOST_BIT3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_bit4_int_ena(&mut self) -> SLC1_TOHOST_BIT4_INT_ENA_W<4> {
        SLC1_TOHOST_BIT4_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_bit5_int_ena(&mut self) -> SLC1_TOHOST_BIT5_INT_ENA_W<5> {
        SLC1_TOHOST_BIT5_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_bit6_int_ena(&mut self) -> SLC1_TOHOST_BIT6_INT_ENA_W<6> {
        SLC1_TOHOST_BIT6_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_bit7_int_ena(&mut self) -> SLC1_TOHOST_BIT7_INT_ENA_W<7> {
        SLC1_TOHOST_BIT7_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token0_1to0_int_ena(&mut self) -> SLC1_TOKEN0_1TO0_INT_ENA_W<8> {
        SLC1_TOKEN0_1TO0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token1_1to0_int_ena(&mut self) -> SLC1_TOKEN1_1TO0_INT_ENA_W<9> {
        SLC1_TOKEN1_1TO0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token0_0to1_int_ena(&mut self) -> SLC1_TOKEN0_0TO1_INT_ENA_W<10> {
        SLC1_TOKEN0_0TO1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token1_0to1_int_ena(&mut self) -> SLC1_TOKEN1_0TO1_INT_ENA_W<11> {
        SLC1_TOKEN1_0TO1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_rx_sof_int_ena(&mut self) -> SLC1HOST_RX_SOF_INT_ENA_W<12> {
        SLC1HOST_RX_SOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_rx_eof_int_ena(&mut self) -> SLC1HOST_RX_EOF_INT_ENA_W<13> {
        SLC1HOST_RX_EOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_rx_start_int_ena(&mut self) -> SLC1HOST_RX_START_INT_ENA_W<14> {
        SLC1HOST_RX_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_tx_start_int_ena(&mut self) -> SLC1HOST_TX_START_INT_ENA_W<15> {
        SLC1HOST_TX_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_udf_int_ena(&mut self) -> SLC1_RX_UDF_INT_ENA_W<16> {
        SLC1_RX_UDF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_ovf_int_ena(&mut self) -> SLC1_TX_OVF_INT_ENA_W<17> {
        SLC1_TX_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_pf_valid_int_ena(&mut self) -> SLC1_RX_PF_VALID_INT_ENA_W<18> {
        SLC1_RX_PF_VALID_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_ext_bit0_int_ena(&mut self) -> SLC1_EXT_BIT0_INT_ENA_W<19> {
        SLC1_EXT_BIT0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_ext_bit1_int_ena(&mut self) -> SLC1_EXT_BIT1_INT_ENA_W<20> {
        SLC1_EXT_BIT1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 21 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_ext_bit2_int_ena(&mut self) -> SLC1_EXT_BIT2_INT_ENA_W<21> {
        SLC1_EXT_BIT2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 22 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_ext_bit3_int_ena(&mut self) -> SLC1_EXT_BIT3_INT_ENA_W<22> {
        SLC1_EXT_BIT3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_wifi_rx_new_packet_int_ena(&mut self) -> SLC1_WIFI_RX_NEW_PACKET_INT_ENA_W<23> {
        SLC1_WIFI_RX_NEW_PACKET_INT_ENA_W::new(self)
    }
    #[doc = "Bit 24 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_host_rd_retry_int_ena(&mut self) -> SLC1_HOST_RD_RETRY_INT_ENA_W<24> {
        SLC1_HOST_RD_RETRY_INT_ENA_W::new(self)
    }
    #[doc = "Bit 25 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_bt_rx_new_packet_int_ena(&mut self) -> SLC1_BT_RX_NEW_PACKET_INT_ENA_W<25> {
        SLC1_BT_RX_NEW_PACKET_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc1host_int_ena](index.html) module"]
pub struct SLC1HOST_INT_ENA_SPEC;
impl crate::RegisterSpec for SLC1HOST_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc1host_int_ena::R](R) reader structure"]
impl crate::Readable for SLC1HOST_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc1host_int_ena::W](W) writer structure"]
impl crate::Writable for SLC1HOST_INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC1HOST_INT_ENA to value 0"]
impl crate::Resettable for SLC1HOST_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
