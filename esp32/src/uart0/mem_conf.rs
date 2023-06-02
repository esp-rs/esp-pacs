#[doc = "Register `MEM_CONF` reader"]
pub struct R(crate::R<MEM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CONF` writer"]
pub struct W(crate::W<MEM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CONF_SPEC>;
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
impl From<crate::W<MEM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_PD` reader - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
pub type MEM_PD_R = crate::BitReader;
#[doc = "Field `MEM_PD` writer - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
pub type MEM_PD_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `RX_SIZE` reader - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
pub type RX_SIZE_R = crate::FieldReader;
#[doc = "Field `RX_SIZE` writer - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
pub type RX_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 4, O>;
#[doc = "Field `TX_SIZE` reader - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
pub type TX_SIZE_R = crate::FieldReader;
#[doc = "Field `TX_SIZE` writer - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
pub type TX_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 4, O>;
#[doc = "Field `RX_FLOW_THRHD_H3` reader - refer to the rx_flow_thrhd's describtion."]
pub type RX_FLOW_THRHD_H3_R = crate::FieldReader;
#[doc = "Field `RX_FLOW_THRHD_H3` writer - refer to the rx_flow_thrhd's describtion."]
pub type RX_FLOW_THRHD_H3_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 3, O>;
#[doc = "Field `RX_TOUT_THRHD_H3` reader - refer to the rx_tout_thrhd's describtion."]
pub type RX_TOUT_THRHD_H3_R = crate::FieldReader;
#[doc = "Field `RX_TOUT_THRHD_H3` writer - refer to the rx_tout_thrhd's describtion."]
pub type RX_TOUT_THRHD_H3_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 3, O>;
#[doc = "Field `XON_THRESHOLD_H2` reader - refer to the uart_xon_threshold's describtion."]
pub type XON_THRESHOLD_H2_R = crate::FieldReader;
#[doc = "Field `XON_THRESHOLD_H2` writer - refer to the uart_xon_threshold's describtion."]
pub type XON_THRESHOLD_H2_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 2, O>;
#[doc = "Field `XOFF_THRESHOLD_H2` reader - refer to the uart_xoff_threshold's describtion."]
pub type XOFF_THRESHOLD_H2_R = crate::FieldReader;
#[doc = "Field `XOFF_THRESHOLD_H2` writer - refer to the uart_xoff_threshold's describtion."]
pub type XOFF_THRESHOLD_H2_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 2, O>;
#[doc = "Field `RX_MEM_FULL_THRHD` reader - refer to the rxfifo_full_thrhd's describtion."]
pub type RX_MEM_FULL_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_MEM_FULL_THRHD` writer - refer to the rxfifo_full_thrhd's describtion."]
pub type RX_MEM_FULL_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 3, O>;
#[doc = "Field `TX_MEM_EMPTY_THRHD` reader - refer to txfifo_empty_thrhd 's describtion."]
pub type TX_MEM_EMPTY_THRHD_R = crate::FieldReader;
#[doc = "Field `TX_MEM_EMPTY_THRHD` writer - refer to txfifo_empty_thrhd 's describtion."]
pub type TX_MEM_EMPTY_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
    #[inline(always)]
    pub fn mem_pd(&self) -> MEM_PD_R {
        MEM_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:6 - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 15:17 - refer to the rx_flow_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_flow_thrhd_h3(&self) -> RX_FLOW_THRHD_H3_R {
        RX_FLOW_THRHD_H3_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - refer to the rx_tout_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_tout_thrhd_h3(&self) -> RX_TOUT_THRHD_H3_R {
        RX_TOUT_THRHD_H3_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - refer to the uart_xon_threshold's describtion."]
    #[inline(always)]
    pub fn xon_threshold_h2(&self) -> XON_THRESHOLD_H2_R {
        XON_THRESHOLD_H2_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - refer to the uart_xoff_threshold's describtion."]
    #[inline(always)]
    pub fn xoff_threshold_h2(&self) -> XOFF_THRESHOLD_H2_R {
        XOFF_THRESHOLD_H2_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27 - refer to the rxfifo_full_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_mem_full_thrhd(&self) -> RX_MEM_FULL_THRHD_R {
        RX_MEM_FULL_THRHD_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - refer to txfifo_empty_thrhd 's describtion."]
    #[inline(always)]
    pub fn tx_mem_empty_thrhd(&self) -> TX_MEM_EMPTY_THRHD_R {
        TX_MEM_EMPTY_THRHD_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF")
            .field("mem_pd", &format_args!("{}", self.mem_pd().bit()))
            .field("rx_size", &format_args!("{}", self.rx_size().bits()))
            .field("tx_size", &format_args!("{}", self.tx_size().bits()))
            .field(
                "rx_flow_thrhd_h3",
                &format_args!("{}", self.rx_flow_thrhd_h3().bits()),
            )
            .field(
                "rx_tout_thrhd_h3",
                &format_args!("{}", self.rx_tout_thrhd_h3().bits()),
            )
            .field(
                "xon_threshold_h2",
                &format_args!("{}", self.xon_threshold_h2().bits()),
            )
            .field(
                "xoff_threshold_h2",
                &format_args!("{}", self.xoff_threshold_h2().bits()),
            )
            .field(
                "rx_mem_full_thrhd",
                &format_args!("{}", self.rx_mem_full_thrhd().bits()),
            )
            .field(
                "tx_mem_empty_thrhd",
                &format_args!("{}", self.tx_mem_empty_thrhd().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem_pd(&mut self) -> MEM_PD_W<0> {
        MEM_PD_W::new(self)
    }
    #[doc = "Bits 3:6 - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
    #[inline(always)]
    #[must_use]
    pub fn rx_size(&mut self) -> RX_SIZE_W<3> {
        RX_SIZE_W::new(self)
    }
    #[doc = "Bits 7:10 - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
    #[inline(always)]
    #[must_use]
    pub fn tx_size(&mut self) -> TX_SIZE_W<7> {
        TX_SIZE_W::new(self)
    }
    #[doc = "Bits 15:17 - refer to the rx_flow_thrhd's describtion."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_thrhd_h3(&mut self) -> RX_FLOW_THRHD_H3_W<15> {
        RX_FLOW_THRHD_H3_W::new(self)
    }
    #[doc = "Bits 18:20 - refer to the rx_tout_thrhd's describtion."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_thrhd_h3(&mut self) -> RX_TOUT_THRHD_H3_W<18> {
        RX_TOUT_THRHD_H3_W::new(self)
    }
    #[doc = "Bits 21:22 - refer to the uart_xon_threshold's describtion."]
    #[inline(always)]
    #[must_use]
    pub fn xon_threshold_h2(&mut self) -> XON_THRESHOLD_H2_W<21> {
        XON_THRESHOLD_H2_W::new(self)
    }
    #[doc = "Bits 23:24 - refer to the uart_xoff_threshold's describtion."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_threshold_h2(&mut self) -> XOFF_THRESHOLD_H2_W<23> {
        XOFF_THRESHOLD_H2_W::new(self)
    }
    #[doc = "Bits 25:27 - refer to the rxfifo_full_thrhd's describtion."]
    #[inline(always)]
    #[must_use]
    pub fn rx_mem_full_thrhd(&mut self) -> RX_MEM_FULL_THRHD_W<25> {
        RX_MEM_FULL_THRHD_W::new(self)
    }
    #[doc = "Bits 28:30 - refer to txfifo_empty_thrhd 's describtion."]
    #[inline(always)]
    #[must_use]
    pub fn tx_mem_empty_thrhd(&mut self) -> TX_MEM_EMPTY_THRHD_W<28> {
        TX_MEM_EMPTY_THRHD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_conf](index.html) module"]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_conf::R](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_conf::W](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x88"]
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x88;
}
