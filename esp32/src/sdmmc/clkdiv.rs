#[doc = "Register `CLKDIV` reader"]
pub struct R(crate::R<CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV` writer"]
pub struct W(crate::W<CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV_SPEC>;
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
impl From<crate::W<CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_DIVIDER0` reader - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER0_R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER0` writer - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER0_W<'a, const O: u8> = crate::FieldWriter<'a, CLKDIV_SPEC, 8, O>;
#[doc = "Field `CLK_DIVIDER1` reader - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER1_R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER1` writer - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER1_W<'a, const O: u8> = crate::FieldWriter<'a, CLKDIV_SPEC, 8, O>;
#[doc = "Field `CLK_DIVIDER2` reader - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER2_R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER2` writer - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER2_W<'a, const O: u8> = crate::FieldWriter<'a, CLKDIV_SPEC, 8, O>;
#[doc = "Field `CLK_DIVIDER3` reader - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER3_R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER3` writer - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER3_W<'a, const O: u8> = crate::FieldWriter<'a, CLKDIV_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider0(&self) -> CLK_DIVIDER0_R {
        CLK_DIVIDER0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider1(&self) -> CLK_DIVIDER1_R {
        CLK_DIVIDER1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider2(&self) -> CLK_DIVIDER2_R {
        CLK_DIVIDER2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider3(&self) -> CLK_DIVIDER3_R {
        CLK_DIVIDER3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV")
            .field(
                "clk_divider0",
                &format_args!("{}", self.clk_divider0().bits()),
            )
            .field(
                "clk_divider1",
                &format_args!("{}", self.clk_divider1().bits()),
            )
            .field(
                "clk_divider2",
                &format_args!("{}", self.clk_divider2().bits()),
            )
            .field(
                "clk_divider3",
                &format_args!("{}", self.clk_divider3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKDIV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    #[must_use]
    pub fn clk_divider0(&mut self) -> CLK_DIVIDER0_W<0> {
        CLK_DIVIDER0_W::new(self)
    }
    #[doc = "Bits 8:15 - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    #[must_use]
    pub fn clk_divider1(&mut self) -> CLK_DIVIDER1_W<8> {
        CLK_DIVIDER1_W::new(self)
    }
    #[doc = "Bits 16:23 - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    #[must_use]
    pub fn clk_divider2(&mut self) -> CLK_DIVIDER2_W<16> {
        CLK_DIVIDER2_W::new(self)
    }
    #[doc = "Bits 24:31 - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    #[must_use]
    pub fn clk_divider3(&mut self) -> CLK_DIVIDER3_W<24> {
        CLK_DIVIDER3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divider configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](index.html) module"]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv::R](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
