#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_CLK_SEL` reader - This bit is used to select clock source for the 4 timers . 2'd1: APB_CLK 2'd2: RTC8M_CLK 2'd3: XTAL_CLK"]
pub type APB_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `APB_CLK_SEL` writer - This bit is used to select clock source for the 4 timers . 2'd1: APB_CLK 2'd2: RTC8M_CLK 2'd3: XTAL_CLK"]
pub type APB_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_SPEC, 2, O>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH0` reader - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH0_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH0` writer - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH0_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH1` reader - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH1_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH1` writer - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH1_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH2` reader - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH2_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH2` writer - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH2_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH3` reader - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH3_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH3` writer - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH3_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH4` reader - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH4_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH4` writer - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH4_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH5` reader - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH5_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH5` writer - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
pub type GAMMA_RAM_CLK_EN_CH5_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `CLK_EN` reader - This bit is used to control clock. 1'b1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - This bit is used to control clock. 1'b1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - This bit is used to select clock source for the 4 timers . 2'd1: APB_CLK 2'd2: RTC8M_CLK 2'd3: XTAL_CLK"]
    #[inline(always)]
    pub fn apb_clk_sel(&self) -> APB_CLK_SEL_R {
        APB_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch0(&self) -> GAMMA_RAM_CLK_EN_CH0_R {
        GAMMA_RAM_CLK_EN_CH0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch1(&self) -> GAMMA_RAM_CLK_EN_CH1_R {
        GAMMA_RAM_CLK_EN_CH1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch2(&self) -> GAMMA_RAM_CLK_EN_CH2_R {
        GAMMA_RAM_CLK_EN_CH2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch3(&self) -> GAMMA_RAM_CLK_EN_CH3_R {
        GAMMA_RAM_CLK_EN_CH3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch4(&self) -> GAMMA_RAM_CLK_EN_CH4_R {
        GAMMA_RAM_CLK_EN_CH4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch5(&self) -> GAMMA_RAM_CLK_EN_CH5_R {
        GAMMA_RAM_CLK_EN_CH5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is used to control clock. 1'b1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field(
                "apb_clk_sel",
                &format_args!("{}", self.apb_clk_sel().bits()),
            )
            .field(
                "gamma_ram_clk_en_ch0",
                &format_args!("{}", self.gamma_ram_clk_en_ch0().bit()),
            )
            .field(
                "gamma_ram_clk_en_ch1",
                &format_args!("{}", self.gamma_ram_clk_en_ch1().bit()),
            )
            .field(
                "gamma_ram_clk_en_ch2",
                &format_args!("{}", self.gamma_ram_clk_en_ch2().bit()),
            )
            .field(
                "gamma_ram_clk_en_ch3",
                &format_args!("{}", self.gamma_ram_clk_en_ch3().bit()),
            )
            .field(
                "gamma_ram_clk_en_ch4",
                &format_args!("{}", self.gamma_ram_clk_en_ch4().bit()),
            )
            .field(
                "gamma_ram_clk_en_ch5",
                &format_args!("{}", self.gamma_ram_clk_en_ch5().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This bit is used to select clock source for the 4 timers . 2'd1: APB_CLK 2'd2: RTC8M_CLK 2'd3: XTAL_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn apb_clk_sel(&mut self) -> APB_CLK_SEL_W<0> {
        APB_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    #[must_use]
    pub fn gamma_ram_clk_en_ch0(&mut self) -> GAMMA_RAM_CLK_EN_CH0_W<2> {
        GAMMA_RAM_CLK_EN_CH0_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    #[must_use]
    pub fn gamma_ram_clk_en_ch1(&mut self) -> GAMMA_RAM_CLK_EN_CH1_W<3> {
        GAMMA_RAM_CLK_EN_CH1_W::new(self)
    }
    #[doc = "Bit 4 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    #[must_use]
    pub fn gamma_ram_clk_en_ch2(&mut self) -> GAMMA_RAM_CLK_EN_CH2_W<4> {
        GAMMA_RAM_CLK_EN_CH2_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    #[must_use]
    pub fn gamma_ram_clk_en_ch3(&mut self) -> GAMMA_RAM_CLK_EN_CH3_W<5> {
        GAMMA_RAM_CLK_EN_CH3_W::new(self)
    }
    #[doc = "Bit 6 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    #[must_use]
    pub fn gamma_ram_clk_en_ch4(&mut self) -> GAMMA_RAM_CLK_EN_CH4_W<6> {
        GAMMA_RAM_CLK_EN_CH4_W::new(self)
    }
    #[doc = "Bit 7 - This bit is used to control clock. 1'b1: Force clock on for gamma ram. 1'h0: Support clock only when application writes or read gamma ram."]
    #[inline(always)]
    #[must_use]
    pub fn gamma_ram_clk_en_ch5(&mut self) -> GAMMA_RAM_CLK_EN_CH5_W<7> {
        GAMMA_RAM_CLK_EN_CH5_W::new(self)
    }
    #[doc = "Bit 31 - This bit is used to control clock. 1'b1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
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
#[doc = "Global ledc configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
