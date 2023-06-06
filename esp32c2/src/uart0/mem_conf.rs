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
#[doc = "Field `RX_SIZE` reader - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
pub type RX_SIZE_R = crate::FieldReader;
#[doc = "Field `RX_SIZE` writer - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
pub type RX_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 3, O>;
#[doc = "Field `TX_SIZE` reader - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
pub type TX_SIZE_R = crate::FieldReader;
#[doc = "Field `TX_SIZE` writer - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
pub type TX_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 3, O>;
#[doc = "Field `RX_FLOW_THRHD` reader - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RX_FLOW_THRHD_R = crate::FieldReader<u16>;
#[doc = "Field `RX_FLOW_THRHD` writer - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RX_FLOW_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 9, O, u16>;
#[doc = "Field `RX_TOUT_THRHD` reader - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub type RX_TOUT_THRHD_R = crate::FieldReader<u16>;
#[doc = "Field `RX_TOUT_THRHD` writer - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub type RX_TOUT_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 10, O, u16>;
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to force power down UART memory."]
pub type MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to force power down UART memory."]
pub type MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `MEM_FORCE_PU` reader - Set this bit to force power up UART memory."]
pub type MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PU` writer - Set this bit to force power up UART memory."]
pub type MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 1:3 - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:15 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:25 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Set this bit to force power down UART memory."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to force power up UART memory."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF")
            .field("rx_size", &format_args!("{}", self.rx_size().bits()))
            .field("tx_size", &format_args!("{}", self.tx_size().bits()))
            .field(
                "rx_flow_thrhd",
                &format_args!("{}", self.rx_flow_thrhd().bits()),
            )
            .field(
                "rx_tout_thrhd",
                &format_args!("{}", self.rx_tout_thrhd().bits()),
            )
            .field(
                "mem_force_pd",
                &format_args!("{}", self.mem_force_pd().bit()),
            )
            .field(
                "mem_force_pu",
                &format_args!("{}", self.mem_force_pu().bit()),
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
    #[doc = "Bits 1:3 - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn rx_size(&mut self) -> RX_SIZE_W<1> {
        RX_SIZE_W::new(self)
    }
    #[doc = "Bits 4:6 - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn tx_size(&mut self) -> TX_SIZE_W<4> {
        TX_SIZE_W::new(self)
    }
    #[doc = "Bits 7:15 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<7> {
        RX_FLOW_THRHD_W::new(self)
    }
    #[doc = "Bits 16:25 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W<16> {
        RX_TOUT_THRHD_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to force power down UART memory."]
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<26> {
        MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to force power up UART memory."]
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<27> {
        MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART threshold and allocation configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_conf](index.html) module"]
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
#[doc = "`reset()` method sets MEM_CONF to value 0x000a_0012"]
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_0012;
}
