#[doc = "Register `TOUT_CONF` reader"]
pub struct R(crate::R<TOUT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUT_CONF` writer"]
pub struct W(crate::W<TOUT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUT_CONF_SPEC>;
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
impl From<crate::W<TOUT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_TOUT_EN` reader - This is the enble bit for uart receiver's timeout function."]
pub type RX_TOUT_EN_R = crate::BitReader;
#[doc = "Field `RX_TOUT_EN` writer - This is the enble bit for uart receiver's timeout function."]
pub type RX_TOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, TOUT_CONF_SPEC, O>;
#[doc = "Field `RX_TOUT_FLOW_DIS` reader - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
pub type RX_TOUT_FLOW_DIS_R = crate::BitReader;
#[doc = "Field `RX_TOUT_FLOW_DIS` writer - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
pub type RX_TOUT_FLOW_DIS_W<'a, const O: u8> = crate::BitWriter<'a, TOUT_CONF_SPEC, O>;
#[doc = "Field `RX_TOUT_THRHD` reader - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub type RX_TOUT_THRHD_R = crate::FieldReader<u16>;
#[doc = "Field `RX_TOUT_THRHD` writer - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub type RX_TOUT_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, TOUT_CONF_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bit 0 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RX_TOUT_EN_R {
        RX_TOUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    #[inline(always)]
    pub fn rx_tout_flow_dis(&self) -> RX_TOUT_FLOW_DIS_R {
        RX_TOUT_FLOW_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUT_CONF")
            .field("rx_tout_en", &format_args!("{}", self.rx_tout_en().bit()))
            .field(
                "rx_tout_flow_dis",
                &format_args!("{}", self.rx_tout_flow_dis().bit()),
            )
            .field(
                "rx_tout_thrhd",
                &format_args!("{}", self.rx_tout_thrhd().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUT_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_en(&mut self) -> RX_TOUT_EN_W<0> {
        RX_TOUT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_flow_dis(&mut self) -> RX_TOUT_FLOW_DIS_W<1> {
        RX_TOUT_FLOW_DIS_W::new(self)
    }
    #[doc = "Bits 2:11 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W<2> {
        RX_TOUT_THRHD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART threshold and allocation configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tout_conf](index.html) module"]
pub struct TOUT_CONF_SPEC;
impl crate::RegisterSpec for TOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tout_conf::R](R) reader structure"]
impl crate::Readable for TOUT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tout_conf::W](W) writer structure"]
impl crate::Writable for TOUT_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUT_CONF to value 0x28"]
impl crate::Resettable for TOUT_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}
