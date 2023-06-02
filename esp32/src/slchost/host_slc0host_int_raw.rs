#[doc = "Register `HOST_SLC0HOST_INT_RAW` reader"]
pub struct R(crate::R<HOST_SLC0HOST_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC0HOST_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC0HOST_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC0HOST_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT0_INT_RAW` reader - "]
pub type HOST_SLC0_TOHOST_BIT0_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOHOST_BIT1_INT_RAW` reader - "]
pub type HOST_SLC0_TOHOST_BIT1_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOHOST_BIT2_INT_RAW` reader - "]
pub type HOST_SLC0_TOHOST_BIT2_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOHOST_BIT3_INT_RAW` reader - "]
pub type HOST_SLC0_TOHOST_BIT3_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOHOST_BIT4_INT_RAW` reader - "]
pub type HOST_SLC0_TOHOST_BIT4_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOHOST_BIT5_INT_RAW` reader - "]
pub type HOST_SLC0_TOHOST_BIT5_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOHOST_BIT6_INT_RAW` reader - "]
pub type HOST_SLC0_TOHOST_BIT6_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOHOST_BIT7_INT_RAW` reader - "]
pub type HOST_SLC0_TOHOST_BIT7_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOKEN0_1TO0_INT_RAW` reader - "]
pub type HOST_SLC0_TOKEN0_1TO0_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOKEN1_1TO0_INT_RAW` reader - "]
pub type HOST_SLC0_TOKEN1_1TO0_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOKEN0_0TO1_INT_RAW` reader - "]
pub type HOST_SLC0_TOKEN0_0TO1_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TOKEN1_0TO1_INT_RAW` reader - "]
pub type HOST_SLC0_TOKEN1_0TO1_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0HOST_RX_SOF_INT_RAW` reader - "]
pub type HOST_SLC0HOST_RX_SOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0HOST_RX_EOF_INT_RAW` reader - "]
pub type HOST_SLC0HOST_RX_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0HOST_RX_START_INT_RAW` reader - "]
pub type HOST_SLC0HOST_RX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0HOST_TX_START_INT_RAW` reader - "]
pub type HOST_SLC0HOST_TX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_RX_UDF_INT_RAW` reader - "]
pub type HOST_SLC0_RX_UDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_TX_OVF_INT_RAW` reader - "]
pub type HOST_SLC0_TX_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_RX_PF_VALID_INT_RAW` reader - "]
pub type HOST_SLC0_RX_PF_VALID_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_EXT_BIT0_INT_RAW` reader - "]
pub type HOST_SLC0_EXT_BIT0_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_EXT_BIT1_INT_RAW` reader - "]
pub type HOST_SLC0_EXT_BIT1_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_EXT_BIT2_INT_RAW` reader - "]
pub type HOST_SLC0_EXT_BIT2_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_EXT_BIT3_INT_RAW` reader - "]
pub type HOST_SLC0_EXT_BIT3_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_RX_NEW_PACKET_INT_RAW` reader - "]
pub type HOST_SLC0_RX_NEW_PACKET_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_SLC0_HOST_RD_RETRY_INT_RAW` reader - "]
pub type HOST_SLC0_HOST_RD_RETRY_INT_RAW_R = crate::BitReader;
#[doc = "Field `HOST_GPIO_SDIO_INT_RAW` reader - "]
pub type HOST_GPIO_SDIO_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit0_int_raw(&self) -> HOST_SLC0_TOHOST_BIT0_INT_RAW_R {
        HOST_SLC0_TOHOST_BIT0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit1_int_raw(&self) -> HOST_SLC0_TOHOST_BIT1_INT_RAW_R {
        HOST_SLC0_TOHOST_BIT1_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit2_int_raw(&self) -> HOST_SLC0_TOHOST_BIT2_INT_RAW_R {
        HOST_SLC0_TOHOST_BIT2_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit3_int_raw(&self) -> HOST_SLC0_TOHOST_BIT3_INT_RAW_R {
        HOST_SLC0_TOHOST_BIT3_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit4_int_raw(&self) -> HOST_SLC0_TOHOST_BIT4_INT_RAW_R {
        HOST_SLC0_TOHOST_BIT4_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit5_int_raw(&self) -> HOST_SLC0_TOHOST_BIT5_INT_RAW_R {
        HOST_SLC0_TOHOST_BIT5_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit6_int_raw(&self) -> HOST_SLC0_TOHOST_BIT6_INT_RAW_R {
        HOST_SLC0_TOHOST_BIT6_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit7_int_raw(&self) -> HOST_SLC0_TOHOST_BIT7_INT_RAW_R {
        HOST_SLC0_TOHOST_BIT7_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0_token0_1to0_int_raw(&self) -> HOST_SLC0_TOKEN0_1TO0_INT_RAW_R {
        HOST_SLC0_TOKEN0_1TO0_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn host_slc0_token1_1to0_int_raw(&self) -> HOST_SLC0_TOKEN1_1TO0_INT_RAW_R {
        HOST_SLC0_TOKEN1_1TO0_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn host_slc0_token0_0to1_int_raw(&self) -> HOST_SLC0_TOKEN0_0TO1_INT_RAW_R {
        HOST_SLC0_TOKEN0_0TO1_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn host_slc0_token1_0to1_int_raw(&self) -> HOST_SLC0_TOKEN1_0TO1_INT_RAW_R {
        HOST_SLC0_TOKEN1_0TO1_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0host_rx_sof_int_raw(&self) -> HOST_SLC0HOST_RX_SOF_INT_RAW_R {
        HOST_SLC0HOST_RX_SOF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn host_slc0host_rx_eof_int_raw(&self) -> HOST_SLC0HOST_RX_EOF_INT_RAW_R {
        HOST_SLC0HOST_RX_EOF_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn host_slc0host_rx_start_int_raw(&self) -> HOST_SLC0HOST_RX_START_INT_RAW_R {
        HOST_SLC0HOST_RX_START_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn host_slc0host_tx_start_int_raw(&self) -> HOST_SLC0HOST_TX_START_INT_RAW_R {
        HOST_SLC0HOST_TX_START_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn host_slc0_rx_udf_int_raw(&self) -> HOST_SLC0_RX_UDF_INT_RAW_R {
        HOST_SLC0_RX_UDF_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn host_slc0_tx_ovf_int_raw(&self) -> HOST_SLC0_TX_OVF_INT_RAW_R {
        HOST_SLC0_TX_OVF_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid_int_raw(&self) -> HOST_SLC0_RX_PF_VALID_INT_RAW_R {
        HOST_SLC0_RX_PF_VALID_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_slc0_ext_bit0_int_raw(&self) -> HOST_SLC0_EXT_BIT0_INT_RAW_R {
        HOST_SLC0_EXT_BIT0_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn host_slc0_ext_bit1_int_raw(&self) -> HOST_SLC0_EXT_BIT1_INT_RAW_R {
        HOST_SLC0_EXT_BIT1_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn host_slc0_ext_bit2_int_raw(&self) -> HOST_SLC0_EXT_BIT2_INT_RAW_R {
        HOST_SLC0_EXT_BIT2_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn host_slc0_ext_bit3_int_raw(&self) -> HOST_SLC0_EXT_BIT3_INT_RAW_R {
        HOST_SLC0_EXT_BIT3_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn host_slc0_rx_new_packet_int_raw(&self) -> HOST_SLC0_RX_NEW_PACKET_INT_RAW_R {
        HOST_SLC0_RX_NEW_PACKET_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn host_slc0_host_rd_retry_int_raw(&self) -> HOST_SLC0_HOST_RD_RETRY_INT_RAW_R {
        HOST_SLC0_HOST_RD_RETRY_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_gpio_sdio_int_raw(&self) -> HOST_GPIO_SDIO_INT_RAW_R {
        HOST_GPIO_SDIO_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC0HOST_INT_RAW")
            .field(
                "host_slc0_tohost_bit0_int_raw",
                &format_args!("{}", self.host_slc0_tohost_bit0_int_raw().bit()),
            )
            .field(
                "host_slc0_tohost_bit1_int_raw",
                &format_args!("{}", self.host_slc0_tohost_bit1_int_raw().bit()),
            )
            .field(
                "host_slc0_tohost_bit2_int_raw",
                &format_args!("{}", self.host_slc0_tohost_bit2_int_raw().bit()),
            )
            .field(
                "host_slc0_tohost_bit3_int_raw",
                &format_args!("{}", self.host_slc0_tohost_bit3_int_raw().bit()),
            )
            .field(
                "host_slc0_tohost_bit4_int_raw",
                &format_args!("{}", self.host_slc0_tohost_bit4_int_raw().bit()),
            )
            .field(
                "host_slc0_tohost_bit5_int_raw",
                &format_args!("{}", self.host_slc0_tohost_bit5_int_raw().bit()),
            )
            .field(
                "host_slc0_tohost_bit6_int_raw",
                &format_args!("{}", self.host_slc0_tohost_bit6_int_raw().bit()),
            )
            .field(
                "host_slc0_tohost_bit7_int_raw",
                &format_args!("{}", self.host_slc0_tohost_bit7_int_raw().bit()),
            )
            .field(
                "host_slc0_token0_1to0_int_raw",
                &format_args!("{}", self.host_slc0_token0_1to0_int_raw().bit()),
            )
            .field(
                "host_slc0_token1_1to0_int_raw",
                &format_args!("{}", self.host_slc0_token1_1to0_int_raw().bit()),
            )
            .field(
                "host_slc0_token0_0to1_int_raw",
                &format_args!("{}", self.host_slc0_token0_0to1_int_raw().bit()),
            )
            .field(
                "host_slc0_token1_0to1_int_raw",
                &format_args!("{}", self.host_slc0_token1_0to1_int_raw().bit()),
            )
            .field(
                "host_slc0host_rx_sof_int_raw",
                &format_args!("{}", self.host_slc0host_rx_sof_int_raw().bit()),
            )
            .field(
                "host_slc0host_rx_eof_int_raw",
                &format_args!("{}", self.host_slc0host_rx_eof_int_raw().bit()),
            )
            .field(
                "host_slc0host_rx_start_int_raw",
                &format_args!("{}", self.host_slc0host_rx_start_int_raw().bit()),
            )
            .field(
                "host_slc0host_tx_start_int_raw",
                &format_args!("{}", self.host_slc0host_tx_start_int_raw().bit()),
            )
            .field(
                "host_slc0_rx_udf_int_raw",
                &format_args!("{}", self.host_slc0_rx_udf_int_raw().bit()),
            )
            .field(
                "host_slc0_tx_ovf_int_raw",
                &format_args!("{}", self.host_slc0_tx_ovf_int_raw().bit()),
            )
            .field(
                "host_slc0_rx_pf_valid_int_raw",
                &format_args!("{}", self.host_slc0_rx_pf_valid_int_raw().bit()),
            )
            .field(
                "host_slc0_ext_bit0_int_raw",
                &format_args!("{}", self.host_slc0_ext_bit0_int_raw().bit()),
            )
            .field(
                "host_slc0_ext_bit1_int_raw",
                &format_args!("{}", self.host_slc0_ext_bit1_int_raw().bit()),
            )
            .field(
                "host_slc0_ext_bit2_int_raw",
                &format_args!("{}", self.host_slc0_ext_bit2_int_raw().bit()),
            )
            .field(
                "host_slc0_ext_bit3_int_raw",
                &format_args!("{}", self.host_slc0_ext_bit3_int_raw().bit()),
            )
            .field(
                "host_slc0_rx_new_packet_int_raw",
                &format_args!("{}", self.host_slc0_rx_new_packet_int_raw().bit()),
            )
            .field(
                "host_slc0_host_rd_retry_int_raw",
                &format_args!("{}", self.host_slc0_host_rd_retry_int_raw().bit()),
            )
            .field(
                "host_gpio_sdio_int_raw",
                &format_args!("{}", self.host_gpio_sdio_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC0HOST_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0host_int_raw](index.html) module"]
pub struct HOST_SLC0HOST_INT_RAW_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc0host_int_raw::R](R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_INT_RAW to value 0"]
impl crate::Resettable for HOST_SLC0HOST_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
