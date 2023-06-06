#[doc = "Register `ETM_CONF` reader"]
pub struct R(crate::R<ETM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETM_CONF` writer"]
pub struct W(crate::W<ETM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETM_CONF_SPEC>;
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
impl From<crate::W<ETM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETM_TX_SEND_WORD_NUM` reader - I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num\\[9:0\\], i2s will trigger an etm event."]
pub type ETM_TX_SEND_WORD_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `ETM_TX_SEND_WORD_NUM` writer - I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num\\[9:0\\], i2s will trigger an etm event."]
pub type ETM_TX_SEND_WORD_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, ETM_CONF_SPEC, 10, O, u16>;
#[doc = "Field `ETM_RX_RECEIVE_WORD_NUM` reader - I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num\\[9:0\\], i2s will trigger an etm event."]
pub type ETM_RX_RECEIVE_WORD_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `ETM_RX_RECEIVE_WORD_NUM` writer - I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num\\[9:0\\], i2s will trigger an etm event."]
pub type ETM_RX_RECEIVE_WORD_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, ETM_CONF_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num\\[9:0\\], i2s will trigger an etm event."]
    #[inline(always)]
    pub fn etm_tx_send_word_num(&self) -> ETM_TX_SEND_WORD_NUM_R {
        ETM_TX_SEND_WORD_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num\\[9:0\\], i2s will trigger an etm event."]
    #[inline(always)]
    pub fn etm_rx_receive_word_num(&self) -> ETM_RX_RECEIVE_WORD_NUM_R {
        ETM_RX_RECEIVE_WORD_NUM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CONF")
            .field(
                "etm_tx_send_word_num",
                &format_args!("{}", self.etm_tx_send_word_num().bits()),
            )
            .field(
                "etm_rx_receive_word_num",
                &format_args!("{}", self.etm_rx_receive_word_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ETM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num\\[9:0\\], i2s will trigger an etm event."]
    #[inline(always)]
    #[must_use]
    pub fn etm_tx_send_word_num(&mut self) -> ETM_TX_SEND_WORD_NUM_W<0> {
        ETM_TX_SEND_WORD_NUM_W::new(self)
    }
    #[doc = "Bits 10:19 - I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num\\[9:0\\], i2s will trigger an etm event."]
    #[inline(always)]
    #[must_use]
    pub fn etm_rx_receive_word_num(&mut self) -> ETM_RX_RECEIVE_WORD_NUM_W<10> {
        ETM_RX_RECEIVE_WORD_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S ETM configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etm_conf](index.html) module"]
pub struct ETM_CONF_SPEC;
impl crate::RegisterSpec for ETM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etm_conf::R](R) reader structure"]
impl crate::Readable for ETM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etm_conf::W](W) writer structure"]
impl crate::Writable for ETM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETM_CONF to value 0x0001_0040"]
impl crate::Resettable for ETM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0040;
}
