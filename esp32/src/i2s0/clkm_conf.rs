#[doc = "Register `CLKM_CONF` reader"]
pub struct R(crate::R<CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKM_CONF` writer"]
pub struct W(crate::W<CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKM_CONF_SPEC>;
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
impl From<crate::W<CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKM_DIV_NUM` reader - "]
pub type CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CLKM_DIV_NUM` writer - "]
pub type CLKM_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CLKM_CONF_SPEC, 8, O>;
#[doc = "Field `CLKM_DIV_B` reader - "]
pub type CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `CLKM_DIV_B` writer - "]
pub type CLKM_DIV_B_W<'a, const O: u8> = crate::FieldWriter<'a, CLKM_CONF_SPEC, 6, O>;
#[doc = "Field `CLKM_DIV_A` reader - "]
pub type CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `CLKM_DIV_A` writer - "]
pub type CLKM_DIV_A_W<'a, const O: u8> = crate::FieldWriter<'a, CLKM_CONF_SPEC, 6, O>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLKM_CONF_SPEC, O>;
#[doc = "Field `CLKA_ENA` reader - "]
pub type CLKA_ENA_R = crate::BitReader;
#[doc = "Field `CLKA_ENA` writer - "]
pub type CLKA_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CLKM_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkm_div_num(&self) -> CLKM_DIV_NUM_R {
        CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn clkm_div_b(&self) -> CLKM_DIV_B_R {
        CLKM_DIV_B_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn clkm_div_a(&self) -> CLKM_DIV_A_R {
        CLKM_DIV_A_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clka_ena(&self) -> CLKA_ENA_R {
        CLKA_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKM_CONF")
            .field(
                "clkm_div_num",
                &format_args!("{}", self.clkm_div_num().bits()),
            )
            .field("clkm_div_b", &format_args!("{}", self.clkm_div_b().bits()))
            .field("clkm_div_a", &format_args!("{}", self.clkm_div_a().bits()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field("clka_ena", &format_args!("{}", self.clka_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_num(&mut self) -> CLKM_DIV_NUM_W<0> {
        CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_b(&mut self) -> CLKM_DIV_B_W<8> {
        CLKM_DIV_B_W::new(self)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_a(&mut self) -> CLKM_DIV_A_W<14> {
        CLKM_DIV_A_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<20> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn clka_ena(&mut self) -> CLKA_ENA_W<21> {
        CLKA_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkm_conf](index.html) module"]
pub struct CLKM_CONF_SPEC;
impl crate::RegisterSpec for CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkm_conf::R](R) reader structure"]
impl crate::Readable for CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkm_conf::W](W) writer structure"]
impl crate::Writable for CLKM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKM_CONF to value 0x04"]
impl crate::Resettable for CLKM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
