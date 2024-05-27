///Register `HOST_SLC1HOST_INT_CLR` writer
pub type W = crate::W<HOST_SLC1HOST_INT_CLR_SPEC>;
///Field `HOST_SLC1_TOHOST_BIT0_INT_CLR` writer -
pub type HOST_SLC1_TOHOST_BIT0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOHOST_BIT1_INT_CLR` writer -
pub type HOST_SLC1_TOHOST_BIT1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOHOST_BIT2_INT_CLR` writer -
pub type HOST_SLC1_TOHOST_BIT2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOHOST_BIT3_INT_CLR` writer -
pub type HOST_SLC1_TOHOST_BIT3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOHOST_BIT4_INT_CLR` writer -
pub type HOST_SLC1_TOHOST_BIT4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOHOST_BIT5_INT_CLR` writer -
pub type HOST_SLC1_TOHOST_BIT5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOHOST_BIT6_INT_CLR` writer -
pub type HOST_SLC1_TOHOST_BIT6_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOHOST_BIT7_INT_CLR` writer -
pub type HOST_SLC1_TOHOST_BIT7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOKEN0_1TO0_INT_CLR` writer -
pub type HOST_SLC1_TOKEN0_1TO0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOKEN1_1TO0_INT_CLR` writer -
pub type HOST_SLC1_TOKEN1_1TO0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOKEN0_0TO1_INT_CLR` writer -
pub type HOST_SLC1_TOKEN0_0TO1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TOKEN1_0TO1_INT_CLR` writer -
pub type HOST_SLC1_TOKEN1_0TO1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1HOST_RX_SOF_INT_CLR` writer -
pub type HOST_SLC1HOST_RX_SOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1HOST_RX_EOF_INT_CLR` writer -
pub type HOST_SLC1HOST_RX_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1HOST_RX_START_INT_CLR` writer -
pub type HOST_SLC1HOST_RX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1HOST_TX_START_INT_CLR` writer -
pub type HOST_SLC1HOST_TX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_RX_UDF_INT_CLR` writer -
pub type HOST_SLC1_RX_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_TX_OVF_INT_CLR` writer -
pub type HOST_SLC1_TX_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_RX_PF_VALID_INT_CLR` writer -
pub type HOST_SLC1_RX_PF_VALID_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_EXT_BIT0_INT_CLR` writer -
pub type HOST_SLC1_EXT_BIT0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_EXT_BIT1_INT_CLR` writer -
pub type HOST_SLC1_EXT_BIT1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_EXT_BIT2_INT_CLR` writer -
pub type HOST_SLC1_EXT_BIT2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_EXT_BIT3_INT_CLR` writer -
pub type HOST_SLC1_EXT_BIT3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_WIFI_RX_NEW_PACKET_INT_CLR` writer -
pub type HOST_SLC1_WIFI_RX_NEW_PACKET_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_HOST_RD_RETRY_INT_CLR` writer -
pub type HOST_SLC1_HOST_RD_RETRY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_SLC1_BT_RX_NEW_PACKET_INT_CLR` writer -
pub type HOST_SLC1_BT_RX_NEW_PACKET_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC1HOST_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit0_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOHOST_BIT0_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOHOST_BIT0_INT_CLR_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit1_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOHOST_BIT1_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOHOST_BIT1_INT_CLR_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit2_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOHOST_BIT2_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOHOST_BIT2_INT_CLR_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit3_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOHOST_BIT3_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOHOST_BIT3_INT_CLR_W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit4_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOHOST_BIT4_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOHOST_BIT4_INT_CLR_W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit5_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOHOST_BIT5_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOHOST_BIT5_INT_CLR_W::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit6_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOHOST_BIT6_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOHOST_BIT6_INT_CLR_W::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tohost_bit7_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOHOST_BIT7_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOHOST_BIT7_INT_CLR_W::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_token0_1to0_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOKEN0_1TO0_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOKEN0_1TO0_INT_CLR_W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_token1_1to0_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOKEN1_1TO0_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOKEN1_1TO0_INT_CLR_W::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_token0_0to1_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOKEN0_0TO1_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOKEN0_0TO1_INT_CLR_W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_token1_0to1_int_clr(
        &mut self,
    ) -> HOST_SLC1_TOKEN1_0TO1_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TOKEN1_0TO1_INT_CLR_W::new(self, 11)
    }
    ///Bit 12
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_rx_sof_int_clr(
        &mut self,
    ) -> HOST_SLC1HOST_RX_SOF_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1HOST_RX_SOF_INT_CLR_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_rx_eof_int_clr(
        &mut self,
    ) -> HOST_SLC1HOST_RX_EOF_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1HOST_RX_EOF_INT_CLR_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_rx_start_int_clr(
        &mut self,
    ) -> HOST_SLC1HOST_RX_START_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1HOST_RX_START_INT_CLR_W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_tx_start_int_clr(
        &mut self,
    ) -> HOST_SLC1HOST_TX_START_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1HOST_TX_START_INT_CLR_W::new(self, 15)
    }
    ///Bit 16
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_rx_udf_int_clr(
        &mut self,
    ) -> HOST_SLC1_RX_UDF_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_RX_UDF_INT_CLR_W::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_tx_ovf_int_clr(
        &mut self,
    ) -> HOST_SLC1_TX_OVF_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_TX_OVF_INT_CLR_W::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_rx_pf_valid_int_clr(
        &mut self,
    ) -> HOST_SLC1_RX_PF_VALID_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_RX_PF_VALID_INT_CLR_W::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_ext_bit0_int_clr(
        &mut self,
    ) -> HOST_SLC1_EXT_BIT0_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_EXT_BIT0_INT_CLR_W::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_ext_bit1_int_clr(
        &mut self,
    ) -> HOST_SLC1_EXT_BIT1_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_EXT_BIT1_INT_CLR_W::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_ext_bit2_int_clr(
        &mut self,
    ) -> HOST_SLC1_EXT_BIT2_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_EXT_BIT2_INT_CLR_W::new(self, 21)
    }
    ///Bit 22
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_ext_bit3_int_clr(
        &mut self,
    ) -> HOST_SLC1_EXT_BIT3_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_EXT_BIT3_INT_CLR_W::new(self, 22)
    }
    ///Bit 23
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_wifi_rx_new_packet_int_clr(
        &mut self,
    ) -> HOST_SLC1_WIFI_RX_NEW_PACKET_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_WIFI_RX_NEW_PACKET_INT_CLR_W::new(self, 23)
    }
    ///Bit 24
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_host_rd_retry_int_clr(
        &mut self,
    ) -> HOST_SLC1_HOST_RD_RETRY_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_HOST_RD_RETRY_INT_CLR_W::new(self, 24)
    }
    ///Bit 25
    #[inline(always)]
    #[must_use]
    pub fn host_slc1_bt_rx_new_packet_int_clr(
        &mut self,
    ) -> HOST_SLC1_BT_RX_NEW_PACKET_INT_CLR_W<HOST_SLC1HOST_INT_CLR_SPEC> {
        HOST_SLC1_BT_RX_NEW_PACKET_INT_CLR_W::new(self, 25)
    }
}
/**

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLC1HOST_INT_CLR_SPEC;
impl crate::RegisterSpec for HOST_SLC1HOST_INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`host_slc1host_int_clr::W`](W) writer structure
impl crate::Writable for HOST_SLC1HOST_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HOST_SLC1HOST_INT_CLR to value 0
impl crate::Resettable for HOST_SLC1HOST_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
