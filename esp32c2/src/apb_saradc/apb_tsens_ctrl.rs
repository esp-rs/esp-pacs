#[doc = "Register `APB_TSENS_CTRL` reader"]
pub struct R(crate::R<APB_TSENS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_TSENS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_TSENS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_TSENS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_TSENS_CTRL` writer"]
pub struct W(crate::W<APB_TSENS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_TSENS_CTRL_SPEC>;
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
impl From<crate::W<APB_TSENS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_TSENS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_TSENS_OUT` reader - Need add description"]
pub type REG_TSENS_OUT_R = crate::FieldReader;
#[doc = "Field `REG_TSENS_IN_INV` reader - Need add description"]
pub type REG_TSENS_IN_INV_R = crate::BitReader;
#[doc = "Field `REG_TSENS_IN_INV` writer - Need add description"]
pub type REG_TSENS_IN_INV_W<'a, const O: u8> = crate::BitWriter<'a, APB_TSENS_CTRL_SPEC, O>;
#[doc = "Field `REG_TSENS_CLK_DIV` reader - Need add description"]
pub type REG_TSENS_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `REG_TSENS_CLK_DIV` writer - Need add description"]
pub type REG_TSENS_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, APB_TSENS_CTRL_SPEC, 8, O>;
#[doc = "Field `REG_TSENS_PU` reader - Need add description"]
pub type REG_TSENS_PU_R = crate::BitReader;
#[doc = "Field `REG_TSENS_PU` writer - Need add description"]
pub type REG_TSENS_PU_W<'a, const O: u8> = crate::BitWriter<'a, APB_TSENS_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_out(&self) -> REG_TSENS_OUT_R {
        REG_TSENS_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_in_inv(&self) -> REG_TSENS_IN_INV_R {
        REG_TSENS_IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_clk_div(&self) -> REG_TSENS_CLK_DIV_R {
        REG_TSENS_CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_pu(&self) -> REG_TSENS_PU_R {
        REG_TSENS_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TSENS_CTRL")
            .field(
                "reg_tsens_out",
                &format_args!("{}", self.reg_tsens_out().bits()),
            )
            .field(
                "reg_tsens_in_inv",
                &format_args!("{}", self.reg_tsens_in_inv().bit()),
            )
            .field(
                "reg_tsens_clk_div",
                &format_args!("{}", self.reg_tsens_clk_div().bits()),
            )
            .field(
                "reg_tsens_pu",
                &format_args!("{}", self.reg_tsens_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_TSENS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 13 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tsens_in_inv(&mut self) -> REG_TSENS_IN_INV_W<13> {
        REG_TSENS_IN_INV_W::new(self)
    }
    #[doc = "Bits 14:21 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tsens_clk_div(&mut self) -> REG_TSENS_CLK_DIV_W<14> {
        REG_TSENS_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 22 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tsens_pu(&mut self) -> REG_TSENS_PU_W<22> {
        REG_TSENS_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_tsens_ctrl](index.html) module"]
pub struct APB_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for APB_TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_tsens_ctrl::R](R) reader structure"]
impl crate::Readable for APB_TSENS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_tsens_ctrl::W](W) writer structure"]
impl crate::Writable for APB_TSENS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_TSENS_CTRL to value 0x0001_8000"]
impl crate::Resettable for APB_TSENS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_8000;
}
