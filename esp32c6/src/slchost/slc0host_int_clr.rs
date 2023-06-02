#[doc = "Register `SLC0HOST_INT_CLR` writer"]
pub struct W(crate::W<SLC0HOST_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC0HOST_INT_CLR_SPEC>;
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
impl From<crate::W<SLC0HOST_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC0HOST_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TOHOST_BIT0_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOHOST_BIT0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOHOST_BIT1_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOHOST_BIT1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOHOST_BIT2_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOHOST_BIT2_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOHOST_BIT3_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOHOST_BIT3_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOHOST_BIT4_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOHOST_BIT4_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOHOST_BIT5_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOHOST_BIT5_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOHOST_BIT6_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOHOST_BIT6_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOHOST_BIT7_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOHOST_BIT7_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOKEN0_1TO0_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOKEN0_1TO0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOKEN1_1TO0_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOKEN1_1TO0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOKEN0_0TO1_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOKEN0_0TO1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TOKEN1_0TO1_INT_CLR` writer - *******Description***********"]
pub type SLC0_TOKEN1_0TO1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0HOST_RX_SOF_INT_CLR` writer - *******Description***********"]
pub type SLC0HOST_RX_SOF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0HOST_RX_EOF_INT_CLR` writer - *******Description***********"]
pub type SLC0HOST_RX_EOF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0HOST_RX_START_INT_CLR` writer - *******Description***********"]
pub type SLC0HOST_RX_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0HOST_TX_START_INT_CLR` writer - *******Description***********"]
pub type SLC0HOST_TX_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_RX_UDF_INT_CLR` writer - *******Description***********"]
pub type SLC0_RX_UDF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_TX_OVF_INT_CLR` writer - *******Description***********"]
pub type SLC0_TX_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_RX_PF_VALID_INT_CLR` writer - *******Description***********"]
pub type SLC0_RX_PF_VALID_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_EXT_BIT0_INT_CLR` writer - *******Description***********"]
pub type SLC0_EXT_BIT0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_EXT_BIT1_INT_CLR` writer - *******Description***********"]
pub type SLC0_EXT_BIT1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_EXT_BIT2_INT_CLR` writer - *******Description***********"]
pub type SLC0_EXT_BIT2_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_EXT_BIT3_INT_CLR` writer - *******Description***********"]
pub type SLC0_EXT_BIT3_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_RX_NEW_PACKET_INT_CLR` writer - *******Description***********"]
pub type SLC0_RX_NEW_PACKET_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `SLC0_HOST_RD_RETRY_INT_CLR` writer - *******Description***********"]
pub type SLC0_HOST_RD_RETRY_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[doc = "Field `GPIO_SDIO_INT_CLR` writer - *******Description***********"]
pub type GPIO_SDIO_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLC0HOST_INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0HOST_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_bit0_int_clr(&mut self) -> SLC0_TOHOST_BIT0_INT_CLR_W<0> {
        SLC0_TOHOST_BIT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_bit1_int_clr(&mut self) -> SLC0_TOHOST_BIT1_INT_CLR_W<1> {
        SLC0_TOHOST_BIT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_bit2_int_clr(&mut self) -> SLC0_TOHOST_BIT2_INT_CLR_W<2> {
        SLC0_TOHOST_BIT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_bit3_int_clr(&mut self) -> SLC0_TOHOST_BIT3_INT_CLR_W<3> {
        SLC0_TOHOST_BIT3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_bit4_int_clr(&mut self) -> SLC0_TOHOST_BIT4_INT_CLR_W<4> {
        SLC0_TOHOST_BIT4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_bit5_int_clr(&mut self) -> SLC0_TOHOST_BIT5_INT_CLR_W<5> {
        SLC0_TOHOST_BIT5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_bit6_int_clr(&mut self) -> SLC0_TOHOST_BIT6_INT_CLR_W<6> {
        SLC0_TOHOST_BIT6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_bit7_int_clr(&mut self) -> SLC0_TOHOST_BIT7_INT_CLR_W<7> {
        SLC0_TOHOST_BIT7_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token0_1to0_int_clr(&mut self) -> SLC0_TOKEN0_1TO0_INT_CLR_W<8> {
        SLC0_TOKEN0_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token1_1to0_int_clr(&mut self) -> SLC0_TOKEN1_1TO0_INT_CLR_W<9> {
        SLC0_TOKEN1_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token0_0to1_int_clr(&mut self) -> SLC0_TOKEN0_0TO1_INT_CLR_W<10> {
        SLC0_TOKEN0_0TO1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token1_0to1_int_clr(&mut self) -> SLC0_TOKEN1_0TO1_INT_CLR_W<11> {
        SLC0_TOKEN1_0TO1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_rx_sof_int_clr(&mut self) -> SLC0HOST_RX_SOF_INT_CLR_W<12> {
        SLC0HOST_RX_SOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_rx_eof_int_clr(&mut self) -> SLC0HOST_RX_EOF_INT_CLR_W<13> {
        SLC0HOST_RX_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_rx_start_int_clr(&mut self) -> SLC0HOST_RX_START_INT_CLR_W<14> {
        SLC0HOST_RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_tx_start_int_clr(&mut self) -> SLC0HOST_TX_START_INT_CLR_W<15> {
        SLC0HOST_TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_udf_int_clr(&mut self) -> SLC0_RX_UDF_INT_CLR_W<16> {
        SLC0_RX_UDF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_ovf_int_clr(&mut self) -> SLC0_TX_OVF_INT_CLR_W<17> {
        SLC0_TX_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_pf_valid_int_clr(&mut self) -> SLC0_RX_PF_VALID_INT_CLR_W<18> {
        SLC0_RX_PF_VALID_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_ext_bit0_int_clr(&mut self) -> SLC0_EXT_BIT0_INT_CLR_W<19> {
        SLC0_EXT_BIT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_ext_bit1_int_clr(&mut self) -> SLC0_EXT_BIT1_INT_CLR_W<20> {
        SLC0_EXT_BIT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_ext_bit2_int_clr(&mut self) -> SLC0_EXT_BIT2_INT_CLR_W<21> {
        SLC0_EXT_BIT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_ext_bit3_int_clr(&mut self) -> SLC0_EXT_BIT3_INT_CLR_W<22> {
        SLC0_EXT_BIT3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_new_packet_int_clr(&mut self) -> SLC0_RX_NEW_PACKET_INT_CLR_W<23> {
        SLC0_RX_NEW_PACKET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_host_rd_retry_int_clr(&mut self) -> SLC0_HOST_RD_RETRY_INT_CLR_W<24> {
        SLC0_HOST_RD_RETRY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_sdio_int_clr(&mut self) -> GPIO_SDIO_INT_CLR_W<25> {
        GPIO_SDIO_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc0host_int_clr](index.html) module"]
pub struct SLC0HOST_INT_CLR_SPEC;
impl crate::RegisterSpec for SLC0HOST_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [slc0host_int_clr::W](W) writer structure"]
impl crate::Writable for SLC0HOST_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC0HOST_INT_CLR to value 0"]
impl crate::Resettable for SLC0HOST_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
