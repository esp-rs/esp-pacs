#[doc = "Register `HOST_SLC1HOST_INT_ENA1` reader"]
pub struct R(crate::R<HOST_SLC1HOST_INT_ENA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC1HOST_INT_ENA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC1HOST_INT_ENA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC1HOST_INT_ENA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLC1HOST_INT_ENA1` writer"]
pub struct W(crate::W<HOST_SLC1HOST_INT_ENA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLC1HOST_INT_ENA1_SPEC>;
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
impl From<crate::W<HOST_SLC1HOST_INT_ENA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLC1HOST_INT_ENA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC1_TOHOST_BIT0_INT_ENA1` reader - "]
pub type HOST_SLC1_TOHOST_BIT0_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOHOST_BIT0_INT_ENA1` writer - "]
pub type HOST_SLC1_TOHOST_BIT0_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOHOST_BIT1_INT_ENA1` reader - "]
pub type HOST_SLC1_TOHOST_BIT1_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOHOST_BIT1_INT_ENA1` writer - "]
pub type HOST_SLC1_TOHOST_BIT1_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOHOST_BIT2_INT_ENA1` reader - "]
pub type HOST_SLC1_TOHOST_BIT2_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOHOST_BIT2_INT_ENA1` writer - "]
pub type HOST_SLC1_TOHOST_BIT2_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOHOST_BIT3_INT_ENA1` reader - "]
pub type HOST_SLC1_TOHOST_BIT3_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOHOST_BIT3_INT_ENA1` writer - "]
pub type HOST_SLC1_TOHOST_BIT3_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOHOST_BIT4_INT_ENA1` reader - "]
pub type HOST_SLC1_TOHOST_BIT4_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOHOST_BIT4_INT_ENA1` writer - "]
pub type HOST_SLC1_TOHOST_BIT4_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOHOST_BIT5_INT_ENA1` reader - "]
pub type HOST_SLC1_TOHOST_BIT5_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOHOST_BIT5_INT_ENA1` writer - "]
pub type HOST_SLC1_TOHOST_BIT5_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOHOST_BIT6_INT_ENA1` reader - "]
pub type HOST_SLC1_TOHOST_BIT6_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOHOST_BIT6_INT_ENA1` writer - "]
pub type HOST_SLC1_TOHOST_BIT6_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOHOST_BIT7_INT_ENA1` reader - "]
pub type HOST_SLC1_TOHOST_BIT7_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOHOST_BIT7_INT_ENA1` writer - "]
pub type HOST_SLC1_TOHOST_BIT7_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOKEN0_1TO0_INT_ENA1` reader - "]
pub type HOST_SLC1_TOKEN0_1TO0_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOKEN0_1TO0_INT_ENA1` writer - "]
pub type HOST_SLC1_TOKEN0_1TO0_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOKEN1_1TO0_INT_ENA1` reader - "]
pub type HOST_SLC1_TOKEN1_1TO0_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOKEN1_1TO0_INT_ENA1` writer - "]
pub type HOST_SLC1_TOKEN1_1TO0_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOKEN0_0TO1_INT_ENA1` reader - "]
pub type HOST_SLC1_TOKEN0_0TO1_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOKEN0_0TO1_INT_ENA1` writer - "]
pub type HOST_SLC1_TOKEN0_0TO1_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TOKEN1_0TO1_INT_ENA1` reader - "]
pub type HOST_SLC1_TOKEN1_0TO1_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TOKEN1_0TO1_INT_ENA1` writer - "]
pub type HOST_SLC1_TOKEN1_0TO1_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1HOST_RX_SOF_INT_ENA1` reader - "]
pub type HOST_SLC1HOST_RX_SOF_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1HOST_RX_SOF_INT_ENA1` writer - "]
pub type HOST_SLC1HOST_RX_SOF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1HOST_RX_EOF_INT_ENA1` reader - "]
pub type HOST_SLC1HOST_RX_EOF_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1HOST_RX_EOF_INT_ENA1` writer - "]
pub type HOST_SLC1HOST_RX_EOF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1HOST_RX_START_INT_ENA1` reader - "]
pub type HOST_SLC1HOST_RX_START_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1HOST_RX_START_INT_ENA1` writer - "]
pub type HOST_SLC1HOST_RX_START_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1HOST_TX_START_INT_ENA1` reader - "]
pub type HOST_SLC1HOST_TX_START_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1HOST_TX_START_INT_ENA1` writer - "]
pub type HOST_SLC1HOST_TX_START_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_RX_UDF_INT_ENA1` reader - "]
pub type HOST_SLC1_RX_UDF_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_RX_UDF_INT_ENA1` writer - "]
pub type HOST_SLC1_RX_UDF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_TX_OVF_INT_ENA1` reader - "]
pub type HOST_SLC1_TX_OVF_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_TX_OVF_INT_ENA1` writer - "]
pub type HOST_SLC1_TX_OVF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_RX_PF_VALID_INT_ENA1` reader - "]
pub type HOST_SLC1_RX_PF_VALID_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_RX_PF_VALID_INT_ENA1` writer - "]
pub type HOST_SLC1_RX_PF_VALID_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_EXT_BIT0_INT_ENA1` reader - "]
pub type HOST_SLC1_EXT_BIT0_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_EXT_BIT0_INT_ENA1` writer - "]
pub type HOST_SLC1_EXT_BIT0_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_EXT_BIT1_INT_ENA1` reader - "]
pub type HOST_SLC1_EXT_BIT1_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_EXT_BIT1_INT_ENA1` writer - "]
pub type HOST_SLC1_EXT_BIT1_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_EXT_BIT2_INT_ENA1` reader - "]
pub type HOST_SLC1_EXT_BIT2_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_EXT_BIT2_INT_ENA1` writer - "]
pub type HOST_SLC1_EXT_BIT2_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_EXT_BIT3_INT_ENA1` reader - "]
pub type HOST_SLC1_EXT_BIT3_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_EXT_BIT3_INT_ENA1` writer - "]
pub type HOST_SLC1_EXT_BIT3_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_WIFI_RX_NEW_PACKET_INT_ENA1` reader - "]
pub type HOST_SLC1_WIFI_RX_NEW_PACKET_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_WIFI_RX_NEW_PACKET_INT_ENA1` writer - "]
pub type HOST_SLC1_WIFI_RX_NEW_PACKET_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_HOST_RD_RETRY_INT_ENA1` reader - "]
pub type HOST_SLC1_HOST_RD_RETRY_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_HOST_RD_RETRY_INT_ENA1` writer - "]
pub type HOST_SLC1_HOST_RD_RETRY_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
#[doc = "Field `HOST_SLC1_BT_RX_NEW_PACKET_INT_ENA1` reader - "]
pub type HOST_SLC1_BT_RX_NEW_PACKET_INT_ENA1_R = crate::BitReader;
#[doc = "Field `HOST_SLC1_BT_RX_NEW_PACKET_INT_ENA1` writer - "]
pub type HOST_SLC1_BT_RX_NEW_PACKET_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC1HOST_INT_ENA1_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc1_tohost_bit0_int_ena1(&self) -> HOST_SLC1_TOHOST_BIT0_INT_ENA1_R {
        HOST_SLC1_TOHOST_BIT0_INT_ENA1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc1_tohost_bit1_int_ena1(&self) -> HOST_SLC1_TOHOST_BIT1_INT_ENA1_R {
        HOST_SLC1_TOHOST_BIT1_INT_ENA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc1_tohost_bit2_int_ena1(&self) -> HOST_SLC1_TOHOST_BIT2_INT_ENA1_R {
        HOST_SLC1_TOHOST_BIT2_INT_ENA1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc1_tohost_bit3_int_ena1(&self) -> HOST_SLC1_TOHOST_BIT3_INT_ENA1_R {
        HOST_SLC1_TOHOST_BIT3_INT_ENA1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc1_tohost_bit4_int_ena1(&self) -> HOST_SLC1_TOHOST_BIT4_INT_ENA1_R {
        HOST_SLC1_TOHOST_BIT4_INT_ENA1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc1_tohost_bit5_int_ena1(&self) -> HOST_SLC1_TOHOST_BIT5_INT_ENA1_R {
        HOST_SLC1_TOHOST_BIT5_INT_ENA1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc1_tohost_bit6_int_ena1(&self) -> HOST_SLC1_TOHOST_BIT6_INT_ENA1_R {
        HOST_SLC1_TOHOST_BIT6_INT_ENA1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc1_tohost_bit7_int_ena1(&self) -> HOST_SLC1_TOHOST_BIT7_INT_ENA1_R {
        HOST_SLC1_TOHOST_BIT7_INT_ENA1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc1_token0_1to0_int_ena1(&self) -> HOST_SLC1_TOKEN0_1TO0_INT_ENA1_R {
        HOST_SLC1_TOKEN0_1TO0_INT_ENA1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn host_slc1_token1_1to0_int_ena1(&self) -> HOST_SLC1_TOKEN1_1TO0_INT_ENA1_R {
        HOST_SLC1_TOKEN1_1TO0_INT_ENA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn host_slc1_token0_0to1_int_ena1(&self) -> HOST_SLC1_TOKEN0_0TO1_INT_ENA1_R {
        HOST_SLC1_TOKEN0_0TO1_INT_ENA1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn host_slc1_token1_0to1_int_ena1(&self) -> HOST_SLC1_TOKEN1_0TO1_INT_ENA1_R {
        HOST_SLC1_TOKEN1_0TO1_INT_ENA1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc1host_rx_sof_int_ena1(&self) -> HOST_SLC1HOST_RX_SOF_INT_ENA1_R {
        HOST_SLC1HOST_RX_SOF_INT_ENA1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn host_slc1host_rx_eof_int_ena1(&self) -> HOST_SLC1HOST_RX_EOF_INT_ENA1_R {
        HOST_SLC1HOST_RX_EOF_INT_ENA1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn host_slc1host_rx_start_int_ena1(&self) -> HOST_SLC1HOST_RX_START_INT_ENA1_R {
        HOST_SLC1HOST_RX_START_INT_ENA1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn host_slc1host_tx_start_int_ena1(&self) -> HOST_SLC1HOST_TX_START_INT_ENA1_R {
        HOST_SLC1HOST_TX_START_INT_ENA1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn host_slc1_rx_udf_int_ena1(&self) -> HOST_SLC1_RX_UDF_INT_ENA1_R {
        HOST_SLC1_RX_UDF_INT_ENA1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn host_slc1_tx_ovf_int_ena1(&self) -> HOST_SLC1_TX_OVF_INT_ENA1_R {
        HOST_SLC1_TX_OVF_INT_ENA1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn host_slc1_rx_pf_valid_int_ena1(&self) -> HOST_SLC1_RX_PF_VALID_INT_ENA1_R {
        HOST_SLC1_RX_PF_VALID_INT_ENA1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_slc1_ext_bit0_int_ena1(&self) -> HOST_SLC1_EXT_BIT0_INT_ENA1_R {
        HOST_SLC1_EXT_BIT0_INT_ENA1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn host_slc1_ext_bit1_int_ena1(&self) -> HOST_SLC1_EXT_BIT1_INT_ENA1_R {
        HOST_SLC1_EXT_BIT1_INT_ENA1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn host_slc1_ext_bit2_int_ena1(&self) -> HOST_SLC1_EXT_BIT2_INT_ENA1_R {
        HOST_SLC1_EXT_BIT2_INT_ENA1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn host_slc1_ext_bit3_int_ena1(&self) -> HOST_SLC1_EXT_BIT3_INT_ENA1_R {
        HOST_SLC1_EXT_BIT3_INT_ENA1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn host_slc1_wifi_rx_new_packet_int_ena1(&self) -> HOST_SLC1_WIFI_RX_NEW_PACKET_INT_ENA1_R {
        HOST_SLC1_WIFI_RX_NEW_PACKET_INT_ENA1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn host_slc1_host_rd_retry_int_ena1(&self) -> HOST_SLC1_HOST_RD_RETRY_INT_ENA1_R {
        HOST_SLC1_HOST_RD_RETRY_INT_ENA1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_slc1_bt_rx_new_packet_int_ena1(&self) -> HOST_SLC1_BT_RX_NEW_PACKET_INT_ENA1_R {
        HOST_SLC1_BT_RX_NEW_PACKET_INT_ENA1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC1HOST_INT_ENA1")
            .field(
                "host_slc1_tohost_bit0_int_ena1",
                &format_args!("{}", self.host_slc1_tohost_bit0_int_ena1().bit()),
            )
            .field(
                "host_slc1_tohost_bit1_int_ena1",
                &format_args!("{}", self.host_slc1_tohost_bit1_int_ena1().bit()),
            )
            .field(
                "host_slc1_tohost_bit2_int_ena1",
                &format_args!("{}", self.host_slc1_tohost_bit2_int_ena1().bit()),
            )
            .field(
                "host_slc1_tohost_bit3_int_ena1",
                &format_args!("{}", self.host_slc1_tohost_bit3_int_ena1().bit()),
            )
            .field(
                "host_slc1_tohost_bit4_int_ena1",
                &format_args!("{}", self.host_slc1_tohost_bit4_int_ena1().bit()),
            )
            .field(
                "host_slc1_tohost_bit5_int_ena1",
                &format_args!("{}", self.host_slc1_tohost_bit5_int_ena1().bit()),
            )
            .field(
                "host_slc1_tohost_bit6_int_ena1",
                &format_args!("{}", self.host_slc1_tohost_bit6_int_ena1().bit()),
            )
            .field(
                "host_slc1_tohost_bit7_int_ena1",
                &format_args!("{}", self.host_slc1_tohost_bit7_int_ena1().bit()),
            )
            .field(
                "host_slc1_token0_1to0_int_ena1",
                &format_args!("{}", self.host_slc1_token0_1to0_int_ena1().bit()),
            )
            .field(
                "host_slc1_token1_1to0_int_ena1",
                &format_args!("{}", self.host_slc1_token1_1to0_int_ena1().bit()),
            )
            .field(
                "host_slc1_token0_0to1_int_ena1",
                &format_args!("{}", self.host_slc1_token0_0to1_int_ena1().bit()),
            )
            .field(
                "host_slc1_token1_0to1_int_ena1",
                &format_args!("{}", self.host_slc1_token1_0to1_int_ena1().bit()),
            )
            .field(
                "host_slc1host_rx_sof_int_ena1",
                &format_args!("{}", self.host_slc1host_rx_sof_int_ena1().bit()),
            )
            .field(
                "host_slc1host_rx_eof_int_ena1",
                &format_args!("{}", self.host_slc1host_rx_eof_int_ena1().bit()),
            )
            .field(
                "host_slc1host_rx_start_int_ena1",
                &format_args!("{}", self.host_slc1host_rx_start_int_ena1().bit()),
            )
            .field(
                "host_slc1host_tx_start_int_ena1",
                &format_args!("{}", self.host_slc1host_tx_start_int_ena1().bit()),
            )
            .field(
                "host_slc1_rx_udf_int_ena1",
                &format_args!("{}", self.host_slc1_rx_udf_int_ena1().bit()),
            )
            .field(
                "host_slc1_tx_ovf_int_ena1",
                &format_args!("{}", self.host_slc1_tx_ovf_int_ena1().bit()),
            )
            .field(
                "host_slc1_rx_pf_valid_int_ena1",
                &format_args!("{}", self.host_slc1_rx_pf_valid_int_ena1().bit()),
            )
            .field(
                "host_slc1_ext_bit0_int_ena1",
                &format_args!("{}", self.host_slc1_ext_bit0_int_ena1().bit()),
            )
            .field(
                "host_slc1_ext_bit1_int_ena1",
                &format_args!("{}", self.host_slc1_ext_bit1_int_ena1().bit()),
            )
            .field(
                "host_slc1_ext_bit2_int_ena1",
                &format_args!("{}", self.host_slc1_ext_bit2_int_ena1().bit()),
            )
            .field(
                "host_slc1_ext_bit3_int_ena1",
                &format_args!("{}", self.host_slc1_ext_bit3_int_ena1().bit()),
            )
            .field(
                "host_slc1_wifi_rx_new_packet_int_ena1",
                &format_args!("{}", self.host_slc1_wifi_rx_new_packet_int_ena1().bit()),
            )
            .field(
                "host_slc1_host_rd_retry_int_ena1",
                &format_args!("{}", self.host_slc1_host_rd_retry_int_ena1().bit()),
            )
            .field(
                "host_slc1_bt_rx_new_packet_int_ena1",
                &format_args!("{}", self.host_slc1_bt_rx_new_packet_int_ena1().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC1HOST_INT_ENA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit0_int_ena1(&mut self) -> HOST_SLC1_TOHOST_BIT0_INT_ENA1_W<0> {
        HOST_SLC1_TOHOST_BIT0_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit1_int_ena1(&mut self) -> HOST_SLC1_TOHOST_BIT1_INT_ENA1_W<1> {
        HOST_SLC1_TOHOST_BIT1_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit2_int_ena1(&mut self) -> HOST_SLC1_TOHOST_BIT2_INT_ENA1_W<2> {
        HOST_SLC1_TOHOST_BIT2_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit3_int_ena1(&mut self) -> HOST_SLC1_TOHOST_BIT3_INT_ENA1_W<3> {
        HOST_SLC1_TOHOST_BIT3_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit4_int_ena1(&mut self) -> HOST_SLC1_TOHOST_BIT4_INT_ENA1_W<4> {
        HOST_SLC1_TOHOST_BIT4_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit5_int_ena1(&mut self) -> HOST_SLC1_TOHOST_BIT5_INT_ENA1_W<5> {
        HOST_SLC1_TOHOST_BIT5_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit6_int_ena1(&mut self) -> HOST_SLC1_TOHOST_BIT6_INT_ENA1_W<6> {
        HOST_SLC1_TOHOST_BIT6_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit7_int_ena1(&mut self) -> HOST_SLC1_TOHOST_BIT7_INT_ENA1_W<7> {
        HOST_SLC1_TOHOST_BIT7_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_token0_1to0_int_ena1(&mut self) -> HOST_SLC1_TOKEN0_1TO0_INT_ENA1_W<8> {
        HOST_SLC1_TOKEN0_1TO0_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_token1_1to0_int_ena1(&mut self) -> HOST_SLC1_TOKEN1_1TO0_INT_ENA1_W<9> {
        HOST_SLC1_TOKEN1_1TO0_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_token0_0to1_int_ena1(&mut self) -> HOST_SLC1_TOKEN0_0TO1_INT_ENA1_W<10> {
        HOST_SLC1_TOKEN0_0TO1_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_token1_0to1_int_ena1(&mut self) -> HOST_SLC1_TOKEN1_0TO1_INT_ENA1_W<11> {
        HOST_SLC1_TOKEN1_0TO1_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_rx_sof_int_ena1(&mut self) -> HOST_SLC1HOST_RX_SOF_INT_ENA1_W<12> {
        HOST_SLC1HOST_RX_SOF_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_rx_eof_int_ena1(&mut self) -> HOST_SLC1HOST_RX_EOF_INT_ENA1_W<13> {
        HOST_SLC1HOST_RX_EOF_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_rx_start_int_ena1(&mut self) -> HOST_SLC1HOST_RX_START_INT_ENA1_W<14> {
        HOST_SLC1HOST_RX_START_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_tx_start_int_ena1(&mut self) -> HOST_SLC1HOST_TX_START_INT_ENA1_W<15> {
        HOST_SLC1HOST_TX_START_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_rx_udf_int_ena1(&mut self) -> HOST_SLC1_RX_UDF_INT_ENA1_W<16> {
        HOST_SLC1_RX_UDF_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tx_ovf_int_ena1(&mut self) -> HOST_SLC1_TX_OVF_INT_ENA1_W<17> {
        HOST_SLC1_TX_OVF_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_rx_pf_valid_int_ena1(&mut self) -> HOST_SLC1_RX_PF_VALID_INT_ENA1_W<18> {
        HOST_SLC1_RX_PF_VALID_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_ext_bit0_int_ena1(&mut self) -> HOST_SLC1_EXT_BIT0_INT_ENA1_W<19> {
        HOST_SLC1_EXT_BIT0_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_ext_bit1_int_ena1(&mut self) -> HOST_SLC1_EXT_BIT1_INT_ENA1_W<20> {
        HOST_SLC1_EXT_BIT1_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_ext_bit2_int_ena1(&mut self) -> HOST_SLC1_EXT_BIT2_INT_ENA1_W<21> {
        HOST_SLC1_EXT_BIT2_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_ext_bit3_int_ena1(&mut self) -> HOST_SLC1_EXT_BIT3_INT_ENA1_W<22> {
        HOST_SLC1_EXT_BIT3_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_wifi_rx_new_packet_int_ena1(
        &mut self,
    ) -> HOST_SLC1_WIFI_RX_NEW_PACKET_INT_ENA1_W<23> {
        HOST_SLC1_WIFI_RX_NEW_PACKET_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_host_rd_retry_int_ena1(&mut self) -> HOST_SLC1_HOST_RD_RETRY_INT_ENA1_W<24> {
        HOST_SLC1_HOST_RD_RETRY_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_bt_rx_new_packet_int_ena1(
        &mut self,
    ) -> HOST_SLC1_BT_RX_NEW_PACKET_INT_ENA1_W<25> {
        HOST_SLC1_BT_RX_NEW_PACKET_INT_ENA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc1host_int_ena1](index.html) module"]
pub struct HOST_SLC1HOST_INT_ENA1_SPEC;
impl crate::RegisterSpec for HOST_SLC1HOST_INT_ENA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc1host_int_ena1::R](R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_ENA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_ena1::W](W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_ENA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLC1HOST_INT_ENA1 to value 0"]
impl crate::Resettable for HOST_SLC1HOST_INT_ENA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
