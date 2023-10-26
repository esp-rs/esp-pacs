#[doc = "Register `SAR_TSENS_CTRL2` reader"]
pub type R = crate::R<SAR_TSENS_CTRL2_SPEC>;
#[doc = "Register `SAR_TSENS_CTRL2` writer"]
pub type W = crate::W<SAR_TSENS_CTRL2_SPEC>;
#[doc = "Field `TSENS_XPD_WAIT` reader - "]
pub type TSENS_XPD_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `TSENS_XPD_WAIT` writer - "]
pub type TSENS_XPD_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `TSENS_XPD_FORCE` reader - "]
pub type TSENS_XPD_FORCE_R = crate::FieldReader;
#[doc = "Field `TSENS_XPD_FORCE` writer - "]
pub type TSENS_XPD_FORCE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TSENS_CLK_INV` reader - "]
pub type TSENS_CLK_INV_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_INV` writer - "]
pub type TSENS_CLK_INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSENS_CLKGATE_EN` reader - Enable temperature sensor clock."]
pub type TSENS_CLKGATE_EN_R = crate::BitReader;
#[doc = "Field `TSENS_CLKGATE_EN` writer - Enable temperature sensor clock."]
pub type TSENS_CLKGATE_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSENS_RESET` reader - Reset temperature sensor."]
pub type TSENS_RESET_R = crate::BitReader;
#[doc = "Field `TSENS_RESET` writer - Reset temperature sensor."]
pub type TSENS_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable temperature sensor clock."]
    #[inline(always)]
    pub fn tsens_clkgate_en(&self) -> TSENS_CLKGATE_EN_R {
        TSENS_CLKGATE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset temperature sensor."]
    #[inline(always)]
    pub fn tsens_reset(&self) -> TSENS_RESET_R {
        TSENS_RESET_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TSENS_CTRL2")
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
                "tsens_clkgate_en",
                &format_args!("{}", self.tsens_clkgate_en().bit()),
            )
            .field("tsens_reset", &format_args!("{}", self.tsens_reset().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TSENS_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W<SAR_TSENS_CTRL2_SPEC, 0> {
        TSENS_XPD_WAIT_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W<SAR_TSENS_CTRL2_SPEC, 12> {
        TSENS_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W<SAR_TSENS_CTRL2_SPEC, 14> {
        TSENS_CLK_INV_W::new(self)
    }
    #[doc = "Bit 15 - Enable temperature sensor clock."]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clkgate_en(&mut self) -> TSENS_CLKGATE_EN_W<SAR_TSENS_CTRL2_SPEC, 15> {
        TSENS_CLKGATE_EN_W::new(self)
    }
    #[doc = "Bit 16 - Reset temperature sensor."]
    #[inline(always)]
    #[must_use]
    pub fn tsens_reset(&mut self) -> TSENS_RESET_W<SAR_TSENS_CTRL2_SPEC, 16> {
        TSENS_RESET_W::new(self)
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
#[doc = "Temperature sensor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_tsens_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_tsens_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_tsens_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_tsens_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for SAR_TSENS_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x4002;
}
