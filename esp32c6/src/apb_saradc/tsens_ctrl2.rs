#[doc = "Register `TSENS_CTRL2` reader"]
pub struct R(crate::R<TSENS_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSENS_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSENS_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSENS_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSENS_CTRL2` writer"]
pub struct W(crate::W<TSENS_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSENS_CTRL2_SPEC>;
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
impl From<crate::W<TSENS_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSENS_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_XPD_WAIT` reader - the time that power up tsens need wait"]
pub type TSENS_XPD_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `TSENS_XPD_WAIT` writer - the time that power up tsens need wait"]
pub type TSENS_XPD_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, TSENS_CTRL2_SPEC, 12, O, u16>;
#[doc = "Field `TSENS_XPD_FORCE` reader - force power up tsens"]
pub type TSENS_XPD_FORCE_R = crate::FieldReader;
#[doc = "Field `TSENS_XPD_FORCE` writer - force power up tsens"]
pub type TSENS_XPD_FORCE_W<'a, const O: u8> = crate::FieldWriter<'a, TSENS_CTRL2_SPEC, 2, O>;
#[doc = "Field `TSENS_CLK_INV` reader - inv tsens clk"]
pub type TSENS_CLK_INV_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_INV` writer - inv tsens clk"]
pub type TSENS_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, TSENS_CTRL2_SPEC, O>;
#[doc = "Field `TSENS_CLK_SEL` reader - tsens clk select"]
pub type TSENS_CLK_SEL_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_SEL` writer - tsens clk select"]
pub type TSENS_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TSENS_CTRL2_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - the time that power up tsens need wait"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - force power up tsens"]
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - inv tsens clk"]
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tsens clk select"]
    #[inline(always)]
    pub fn tsens_clk_sel(&self) -> TSENS_CLK_SEL_R {
        TSENS_CLK_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSENS_CTRL2")
            .field(
                "tsens_xpd_wait",
                &format_args!("{}", self.tsens_xpd_wait().bits()),
            )
            .field(
                "tsens_xpd_force",
                &format_args!("{}", self.tsens_xpd_force().bits()),
            )
            .field(
                "tsens_clk_inv",
                &format_args!("{}", self.tsens_clk_inv().bit()),
            )
            .field(
                "tsens_clk_sel",
                &format_args!("{}", self.tsens_clk_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TSENS_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - the time that power up tsens need wait"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W<0> {
        TSENS_XPD_WAIT_W::new(self)
    }
    #[doc = "Bits 12:13 - force power up tsens"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W<12> {
        TSENS_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 14 - inv tsens clk"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W<14> {
        TSENS_CLK_INV_W::new(self)
    }
    #[doc = "Bit 15 - tsens clk select"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_sel(&mut self) -> TSENS_CLK_SEL_W<15> {
        TSENS_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital tsens configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsens_ctrl2](index.html) module"]
pub struct TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsens_ctrl2::R](R) reader structure"]
impl crate::Readable for TSENS_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsens_ctrl2::W](W) writer structure"]
impl crate::Writable for TSENS_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for TSENS_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x4002;
}
