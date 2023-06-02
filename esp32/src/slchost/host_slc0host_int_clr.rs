#[doc = "Register `HOST_SLC0HOST_INT_CLR` writer"]
pub struct W(crate::W<HOST_SLC0HOST_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLC0HOST_INT_CLR_SPEC>;
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
impl From<crate::W<HOST_SLC0HOST_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLC0HOST_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT0_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT1_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT2_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT2_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT3_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT3_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT4_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT4_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT5_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT5_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT6_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT6_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT7_INT_CLR` writer - "]
pub type HOST_SLC0_TOHOST_BIT7_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOKEN0_1TO0_INT_CLR` writer - "]
pub type HOST_SLC0_TOKEN0_1TO0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOKEN1_1TO0_INT_CLR` writer - "]
pub type HOST_SLC0_TOKEN1_1TO0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOKEN0_0TO1_INT_CLR` writer - "]
pub type HOST_SLC0_TOKEN0_0TO1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TOKEN1_0TO1_INT_CLR` writer - "]
pub type HOST_SLC0_TOKEN1_0TO1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0HOST_RX_SOF_INT_CLR` writer - "]
pub type HOST_SLC0HOST_RX_SOF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0HOST_RX_EOF_INT_CLR` writer - "]
pub type HOST_SLC0HOST_RX_EOF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0HOST_RX_START_INT_CLR` writer - "]
pub type HOST_SLC0HOST_RX_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0HOST_TX_START_INT_CLR` writer - "]
pub type HOST_SLC0HOST_TX_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_RX_UDF_INT_CLR` writer - "]
pub type HOST_SLC0_RX_UDF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_TX_OVF_INT_CLR` writer - "]
pub type HOST_SLC0_TX_OVF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_RX_PF_VALID_INT_CLR` writer - "]
pub type HOST_SLC0_RX_PF_VALID_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_EXT_BIT0_INT_CLR` writer - "]
pub type HOST_SLC0_EXT_BIT0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_EXT_BIT1_INT_CLR` writer - "]
pub type HOST_SLC0_EXT_BIT1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_EXT_BIT2_INT_CLR` writer - "]
pub type HOST_SLC0_EXT_BIT2_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_EXT_BIT3_INT_CLR` writer - "]
pub type HOST_SLC0_EXT_BIT3_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_RX_NEW_PACKET_INT_CLR` writer - "]
pub type HOST_SLC0_RX_NEW_PACKET_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_SLC0_HOST_RD_RETRY_INT_CLR` writer - "]
pub type HOST_SLC0_HOST_RD_RETRY_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `HOST_GPIO_SDIO_INT_CLR` writer - "]
pub type HOST_GPIO_SDIO_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, HOST_SLC0HOST_INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC0HOST_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tohost_bit0_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT0_INT_CLR_W<0> {
        HOST_SLC0_TOHOST_BIT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tohost_bit1_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT1_INT_CLR_W<1> {
        HOST_SLC0_TOHOST_BIT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tohost_bit2_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT2_INT_CLR_W<2> {
        HOST_SLC0_TOHOST_BIT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tohost_bit3_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT3_INT_CLR_W<3> {
        HOST_SLC0_TOHOST_BIT3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tohost_bit4_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT4_INT_CLR_W<4> {
        HOST_SLC0_TOHOST_BIT4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tohost_bit5_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT5_INT_CLR_W<5> {
        HOST_SLC0_TOHOST_BIT5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tohost_bit6_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT6_INT_CLR_W<6> {
        HOST_SLC0_TOHOST_BIT6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tohost_bit7_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT7_INT_CLR_W<7> {
        HOST_SLC0_TOHOST_BIT7_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_token0_1to0_int_clr(&mut self) -> HOST_SLC0_TOKEN0_1TO0_INT_CLR_W<8> {
        HOST_SLC0_TOKEN0_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_token1_1to0_int_clr(&mut self) -> HOST_SLC0_TOKEN1_1TO0_INT_CLR_W<9> {
        HOST_SLC0_TOKEN1_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_token0_0to1_int_clr(&mut self) -> HOST_SLC0_TOKEN0_0TO1_INT_CLR_W<10> {
        HOST_SLC0_TOKEN0_0TO1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_token1_0to1_int_clr(&mut self) -> HOST_SLC0_TOKEN1_0TO1_INT_CLR_W<11> {
        HOST_SLC0_TOKEN1_0TO1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_rx_sof_int_clr(&mut self) -> HOST_SLC0HOST_RX_SOF_INT_CLR_W<12> {
        HOST_SLC0HOST_RX_SOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_rx_eof_int_clr(&mut self) -> HOST_SLC0HOST_RX_EOF_INT_CLR_W<13> {
        HOST_SLC0HOST_RX_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_rx_start_int_clr(&mut self) -> HOST_SLC0HOST_RX_START_INT_CLR_W<14> {
        HOST_SLC0HOST_RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_tx_start_int_clr(&mut self) -> HOST_SLC0HOST_TX_START_INT_CLR_W<15> {
        HOST_SLC0HOST_TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_rx_udf_int_clr(&mut self) -> HOST_SLC0_RX_UDF_INT_CLR_W<16> {
        HOST_SLC0_RX_UDF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_tx_ovf_int_clr(&mut self) -> HOST_SLC0_TX_OVF_INT_CLR_W<17> {
        HOST_SLC0_TX_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_rx_pf_valid_int_clr(&mut self) -> HOST_SLC0_RX_PF_VALID_INT_CLR_W<18> {
        HOST_SLC0_RX_PF_VALID_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_ext_bit0_int_clr(&mut self) -> HOST_SLC0_EXT_BIT0_INT_CLR_W<19> {
        HOST_SLC0_EXT_BIT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_ext_bit1_int_clr(&mut self) -> HOST_SLC0_EXT_BIT1_INT_CLR_W<20> {
        HOST_SLC0_EXT_BIT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_ext_bit2_int_clr(&mut self) -> HOST_SLC0_EXT_BIT2_INT_CLR_W<21> {
        HOST_SLC0_EXT_BIT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_ext_bit3_int_clr(&mut self) -> HOST_SLC0_EXT_BIT3_INT_CLR_W<22> {
        HOST_SLC0_EXT_BIT3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_rx_new_packet_int_clr(&mut self) -> HOST_SLC0_RX_NEW_PACKET_INT_CLR_W<23> {
        HOST_SLC0_RX_NEW_PACKET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0_host_rd_retry_int_clr(&mut self) -> HOST_SLC0_HOST_RD_RETRY_INT_CLR_W<24> {
        HOST_SLC0_HOST_RD_RETRY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn host_gpio_sdio_int_clr(&mut self) -> HOST_GPIO_SDIO_INT_CLR_W<25> {
        HOST_GPIO_SDIO_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0host_int_clr](index.html) module"]
pub struct HOST_SLC0HOST_INT_CLR_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_clr::W](W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_INT_CLR to value 0"]
impl crate::Resettable for HOST_SLC0HOST_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
