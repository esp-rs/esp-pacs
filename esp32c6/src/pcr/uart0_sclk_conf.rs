#[doc = "Register `UART0_SCLK_CONF` reader"]
pub struct R(crate::R<UART0_SCLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART0_SCLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART0_SCLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART0_SCLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART0_SCLK_CONF` writer"]
pub struct W(crate::W<UART0_SCLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART0_SCLK_CONF_SPEC>;
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
impl From<crate::W<UART0_SCLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART0_SCLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0_SCLK_DIV_A` reader - The denominator of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_A_R = crate::FieldReader;
#[doc = "Field `UART0_SCLK_DIV_A` writer - The denominator of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_A_W<'a, const O: u8> = crate::FieldWriter<'a, UART0_SCLK_CONF_SPEC, 6, O>;
#[doc = "Field `UART0_SCLK_DIV_B` reader - The numerator of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_B_R = crate::FieldReader;
#[doc = "Field `UART0_SCLK_DIV_B` writer - The numerator of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_B_W<'a, const O: u8> = crate::FieldWriter<'a, UART0_SCLK_CONF_SPEC, 6, O>;
#[doc = "Field `UART0_SCLK_DIV_NUM` reader - The integral part of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `UART0_SCLK_DIV_NUM` writer - The integral part of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, UART0_SCLK_CONF_SPEC, 8, O>;
#[doc = "Field `UART0_SCLK_SEL` reader - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
pub type UART0_SCLK_SEL_R = crate::FieldReader;
#[doc = "Field `UART0_SCLK_SEL` writer - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
pub type UART0_SCLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, UART0_SCLK_CONF_SPEC, 2, O>;
#[doc = "Field `UART0_SCLK_EN` reader - Set 1 to enable uart0 function clock"]
pub type UART0_SCLK_EN_R = crate::BitReader;
#[doc = "Field `UART0_SCLK_EN` writer - Set 1 to enable uart0 function clock"]
pub type UART0_SCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, UART0_SCLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_a(&self) -> UART0_SCLK_DIV_A_R {
        UART0_SCLK_DIV_A_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_b(&self) -> UART0_SCLK_DIV_B_R {
        UART0_SCLK_DIV_B_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_num(&self) -> UART0_SCLK_DIV_NUM_R {
        UART0_SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
    #[inline(always)]
    pub fn uart0_sclk_sel(&self) -> UART0_SCLK_SEL_R {
        UART0_SCLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable uart0 function clock"]
    #[inline(always)]
    pub fn uart0_sclk_en(&self) -> UART0_SCLK_EN_R {
        UART0_SCLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0_SCLK_CONF")
            .field(
                "uart0_sclk_div_a",
                &format_args!("{}", self.uart0_sclk_div_a().bits()),
            )
            .field(
                "uart0_sclk_div_b",
                &format_args!("{}", self.uart0_sclk_div_b().bits()),
            )
            .field(
                "uart0_sclk_div_num",
                &format_args!("{}", self.uart0_sclk_div_num().bits()),
            )
            .field(
                "uart0_sclk_sel",
                &format_args!("{}", self.uart0_sclk_sel().bits()),
            )
            .field(
                "uart0_sclk_en",
                &format_args!("{}", self.uart0_sclk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART0_SCLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    #[must_use]
    pub fn uart0_sclk_div_a(&mut self) -> UART0_SCLK_DIV_A_W<0> {
        UART0_SCLK_DIV_A_W::new(self)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    #[must_use]
    pub fn uart0_sclk_div_b(&mut self) -> UART0_SCLK_DIV_B_W<6> {
        UART0_SCLK_DIV_B_W::new(self)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    #[must_use]
    pub fn uart0_sclk_div_num(&mut self) -> UART0_SCLK_DIV_NUM_W<12> {
        UART0_SCLK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn uart0_sclk_sel(&mut self) -> UART0_SCLK_SEL_W<20> {
        UART0_SCLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable uart0 function clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart0_sclk_en(&mut self) -> UART0_SCLK_EN_W<22> {
        UART0_SCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0_SCLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_sclk_conf](index.html) module"]
pub struct UART0_SCLK_CONF_SPEC;
impl crate::RegisterSpec for UART0_SCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart0_sclk_conf::R](R) reader structure"]
impl crate::Readable for UART0_SCLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart0_sclk_conf::W](W) writer structure"]
impl crate::Writable for UART0_SCLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART0_SCLK_CONF to value 0x0070_0000"]
impl crate::Resettable for UART0_SCLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0070_0000;
}
