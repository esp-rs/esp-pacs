#[doc = "Register `RX_CH%sCONF0` reader"]
pub struct R(crate::R<RX_CHCONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CHCONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CHCONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CHCONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CH%sCONF0` writer"]
pub struct W(crate::W<RX_CHCONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CHCONF0_SPEC>;
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
impl From<crate::W<RX_CHCONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CHCONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_CNT_CH2` reader - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_CH2_R = crate::FieldReader;
#[doc = "Field `DIV_CNT_CH2` writer - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_CH2_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CHCONF0_SPEC, 8, O>;
#[doc = "Field `IDLE_THRES_CH2` reader - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub type IDLE_THRES_CH2_R = crate::FieldReader<u16>;
#[doc = "Field `IDLE_THRES_CH2` writer - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub type IDLE_THRES_CH2_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CHCONF0_SPEC, 15, O, u16>;
#[doc = "Field `MEM_SIZE_CH2` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_CH2_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE_CH2` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_CH2_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CHCONF0_SPEC, 3, O>;
#[doc = "Field `CARRIER_EN_CH2` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_CH2_R = crate::BitReader;
#[doc = "Field `CARRIER_EN_CH2` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF0_SPEC, O>;
#[doc = "Field `CARRIER_OUT_LV_CH2` reader - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_CH2_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV_CH2` writer - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF0_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt_ch2(&self) -> DIV_CNT_CH2_R {
        DIV_CNT_CH2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    pub fn idle_thres_ch2(&self) -> IDLE_THRES_CH2_R {
        IDLE_THRES_CH2_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 23:25 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size_ch2(&self) -> MEM_SIZE_CH2_R {
        MEM_SIZE_CH2_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en_ch2(&self) -> CARRIER_EN_CH2_R {
        CARRIER_EN_CH2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch2(&self) -> CARRIER_OUT_LV_CH2_R {
        CARRIER_OUT_LV_CH2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CHCONF0")
            .field(
                "div_cnt_ch2",
                &format_args!("{}", self.div_cnt_ch2().bits()),
            )
            .field(
                "idle_thres_ch2",
                &format_args!("{}", self.idle_thres_ch2().bits()),
            )
            .field(
                "mem_size_ch2",
                &format_args!("{}", self.mem_size_ch2().bits()),
            )
            .field(
                "carrier_en_ch2",
                &format_args!("{}", self.carrier_en_ch2().bit()),
            )
            .field(
                "carrier_out_lv_ch2",
                &format_args!("{}", self.carrier_out_lv_ch2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CHCONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn div_cnt_ch2(&mut self) -> DIV_CNT_CH2_W<0> {
        DIV_CNT_CH2_W::new(self)
    }
    #[doc = "Bits 8:22 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    #[must_use]
    pub fn idle_thres_ch2(&mut self) -> IDLE_THRES_CH2_W<8> {
        IDLE_THRES_CH2_W::new(self)
    }
    #[doc = "Bits 23:25 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size_ch2(&mut self) -> MEM_SIZE_CH2_W<23> {
        MEM_SIZE_CH2_W::new(self)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_en_ch2(&mut self) -> CARRIER_EN_CH2_W<28> {
        CARRIER_EN_CH2_W::new(self)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv_ch2(&mut self) -> CARRIER_OUT_LV_CH2_W<29> {
        CARRIER_OUT_LV_CH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s configure register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_chconf0](index.html) module"]
pub struct RX_CHCONF0_SPEC;
impl crate::RegisterSpec for RX_CHCONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_chconf0::R](R) reader structure"]
impl crate::Readable for RX_CHCONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_chconf0::W](W) writer structure"]
impl crate::Writable for RX_CHCONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CH%sCONF0 to value 0x30ff_ff02"]
impl crate::Resettable for RX_CHCONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x30ff_ff02;
}
