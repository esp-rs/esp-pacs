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
pub type RX_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_SIZE` writer - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
pub type RX_SIZE_W<'a> = crate::FieldWriter<'a, u32, MEM_CONF_SPEC, u8, u8, 3, 1>;
#[doc = "Field `TX_SIZE` reader - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
pub type TX_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_SIZE` writer - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
pub type TX_SIZE_W<'a> = crate::FieldWriter<'a, u32, MEM_CONF_SPEC, u8, u8, 3, 4>;
#[doc = "Field `RX_FLOW_THRHD` reader - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RX_FLOW_THRHD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_FLOW_THRHD` writer - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RX_FLOW_THRHD_W<'a> = crate::FieldWriter<'a, u32, MEM_CONF_SPEC, u16, u16, 9, 7>;
#[doc = "Field `RX_TOUT_THRHD` reader - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub type RX_TOUT_THRHD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_TOUT_THRHD` writer - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub type RX_TOUT_THRHD_W<'a> = crate::FieldWriter<'a, u32, MEM_CONF_SPEC, u16, u16, 10, 16>;
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to force power down UART memory."]
pub type MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to force power down UART memory."]
pub type MEM_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, MEM_CONF_SPEC, bool, 26>;
#[doc = "Field `MEM_FORCE_PU` reader - Set this bit to force power up UART memory."]
pub type MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `MEM_FORCE_PU` writer - Set this bit to force power up UART memory."]
pub type MEM_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, MEM_CONF_SPEC, bool, 27>;
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
impl W {
    #[doc = "Bits 1:3 - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    pub fn rx_size(&mut self) -> RX_SIZE_W {
        RX_SIZE_W::new(self)
    }
    #[doc = "Bits 4:6 - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    pub fn tx_size(&mut self) -> TX_SIZE_W {
        TX_SIZE_W::new(self)
    }
    #[doc = "Bits 7:15 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W {
        RX_FLOW_THRHD_W::new(self)
    }
    #[doc = "Bits 16:25 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W {
        RX_TOUT_THRHD_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to force power down UART memory."]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W {
        MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to force power up UART memory."]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W {
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
}
#[doc = "`reset()` method sets MEM_CONF to value 0x000a_0012"]
impl crate::Resettable for MEM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_0012
    }
}
