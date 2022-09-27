#[doc = "Register `I2S_RXEOF_NUM` reader"]
pub struct R(crate::R<I2S_RXEOF_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_RXEOF_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_RXEOF_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_RXEOF_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_RXEOF_NUM` writer"]
pub struct W(crate::W<I2S_RXEOF_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_RXEOF_NUM_SPEC>;
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
impl From<crate::W<I2S_RXEOF_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_RXEOF_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_RX_EOF_NUM` reader - The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
pub type I2S_RX_EOF_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `I2S_RX_EOF_NUM` writer - The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
pub type I2S_RX_EOF_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_RXEOF_NUM_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
    #[inline(always)]
    pub fn i2s_rx_eof_num(&self) -> I2S_RX_EOF_NUM_R {
        I2S_RX_EOF_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
    #[inline(always)]
    pub fn i2s_rx_eof_num(&mut self) -> I2S_RX_EOF_NUM_W<0> {
        I2S_RX_EOF_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S RX data number control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rxeof_num](index.html) module"]
pub struct I2S_RXEOF_NUM_SPEC;
impl crate::RegisterSpec for I2S_RXEOF_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_rxeof_num::R](R) reader structure"]
impl crate::Readable for I2S_RXEOF_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_rxeof_num::W](W) writer structure"]
impl crate::Writable for I2S_RXEOF_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_RXEOF_NUM to value 0x40"]
impl crate::Resettable for I2S_RXEOF_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
