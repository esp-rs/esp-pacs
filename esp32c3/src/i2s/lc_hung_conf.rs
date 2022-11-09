#[doc = "Register `LC_HUNG_CONF` reader"]
pub struct R(crate::R<LC_HUNG_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_HUNG_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_HUNG_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_HUNG_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LC_HUNG_CONF` writer"]
pub struct W(crate::W<LC_HUNG_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LC_HUNG_CONF_SPEC>;
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
impl From<crate::W<LC_HUNG_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LC_HUNG_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LC_FIFO_TIMEOUT` reader - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
pub type LC_FIFO_TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LC_FIFO_TIMEOUT` writer - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
pub type LC_FIFO_TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LC_HUNG_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `LC_FIFO_TIMEOUT_SHIFT` reader - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
pub type LC_FIFO_TIMEOUT_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LC_FIFO_TIMEOUT_SHIFT` writer - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
pub type LC_FIFO_TIMEOUT_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LC_HUNG_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `LC_FIFO_TIMEOUT_ENA` reader - The enable bit for FIFO timeout"]
pub type LC_FIFO_TIMEOUT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `LC_FIFO_TIMEOUT_ENA` writer - The enable bit for FIFO timeout"]
pub type LC_FIFO_TIMEOUT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LC_HUNG_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
    #[inline(always)]
    pub fn lc_fifo_timeout(&self) -> LC_FIFO_TIMEOUT_R {
        LC_FIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
    #[inline(always)]
    pub fn lc_fifo_timeout_shift(&self) -> LC_FIFO_TIMEOUT_SHIFT_R {
        LC_FIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - The enable bit for FIFO timeout"]
    #[inline(always)]
    pub fn lc_fifo_timeout_ena(&self) -> LC_FIFO_TIMEOUT_ENA_R {
        LC_FIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
    #[inline(always)]
    #[must_use]
    pub fn lc_fifo_timeout(&mut self) -> LC_FIFO_TIMEOUT_W<0> {
        LC_FIFO_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 8:10 - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
    #[inline(always)]
    #[must_use]
    pub fn lc_fifo_timeout_shift(&mut self) -> LC_FIFO_TIMEOUT_SHIFT_W<8> {
        LC_FIFO_TIMEOUT_SHIFT_W::new(self)
    }
    #[doc = "Bit 11 - The enable bit for FIFO timeout"]
    #[inline(always)]
    #[must_use]
    pub fn lc_fifo_timeout_ena(&mut self) -> LC_FIFO_TIMEOUT_ENA_W<11> {
        LC_FIFO_TIMEOUT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S HUNG configure register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_hung_conf](index.html) module"]
pub struct LC_HUNG_CONF_SPEC;
impl crate::RegisterSpec for LC_HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_hung_conf::R](R) reader structure"]
impl crate::Readable for LC_HUNG_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lc_hung_conf::W](W) writer structure"]
impl crate::Writable for LC_HUNG_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LC_HUNG_CONF to value 0x0810"]
impl crate::Resettable for LC_HUNG_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0810;
}
