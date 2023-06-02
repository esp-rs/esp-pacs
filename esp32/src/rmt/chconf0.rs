#[doc = "Register `CH%sCONF0` reader"]
pub struct R(crate::R<CHCONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sCONF0` writer"]
pub struct W(crate::W<CHCONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCONF0_SPEC>;
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
impl From<crate::W<CHCONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_CNT` reader - This register is used to configure the frequency divider's factor in channel0."]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - This register is used to configure the frequency divider's factor in channel0."]
pub type DIV_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, CHCONF0_SPEC, 8, O>;
#[doc = "Field `IDLE_THRES` reader - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
pub type IDLE_THRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IDLE_THRES` writer - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
pub type IDLE_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, CHCONF0_SPEC, 16, O, u16, u16>;
#[doc = "Field `MEM_SIZE` reader - This register is used to configure the the amount of memory blocks allocated to channel0."]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - This register is used to configure the the amount of memory blocks allocated to channel0."]
pub type MEM_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, CHCONF0_SPEC, 4, O>;
#[doc = "Field `CARRIER_EN` reader - This is the carrier modulation enable control bit for channel0."]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - This is the carrier modulation enable control bit for channel0."]
pub type CARRIER_EN_W<'a, const O: u8> = crate::BitWriter<'a, CHCONF0_SPEC, O>;
#[doc = "Field `CARRIER_OUT_LV` reader - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
pub type CARRIER_OUT_LV_W<'a, const O: u8> = crate::BitWriter<'a, CHCONF0_SPEC, O>;
#[doc = "Field `MEM_PD` reader - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
pub type MEM_PD_R = crate::BitReader;
#[doc = "Field `MEM_PD` writer - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
pub type MEM_PD_W<'a, const O: u8> = crate::BitWriter<'a, CHCONF0_SPEC, O>;
#[doc = "Field `CLK_EN` reader - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CHCONF0_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel0."]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel0."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel0."]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
    #[inline(always)]
    pub fn mem_pd(&self) -> MEM_PD_R {
        MEM_PD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCONF0")
            .field("div_cnt", &format_args!("{}", self.div_cnt().bits()))
            .field("idle_thres", &format_args!("{}", self.idle_thres().bits()))
            .field("mem_size", &format_args!("{}", self.mem_size().bits()))
            .field("carrier_en", &format_args!("{}", self.carrier_en().bit()))
            .field(
                "carrier_out_lv",
                &format_args!("{}", self.carrier_out_lv().bit()),
            )
            .field("mem_pd", &format_args!("{}", self.mem_pd().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHCONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel0."]
    #[inline(always)]
    #[must_use]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<0> {
        DIV_CNT_W::new(self)
    }
    #[doc = "Bits 8:23 - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
    #[inline(always)]
    #[must_use]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W<8> {
        IDLE_THRES_W::new(self)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel0."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<24> {
        MEM_SIZE_W::new(self)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel0."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<28> {
        CARRIER_EN_W::new(self)
    }
    #[doc = "Bit 29 - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<29> {
        CARRIER_OUT_LV_W::new(self)
    }
    #[doc = "Bit 30 - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
    #[inline(always)]
    #[must_use]
    pub fn mem_pd(&mut self) -> MEM_PD_W<30> {
        MEM_PD_W::new(self)
    }
    #[doc = "Bit 31 - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chconf0](index.html) module"]
pub struct CHCONF0_SPEC;
impl crate::RegisterSpec for CHCONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chconf0::R](R) reader structure"]
impl crate::Readable for CHCONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chconf0::W](W) writer structure"]
impl crate::Writable for CHCONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sCONF0 to value 0x3110_0002"]
impl crate::Resettable for CHCONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3110_0002;
}
