#[doc = "Register `MEM_CONF` reader"]
pub type R = crate::R<MEM_CONF_SPEC>;
#[doc = "Register `MEM_CONF` writer"]
pub type W = crate::W<MEM_CONF_SPEC>;
#[doc = "Field `MEM_PD` reader - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
pub type MEM_PD_R = crate::BitReader;
#[doc = "Field `MEM_PD` writer - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
pub type MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SIZE` reader - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
pub type RX_SIZE_R = crate::FieldReader;
#[doc = "Field `RX_SIZE` writer - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
pub type RX_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_SIZE` reader - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
pub type TX_SIZE_R = crate::FieldReader;
#[doc = "Field `TX_SIZE` writer - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
pub type TX_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_FLOW_THRHD_H3` reader - Refer to the rx_flow_thrhd's description."]
pub type RX_FLOW_THRHD_H3_R = crate::FieldReader;
#[doc = "Field `RX_FLOW_THRHD_H3` writer - Refer to the rx_flow_thrhd's description."]
pub type RX_FLOW_THRHD_H3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_TOUT_THRHD_H3` reader - Refer to the rx_tout_thrhd's description."]
pub type RX_TOUT_THRHD_H3_R = crate::FieldReader;
#[doc = "Field `RX_TOUT_THRHD_H3` writer - Refer to the rx_tout_thrhd's description."]
pub type RX_TOUT_THRHD_H3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XON_THRESHOLD_H2` reader - Refer to the uart_xon_threshold's description."]
pub type XON_THRESHOLD_H2_R = crate::FieldReader;
#[doc = "Field `XON_THRESHOLD_H2` writer - Refer to the uart_xon_threshold's description."]
pub type XON_THRESHOLD_H2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XOFF_THRESHOLD_H2` reader - Refer to the uart_xoff_threshold's description."]
pub type XOFF_THRESHOLD_H2_R = crate::FieldReader;
#[doc = "Field `XOFF_THRESHOLD_H2` writer - Refer to the uart_xoff_threshold's description."]
pub type XOFF_THRESHOLD_H2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_MEM_FULL_THRHD` reader - Refer to the rxfifo_full_thrhd's description."]
pub type RX_MEM_FULL_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_MEM_FULL_THRHD` writer - Refer to the rxfifo_full_thrhd's description."]
pub type RX_MEM_FULL_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TX_MEM_EMPTY_THRHD` reader - Refer to txfifo_empty_thrhd's description."]
pub type TX_MEM_EMPTY_THRHD_R = crate::FieldReader;
#[doc = "Field `TX_MEM_EMPTY_THRHD` writer - Refer to txfifo_empty_thrhd's description."]
pub type TX_MEM_EMPTY_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    #[doc = "Bits 15:17 - Refer to the rx_flow_thrhd's description."]
    #[inline(always)]
    pub fn rx_flow_thrhd_h3(&self) -> RX_FLOW_THRHD_H3_R {
        RX_FLOW_THRHD_H3_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Refer to the rx_tout_thrhd's description."]
    #[inline(always)]
    pub fn rx_tout_thrhd_h3(&self) -> RX_TOUT_THRHD_H3_R {
        RX_TOUT_THRHD_H3_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Refer to the uart_xon_threshold's description."]
    #[inline(always)]
    pub fn xon_threshold_h2(&self) -> XON_THRESHOLD_H2_R {
        XON_THRESHOLD_H2_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Refer to the uart_xoff_threshold's description."]
    #[inline(always)]
    pub fn xoff_threshold_h2(&self) -> XOFF_THRESHOLD_H2_R {
        XOFF_THRESHOLD_H2_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27 - Refer to the rxfifo_full_thrhd's description."]
    #[inline(always)]
    pub fn rx_mem_full_thrhd(&self) -> RX_MEM_FULL_THRHD_R {
        RX_MEM_FULL_THRHD_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Refer to txfifo_empty_thrhd's description."]
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem_pd(&mut self) -> MEM_PD_W<MEM_CONF_SPEC> {
        MEM_PD_W::new(self, 0)
    }
    #[doc = "Bits 3:6 - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
    #[inline(always)]
    #[must_use]
    pub fn rx_size(&mut self) -> RX_SIZE_W<MEM_CONF_SPEC> {
        RX_SIZE_W::new(self, 3)
    }
    #[doc = "Bits 7:10 - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
    #[inline(always)]
    #[must_use]
    pub fn tx_size(&mut self) -> TX_SIZE_W<MEM_CONF_SPEC> {
        TX_SIZE_W::new(self, 7)
    }
    #[doc = "Bits 15:17 - Refer to the rx_flow_thrhd's description."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_thrhd_h3(&mut self) -> RX_FLOW_THRHD_H3_W<MEM_CONF_SPEC> {
        RX_FLOW_THRHD_H3_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Refer to the rx_tout_thrhd's description."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_thrhd_h3(&mut self) -> RX_TOUT_THRHD_H3_W<MEM_CONF_SPEC> {
        RX_TOUT_THRHD_H3_W::new(self, 18)
    }
    #[doc = "Bits 21:22 - Refer to the uart_xon_threshold's description."]
    #[inline(always)]
    #[must_use]
    pub fn xon_threshold_h2(&mut self) -> XON_THRESHOLD_H2_W<MEM_CONF_SPEC> {
        XON_THRESHOLD_H2_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Refer to the uart_xoff_threshold's description."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_threshold_h2(&mut self) -> XOFF_THRESHOLD_H2_W<MEM_CONF_SPEC> {
        XOFF_THRESHOLD_H2_W::new(self, 23)
    }
    #[doc = "Bits 25:27 - Refer to the rxfifo_full_thrhd's description."]
    #[inline(always)]
    #[must_use]
    pub fn rx_mem_full_thrhd(&mut self) -> RX_MEM_FULL_THRHD_W<MEM_CONF_SPEC> {
        RX_MEM_FULL_THRHD_W::new(self, 25)
    }
    #[doc = "Bits 28:30 - Refer to txfifo_empty_thrhd's description."]
    #[inline(always)]
    #[must_use]
    pub fn tx_mem_empty_thrhd(&mut self) -> TX_MEM_EMPTY_THRHD_W<MEM_CONF_SPEC> {
        TX_MEM_EMPTY_THRHD_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_conf::R`](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_conf::W`](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x88"]
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x88;
}
