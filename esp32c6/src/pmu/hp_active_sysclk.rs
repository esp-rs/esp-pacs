#[doc = "Register `HP_ACTIVE_SYSCLK` reader"]
pub struct R(crate::R<HP_ACTIVE_SYSCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_ACTIVE_SYSCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_ACTIVE_SYSCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_ACTIVE_SYSCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_ACTIVE_SYSCLK` writer"]
pub struct W(crate::W<HP_ACTIVE_SYSCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_ACTIVE_SYSCLK_SPEC>;
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
impl From<crate::W<HP_ACTIVE_SYSCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_ACTIVE_SYSCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_ACTIVE_DIG_SYS_CLK_NO_DIV` reader - need_des"]
pub type HP_ACTIVE_DIG_SYS_CLK_NO_DIV_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_DIG_SYS_CLK_NO_DIV` writer - need_des"]
pub type HP_ACTIVE_DIG_SYS_CLK_NO_DIV_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_ACTIVE_SYSCLK_SPEC, O>;
#[doc = "Field `HP_ACTIVE_ICG_SYS_CLOCK_EN` reader - need_des"]
pub type HP_ACTIVE_ICG_SYS_CLOCK_EN_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_ICG_SYS_CLOCK_EN` writer - need_des"]
pub type HP_ACTIVE_ICG_SYS_CLOCK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_ACTIVE_SYSCLK_SPEC, O>;
#[doc = "Field `HP_ACTIVE_SYS_CLK_SLP_SEL` reader - need_des"]
pub type HP_ACTIVE_SYS_CLK_SLP_SEL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_SYS_CLK_SLP_SEL` writer - need_des"]
pub type HP_ACTIVE_SYS_CLK_SLP_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_ACTIVE_SYSCLK_SPEC, O>;
#[doc = "Field `HP_ACTIVE_ICG_SLP_SEL` reader - need_des"]
pub type HP_ACTIVE_ICG_SLP_SEL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_ICG_SLP_SEL` writer - need_des"]
pub type HP_ACTIVE_ICG_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, HP_ACTIVE_SYSCLK_SPEC, O>;
#[doc = "Field `HP_ACTIVE_DIG_SYS_CLK_SEL` reader - need_des"]
pub type HP_ACTIVE_DIG_SYS_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_DIG_SYS_CLK_SEL` writer - need_des"]
pub type HP_ACTIVE_DIG_SYS_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_ACTIVE_SYSCLK_SPEC, 2, O>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_active_dig_sys_clk_no_div(&self) -> HP_ACTIVE_DIG_SYS_CLK_NO_DIV_R {
        HP_ACTIVE_DIG_SYS_CLK_NO_DIV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_active_icg_sys_clock_en(&self) -> HP_ACTIVE_ICG_SYS_CLOCK_EN_R {
        HP_ACTIVE_ICG_SYS_CLOCK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn hp_active_sys_clk_slp_sel(&self) -> HP_ACTIVE_SYS_CLK_SLP_SEL_R {
        HP_ACTIVE_SYS_CLK_SLP_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_active_icg_slp_sel(&self) -> HP_ACTIVE_ICG_SLP_SEL_R {
        HP_ACTIVE_ICG_SLP_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn hp_active_dig_sys_clk_sel(&self) -> HP_ACTIVE_DIG_SYS_CLK_SEL_R {
        HP_ACTIVE_DIG_SYS_CLK_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_SYSCLK")
            .field(
                "hp_active_dig_sys_clk_no_div",
                &format_args!("{}", self.hp_active_dig_sys_clk_no_div().bit()),
            )
            .field(
                "hp_active_icg_sys_clock_en",
                &format_args!("{}", self.hp_active_icg_sys_clock_en().bit()),
            )
            .field(
                "hp_active_sys_clk_slp_sel",
                &format_args!("{}", self.hp_active_sys_clk_slp_sel().bit()),
            )
            .field(
                "hp_active_icg_slp_sel",
                &format_args!("{}", self.hp_active_icg_slp_sel().bit()),
            )
            .field(
                "hp_active_dig_sys_clk_sel",
                &format_args!("{}", self.hp_active_dig_sys_clk_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_ACTIVE_SYSCLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_dig_sys_clk_no_div(&mut self) -> HP_ACTIVE_DIG_SYS_CLK_NO_DIV_W<26> {
        HP_ACTIVE_DIG_SYS_CLK_NO_DIV_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_icg_sys_clock_en(&mut self) -> HP_ACTIVE_ICG_SYS_CLOCK_EN_W<27> {
        HP_ACTIVE_ICG_SYS_CLOCK_EN_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_sys_clk_slp_sel(&mut self) -> HP_ACTIVE_SYS_CLK_SLP_SEL_W<28> {
        HP_ACTIVE_SYS_CLK_SLP_SEL_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_icg_slp_sel(&mut self) -> HP_ACTIVE_ICG_SLP_SEL_W<29> {
        HP_ACTIVE_ICG_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_dig_sys_clk_sel(&mut self) -> HP_ACTIVE_DIG_SYS_CLK_SEL_W<30> {
        HP_ACTIVE_DIG_SYS_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_active_sysclk](index.html) module"]
pub struct HP_ACTIVE_SYSCLK_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_SYSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_active_sysclk::R](R) reader structure"]
impl crate::Readable for HP_ACTIVE_SYSCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_active_sysclk::W](W) writer structure"]
impl crate::Writable for HP_ACTIVE_SYSCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_SYSCLK to value 0"]
impl crate::Resettable for HP_ACTIVE_SYSCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
