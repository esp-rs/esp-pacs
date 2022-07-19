#[doc = "Register `APB_TSENS_CTRL2` reader"]
pub struct R(crate::R<APB_TSENS_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_TSENS_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_TSENS_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_TSENS_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_TSENS_CTRL2` writer"]
pub struct W(crate::W<APB_TSENS_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_TSENS_CTRL2_SPEC>;
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
impl From<crate::W<APB_TSENS_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_TSENS_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_TSENS_XPD_WAIT` reader - Need add description"]
pub type REG_TSENS_XPD_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG_TSENS_XPD_WAIT` writer - Need add description"]
pub type REG_TSENS_XPD_WAIT_W<'a> =
    crate::FieldWriter<'a, u32, APB_TSENS_CTRL2_SPEC, u16, u16, 12, 0>;
#[doc = "Field `REG_TSENS_XPD_FORCE` reader - Need add description"]
pub type REG_TSENS_XPD_FORCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REG_TSENS_XPD_FORCE` writer - Need add description"]
pub type REG_TSENS_XPD_FORCE_W<'a> =
    crate::FieldWriter<'a, u32, APB_TSENS_CTRL2_SPEC, u8, u8, 2, 12>;
#[doc = "Field `REG_TSENS_CLK_INV` reader - Need add description"]
pub type REG_TSENS_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `REG_TSENS_CLK_INV` writer - Need add description"]
pub type REG_TSENS_CLK_INV_W<'a> = crate::BitWriter<'a, u32, APB_TSENS_CTRL2_SPEC, bool, 14>;
#[doc = "Field `TSENS_CLK_SEL` reader - Need add description"]
pub type TSENS_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_CLK_SEL` writer - Need add description"]
pub type TSENS_CLK_SEL_W<'a> = crate::BitWriter<'a, u32, APB_TSENS_CTRL2_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:11 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_xpd_wait(&self) -> REG_TSENS_XPD_WAIT_R {
        REG_TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_xpd_force(&self) -> REG_TSENS_XPD_FORCE_R {
        REG_TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_clk_inv(&self) -> REG_TSENS_CLK_INV_R {
        REG_TSENS_CLK_INV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Need add description"]
    #[inline(always)]
    pub fn tsens_clk_sel(&self) -> TSENS_CLK_SEL_R {
        TSENS_CLK_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_xpd_wait(&mut self) -> REG_TSENS_XPD_WAIT_W {
        REG_TSENS_XPD_WAIT_W::new(self)
    }
    #[doc = "Bits 12:13 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_xpd_force(&mut self) -> REG_TSENS_XPD_FORCE_W {
        REG_TSENS_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 14 - Need add description"]
    #[inline(always)]
    pub fn reg_tsens_clk_inv(&mut self) -> REG_TSENS_CLK_INV_W {
        REG_TSENS_CLK_INV_W::new(self)
    }
    #[doc = "Bit 15 - Need add description"]
    #[inline(always)]
    pub fn tsens_clk_sel(&mut self) -> TSENS_CLK_SEL_W {
        TSENS_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_tsens_ctrl2](index.html) module"]
pub struct APB_TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for APB_TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_tsens_ctrl2::R](R) reader structure"]
impl crate::Readable for APB_TSENS_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_tsens_ctrl2::W](W) writer structure"]
impl crate::Writable for APB_TSENS_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for APB_TSENS_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4002
    }
}
