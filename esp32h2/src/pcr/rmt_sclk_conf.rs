#[doc = "Register `RMT_SCLK_CONF` reader"]
pub struct R(crate::R<RMT_SCLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMT_SCLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMT_SCLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMT_SCLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMT_SCLK_CONF` writer"]
pub struct W(crate::W<RMT_SCLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMT_SCLK_CONF_SPEC>;
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
impl From<crate::W<RMT_SCLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMT_SCLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMT_SCLK_DIV_A` reader - The denominator of the frequency divider factor of the rmt function clock."]
pub type RMT_SCLK_DIV_A_R = crate::FieldReader;
#[doc = "Field `RMT_SCLK_DIV_A` writer - The denominator of the frequency divider factor of the rmt function clock."]
pub type RMT_SCLK_DIV_A_W<'a, const O: u8> = crate::FieldWriter<'a, RMT_SCLK_CONF_SPEC, 6, O>;
#[doc = "Field `RMT_SCLK_DIV_B` reader - The numerator of the frequency divider factor of the rmt function clock."]
pub type RMT_SCLK_DIV_B_R = crate::FieldReader;
#[doc = "Field `RMT_SCLK_DIV_B` writer - The numerator of the frequency divider factor of the rmt function clock."]
pub type RMT_SCLK_DIV_B_W<'a, const O: u8> = crate::FieldWriter<'a, RMT_SCLK_CONF_SPEC, 6, O>;
#[doc = "Field `RMT_SCLK_DIV_NUM` reader - The integral part of the frequency divider factor of the rmt function clock."]
pub type RMT_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `RMT_SCLK_DIV_NUM` writer - The integral part of the frequency divider factor of the rmt function clock."]
pub type RMT_SCLK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, RMT_SCLK_CONF_SPEC, 8, O>;
#[doc = "Field `RMT_SCLK_SEL` reader - set this field to select clock-source. 0: do not select anyone clock, 1(default): 80MHz, 2: FOSC, 3: XTAL."]
pub type RMT_SCLK_SEL_R = crate::BitReader;
#[doc = "Field `RMT_SCLK_SEL` writer - set this field to select clock-source. 0: do not select anyone clock, 1(default): 80MHz, 2: FOSC, 3: XTAL."]
pub type RMT_SCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, RMT_SCLK_CONF_SPEC, O>;
#[doc = "Field `RMT_SCLK_EN` reader - Set 1 to enable rmt function clock"]
pub type RMT_SCLK_EN_R = crate::BitReader;
#[doc = "Field `RMT_SCLK_EN` writer - Set 1 to enable rmt function clock"]
pub type RMT_SCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, RMT_SCLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn rmt_sclk_div_a(&self) -> RMT_SCLK_DIV_A_R {
        RMT_SCLK_DIV_A_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn rmt_sclk_div_b(&self) -> RMT_SCLK_DIV_B_R {
        RMT_SCLK_DIV_B_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn rmt_sclk_div_num(&self) -> RMT_SCLK_DIV_NUM_R {
        RMT_SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - set this field to select clock-source. 0: do not select anyone clock, 1(default): 80MHz, 2: FOSC, 3: XTAL."]
    #[inline(always)]
    pub fn rmt_sclk_sel(&self) -> RMT_SCLK_SEL_R {
        RMT_SCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set 1 to enable rmt function clock"]
    #[inline(always)]
    pub fn rmt_sclk_en(&self) -> RMT_SCLK_EN_R {
        RMT_SCLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMT_SCLK_CONF")
            .field(
                "rmt_sclk_div_a",
                &format_args!("{}", self.rmt_sclk_div_a().bits()),
            )
            .field(
                "rmt_sclk_div_b",
                &format_args!("{}", self.rmt_sclk_div_b().bits()),
            )
            .field(
                "rmt_sclk_div_num",
                &format_args!("{}", self.rmt_sclk_div_num().bits()),
            )
            .field(
                "rmt_sclk_sel",
                &format_args!("{}", self.rmt_sclk_sel().bit()),
            )
            .field("rmt_sclk_en", &format_args!("{}", self.rmt_sclk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RMT_SCLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    #[must_use]
    pub fn rmt_sclk_div_a(&mut self) -> RMT_SCLK_DIV_A_W<0> {
        RMT_SCLK_DIV_A_W::new(self)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    #[must_use]
    pub fn rmt_sclk_div_b(&mut self) -> RMT_SCLK_DIV_B_W<6> {
        RMT_SCLK_DIV_B_W::new(self)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    #[must_use]
    pub fn rmt_sclk_div_num(&mut self) -> RMT_SCLK_DIV_NUM_W<12> {
        RMT_SCLK_DIV_NUM_W::new(self)
    }
    #[doc = "Bit 20 - set this field to select clock-source. 0: do not select anyone clock, 1(default): 80MHz, 2: FOSC, 3: XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn rmt_sclk_sel(&mut self) -> RMT_SCLK_SEL_W<20> {
        RMT_SCLK_SEL_W::new(self)
    }
    #[doc = "Bit 21 - Set 1 to enable rmt function clock"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_sclk_en(&mut self) -> RMT_SCLK_EN_W<21> {
        RMT_SCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_SCLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_sclk_conf](index.html) module"]
pub struct RMT_SCLK_CONF_SPEC;
impl crate::RegisterSpec for RMT_SCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmt_sclk_conf::R](R) reader structure"]
impl crate::Readable for RMT_SCLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmt_sclk_conf::W](W) writer structure"]
impl crate::Writable for RMT_SCLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMT_SCLK_CONF to value 0x0030_1000"]
impl crate::Resettable for RMT_SCLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_1000;
}
