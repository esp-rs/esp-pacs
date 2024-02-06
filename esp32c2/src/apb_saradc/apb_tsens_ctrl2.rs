#[doc = "Register `APB_TSENS_CTRL2` reader"]
pub type R = crate::R<APB_TSENS_CTRL2_SPEC>;
#[doc = "Register `APB_TSENS_CTRL2` writer"]
pub type W = crate::W<APB_TSENS_CTRL2_SPEC>;
#[doc = "Field `REG_TSENS_XPD_WAIT` reader - Need add description"]
pub type REG_TSENS_XPD_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `REG_TSENS_XPD_WAIT` writer - Need add description"]
pub type REG_TSENS_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `REG_TSENS_XPD_FORCE` reader - Need add description"]
pub type REG_TSENS_XPD_FORCE_R = crate::FieldReader;
#[doc = "Field `REG_TSENS_XPD_FORCE` writer - Need add description"]
pub type REG_TSENS_XPD_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_TSENS_CLK_INV` reader - Need add description"]
pub type REG_TSENS_CLK_INV_R = crate::BitReader;
#[doc = "Field `REG_TSENS_CLK_INV` writer - Need add description"]
pub type REG_TSENS_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_CLK_SEL` reader - Need add description"]
pub type TSENS_CLK_SEL_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_SEL` writer - Need add description"]
pub type TSENS_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TSENS_CTRL2")
            .field(
                "reg_tsens_xpd_wait",
                &format_args!("{}", self.reg_tsens_xpd_wait().bits()),
            )
            .field(
                "reg_tsens_xpd_force",
                &format_args!("{}", self.reg_tsens_xpd_force().bits()),
            )
            .field(
                "reg_tsens_clk_inv",
                &format_args!("{}", self.reg_tsens_clk_inv().bit()),
            )
            .field(
                "tsens_clk_sel",
                &format_args!("{}", self.tsens_clk_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_TSENS_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tsens_xpd_wait(&mut self) -> REG_TSENS_XPD_WAIT_W<APB_TSENS_CTRL2_SPEC> {
        REG_TSENS_XPD_WAIT_W::new(self, 0)
    }
    #[doc = "Bits 12:13 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tsens_xpd_force(&mut self) -> REG_TSENS_XPD_FORCE_W<APB_TSENS_CTRL2_SPEC> {
        REG_TSENS_XPD_FORCE_W::new(self, 12)
    }
    #[doc = "Bit 14 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tsens_clk_inv(&mut self) -> REG_TSENS_CLK_INV_W<APB_TSENS_CTRL2_SPEC> {
        REG_TSENS_CLK_INV_W::new(self, 14)
    }
    #[doc = "Bit 15 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_sel(&mut self) -> TSENS_CLK_SEL_W<APB_TSENS_CTRL2_SPEC> {
        TSENS_CLK_SEL_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_tsens_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_tsens_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for APB_TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_tsens_ctrl2::R`](R) reader structure"]
impl crate::Readable for APB_TSENS_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_tsens_ctrl2::W`](W) writer structure"]
impl crate::Writable for APB_TSENS_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for APB_TSENS_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x4002;
}
