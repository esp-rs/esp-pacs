#[doc = "Register `SYSCLK_CONF` reader"]
pub struct R(crate::R<SYSCLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCLK_CONF` writer"]
pub struct W(crate::W<SYSCLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCLK_CONF_SPEC>;
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
impl From<crate::W<SYSCLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_DIV_CNT` reader - "]
pub type PRE_DIV_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRE_DIV_CNT` writer - "]
pub type PRE_DIV_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCLK_CONF_SPEC, 10, O, u16, u16>;
#[doc = "Field `CLK_320M_EN` reader - "]
pub type CLK_320M_EN_R = crate::BitReader;
#[doc = "Field `CLK_320M_EN` writer - "]
pub type CLK_320M_EN_W<'a, const O: u8> = crate::BitWriter<'a, SYSCLK_CONF_SPEC, O>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, SYSCLK_CONF_SPEC, O>;
#[doc = "Field `RST_TICK_CNT` reader - "]
pub type RST_TICK_CNT_R = crate::BitReader;
#[doc = "Field `RST_TICK_CNT` writer - "]
pub type RST_TICK_CNT_W<'a, const O: u8> = crate::BitWriter<'a, SYSCLK_CONF_SPEC, O>;
#[doc = "Field `QUICK_CLK_CHNG` reader - "]
pub type QUICK_CLK_CHNG_R = crate::BitReader;
#[doc = "Field `QUICK_CLK_CHNG` writer - "]
pub type QUICK_CLK_CHNG_W<'a, const O: u8> = crate::BitWriter<'a, SYSCLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn pre_div_cnt(&self) -> PRE_DIV_CNT_R {
        PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_320m_en(&self) -> CLK_320M_EN_R {
        CLK_320M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rst_tick_cnt(&self) -> RST_TICK_CNT_R {
        RST_TICK_CNT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn quick_clk_chng(&self) -> QUICK_CLK_CHNG_R {
        QUICK_CLK_CHNG_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCLK_CONF")
            .field(
                "pre_div_cnt",
                &format_args!("{}", self.pre_div_cnt().bits()),
            )
            .field("clk_320m_en", &format_args!("{}", self.clk_320m_en().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field(
                "rst_tick_cnt",
                &format_args!("{}", self.rst_tick_cnt().bit()),
            )
            .field(
                "quick_clk_chng",
                &format_args!("{}", self.quick_clk_chng().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYSCLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn pre_div_cnt(&mut self) -> PRE_DIV_CNT_W<0> {
        PRE_DIV_CNT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_320m_en(&mut self) -> CLK_320M_EN_W<10> {
        CLK_320M_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<11> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rst_tick_cnt(&mut self) -> RST_TICK_CNT_W<12> {
        RST_TICK_CNT_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn quick_clk_chng(&mut self) -> QUICK_CLK_CHNG_W<13> {
        QUICK_CLK_CHNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclk_conf](index.html) module"]
pub struct SYSCLK_CONF_SPEC;
impl crate::RegisterSpec for SYSCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysclk_conf::R](R) reader structure"]
impl crate::Readable for SYSCLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysclk_conf::W](W) writer structure"]
impl crate::Writable for SYSCLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCLK_CONF to value 0x2000"]
impl crate::Resettable for SYSCLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
