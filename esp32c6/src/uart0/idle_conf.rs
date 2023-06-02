#[doc = "Register `IDLE_CONF` reader"]
pub struct R(crate::R<IDLE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLE_CONF` writer"]
pub struct W(crate::W<IDLE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLE_CONF_SPEC>;
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
impl From<crate::W<IDLE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_IDLE_THRHD` reader - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
pub type RX_IDLE_THRHD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_IDLE_THRHD` writer - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
pub type RX_IDLE_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, IDLE_CONF_SPEC, 10, O, u16, u16>;
#[doc = "Field `TX_IDLE_NUM` reader - This register is used to configure the duration time between transfers."]
pub type TX_IDLE_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_IDLE_NUM` writer - This register is used to configure the duration time between transfers."]
pub type TX_IDLE_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, IDLE_CONF_SPEC, 10, O, u16, u16>;
impl R {
    #[doc = "Bits 0:9 - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
    #[inline(always)]
    pub fn rx_idle_thrhd(&self) -> RX_IDLE_THRHD_R {
        RX_IDLE_THRHD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to configure the duration time between transfers."]
    #[inline(always)]
    pub fn tx_idle_num(&self) -> TX_IDLE_NUM_R {
        TX_IDLE_NUM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE_CONF")
            .field(
                "rx_idle_thrhd",
                &format_args!("{}", self.rx_idle_thrhd().bits()),
            )
            .field(
                "tx_idle_num",
                &format_args!("{}", self.tx_idle_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IDLE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
    #[inline(always)]
    #[must_use]
    pub fn rx_idle_thrhd(&mut self) -> RX_IDLE_THRHD_W<0> {
        RX_IDLE_THRHD_W::new(self)
    }
    #[doc = "Bits 10:19 - This register is used to configure the duration time between transfers."]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle_num(&mut self) -> TX_IDLE_NUM_W<10> {
        TX_IDLE_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame-end idle configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle_conf](index.html) module"]
pub struct IDLE_CONF_SPEC;
impl crate::RegisterSpec for IDLE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idle_conf::R](R) reader structure"]
impl crate::Readable for IDLE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idle_conf::W](W) writer structure"]
impl crate::Writable for IDLE_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDLE_CONF to value 0x0004_0100"]
impl crate::Resettable for IDLE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0100;
}
