#[doc = "Register `CH%s_RX_CONF0` reader"]
pub struct R(crate::R<CH_RX_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_RX_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_RX_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_RX_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_RX_CONF0` writer"]
pub struct W(crate::W<CH_RX_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_RX_CONF0_SPEC>;
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
impl From<crate::W<CH_RX_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_RX_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_CNT` reader - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, CH_RX_CONF0_SPEC, 8, O>;
#[doc = "Field `IDLE_THRES` reader - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub type IDLE_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `IDLE_THRES` writer - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub type IDLE_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, CH_RX_CONF0_SPEC, 15, O, u16>;
#[doc = "Field `MEM_SIZE` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, CH_RX_CONF0_SPEC, 4, O>;
#[doc = "Field `CARRIER_EN` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_W<'a, const O: u8> = crate::BitWriter<'a, CH_RX_CONF0_SPEC, O>;
#[doc = "Field `CARRIER_OUT_LV` reader - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_W<'a, const O: u8> = crate::BitWriter<'a, CH_RX_CONF0_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CONF0")
            .field("div_cnt", &format_args!("{}", self.div_cnt().bits()))
            .field("idle_thres", &format_args!("{}", self.idle_thres().bits()))
            .field("mem_size", &format_args!("{}", self.mem_size().bits()))
            .field("carrier_en", &format_args!("{}", self.carrier_en().bit()))
            .field(
                "carrier_out_lv",
                &format_args!("{}", self.carrier_out_lv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_RX_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<0> {
        DIV_CNT_W::new(self)
    }
    #[doc = "Bits 8:22 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    #[must_use]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W<8> {
        IDLE_THRES_W::new(self)
    }
    #[doc = "Bits 24:27 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<24> {
        MEM_SIZE_W::new(self)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<28> {
        CARRIER_EN_W::new(self)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<29> {
        CARRIER_OUT_LV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s configure register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_rx_conf0](index.html) module"]
pub struct CH_RX_CONF0_SPEC;
impl crate::RegisterSpec for CH_RX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_rx_conf0::R](R) reader structure"]
impl crate::Readable for CH_RX_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_rx_conf0::W](W) writer structure"]
impl crate::Writable for CH_RX_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_RX_CONF0 to value 0x317f_ff02"]
impl crate::Resettable for CH_RX_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x317f_ff02;
}
