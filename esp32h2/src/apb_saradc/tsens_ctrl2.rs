#[doc = "Register `TSENS_CTRL2` reader"]
pub type R = crate::R<TSENS_CTRL2_SPEC>;
#[doc = "Register `TSENS_CTRL2` writer"]
pub type W = crate::W<TSENS_CTRL2_SPEC>;
#[doc = "Field `TSENS_XPD_WAIT` reader - the time that power up tsens need wait"]
pub type TSENS_XPD_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `TSENS_XPD_WAIT` writer - the time that power up tsens need wait"]
pub type TSENS_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TSENS_XPD_FORCE` reader - force power up tsens"]
pub type TSENS_XPD_FORCE_R = crate::FieldReader;
#[doc = "Field `TSENS_XPD_FORCE` writer - force power up tsens"]
pub type TSENS_XPD_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENS_CLK_INV` reader - inv tsens clk"]
pub type TSENS_CLK_INV_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_INV` writer - inv tsens clk"]
pub type TSENS_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_CLK_SEL` reader - tsens clk select"]
pub type TSENS_CLK_SEL_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_SEL` writer - tsens clk select"]
pub type TSENS_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - the time that power up tsens need wait"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W<TSENS_CTRL2_SPEC> {
        TSENS_XPD_WAIT_W::new(self, 0)
    }
    #[doc = "Bits 12:13 - force power up tsens"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W<TSENS_CTRL2_SPEC> {
        TSENS_XPD_FORCE_W::new(self, 12)
    }
    #[doc = "Bit 14 - inv tsens clk"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W<TSENS_CTRL2_SPEC> {
        TSENS_CLK_INV_W::new(self, 14)
    }
    #[doc = "Bit 15 - tsens clk select"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_sel(&mut self) -> TSENS_CLK_SEL_W<TSENS_CTRL2_SPEC> {
        TSENS_CLK_SEL_W::new(self, 15)
    }
}
#[doc = "digital tsens configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsens_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsens_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsens_ctrl2::R`](R) reader structure"]
impl crate::Readable for TSENS_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsens_ctrl2::W`](W) writer structure"]
impl crate::Writable for TSENS_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for TSENS_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x4002;
}
