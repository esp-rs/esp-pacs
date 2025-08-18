#[doc = "Register `HOST_SLC0HOST_INT_CLR` writer"]
pub type W = crate::W<HOST_SLC0HOST_INT_CLR_SPEC>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT0_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT1_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT2_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT3_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT4_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT5_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT6_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT6_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT7_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOKEN0_1TO0_INT_CLR` writer - "]
pub type HOST_SLC0_TOKEN0_1TO0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOKEN1_1TO0_INT_CLR` writer - "]
pub type HOST_SLC0_TOKEN1_1TO0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOKEN0_0TO1_INT_CLR` writer - "]
pub type HOST_SLC0_TOKEN0_0TO1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TOKEN1_0TO1_INT_CLR` writer - "]
pub type HOST_SLC0_TOKEN1_0TO1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0HOST_RX_SOF_INT_CLR` writer - "]
pub type HOST_SLC0HOST_RX_SOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0HOST_RX_EOF_INT_CLR` writer - "]
pub type HOST_SLC0HOST_RX_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0HOST_RX_START_INT_CLR` writer - "]
pub type HOST_SLC0HOST_RX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0HOST_TX_START_INT_CLR` writer - "]
pub type HOST_SLC0HOST_TX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_RX_UDF_INT_CLR` writer - "]
pub type HOST_SLC0_RX_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_TX_OVF_INT_CLR` writer - "]
pub type HOST_SLC0_TX_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_RX_PF_VALID_INT_CLR` writer - "]
pub type HOST_SLC0_RX_PF_VALID_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_EXT_BIT0_INT_CLR` writer - "]
pub type HOST_SLC0_EXT_BIT0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_EXT_BIT1_INT_CLR` writer - "]
pub type HOST_SLC0_EXT_BIT1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_EXT_BIT2_INT_CLR` writer - "]
pub type HOST_SLC0_EXT_BIT2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_EXT_BIT3_INT_CLR` writer - "]
pub type HOST_SLC0_EXT_BIT3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_RX_NEW_PACKET_INT_CLR` writer - "]
pub type HOST_SLC0_RX_NEW_PACKET_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC0_HOST_RD_RETRY_INT_CLR` writer - "]
pub type HOST_SLC0_HOST_RD_RETRY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_GPIO_SDIO_INT_CLR` writer - "]
pub type HOST_GPIO_SDIO_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC0HOST_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit0_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOHOST_BIT0_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOHOST_BIT0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit1_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOHOST_BIT1_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOHOST_BIT1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit2_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOHOST_BIT2_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOHOST_BIT2_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit3_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOHOST_BIT3_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOHOST_BIT3_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit4_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOHOST_BIT4_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOHOST_BIT4_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit5_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOHOST_BIT5_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOHOST_BIT5_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit6_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOHOST_BIT6_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOHOST_BIT6_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit7_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOHOST_BIT7_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOHOST_BIT7_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0_token0_1to0_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOKEN0_1TO0_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOKEN0_1TO0_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn host_slc0_token1_1to0_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOKEN1_1TO0_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOKEN1_1TO0_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn host_slc0_token0_0to1_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOKEN0_0TO1_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOKEN0_0TO1_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn host_slc0_token1_0to1_int_clr(
        &mut self,
    ) -> HOST_SLC0_TOKEN1_0TO1_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TOKEN1_0TO1_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0host_rx_sof_int_clr(
        &mut self,
    ) -> HOST_SLC0HOST_RX_SOF_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0HOST_RX_SOF_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn host_slc0host_rx_eof_int_clr(
        &mut self,
    ) -> HOST_SLC0HOST_RX_EOF_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0HOST_RX_EOF_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn host_slc0host_rx_start_int_clr(
        &mut self,
    ) -> HOST_SLC0HOST_RX_START_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0HOST_RX_START_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn host_slc0host_tx_start_int_clr(
        &mut self,
    ) -> HOST_SLC0HOST_TX_START_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0HOST_TX_START_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn host_slc0_rx_udf_int_clr(
        &mut self,
    ) -> HOST_SLC0_RX_UDF_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_RX_UDF_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn host_slc0_tx_ovf_int_clr(
        &mut self,
    ) -> HOST_SLC0_TX_OVF_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_TX_OVF_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid_int_clr(
        &mut self,
    ) -> HOST_SLC0_RX_PF_VALID_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_RX_PF_VALID_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_slc0_ext_bit0_int_clr(
        &mut self,
    ) -> HOST_SLC0_EXT_BIT0_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_EXT_BIT0_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn host_slc0_ext_bit1_int_clr(
        &mut self,
    ) -> HOST_SLC0_EXT_BIT1_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_EXT_BIT1_INT_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn host_slc0_ext_bit2_int_clr(
        &mut self,
    ) -> HOST_SLC0_EXT_BIT2_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_EXT_BIT2_INT_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn host_slc0_ext_bit3_int_clr(
        &mut self,
    ) -> HOST_SLC0_EXT_BIT3_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_EXT_BIT3_INT_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn host_slc0_rx_new_packet_int_clr(
        &mut self,
    ) -> HOST_SLC0_RX_NEW_PACKET_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_RX_NEW_PACKET_INT_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn host_slc0_host_rd_retry_int_clr(
        &mut self,
    ) -> HOST_SLC0_HOST_RD_RETRY_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_SLC0_HOST_RD_RETRY_INT_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_gpio_sdio_int_clr(
        &mut self,
    ) -> HOST_GPIO_SDIO_INT_CLR_W<'_, HOST_SLC0HOST_INT_CLR_SPEC> {
        HOST_GPIO_SDIO_INT_CLR_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slc0host_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLC0HOST_INT_CLR_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`host_slc0host_int_clr::W`](W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_INT_CLR to value 0"]
impl crate::Resettable for HOST_SLC0HOST_INT_CLR_SPEC {}
