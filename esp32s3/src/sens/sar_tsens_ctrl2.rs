#[doc = "Register `SAR_TSENS_CTRL2` reader"]
pub type R = crate::R<SAR_TSENS_CTRL2_SPEC>;
#[doc = "Register `SAR_TSENS_CTRL2` writer"]
pub type W = crate::W<SAR_TSENS_CTRL2_SPEC>;
#[doc = "Field `SAR_TSENS_XPD_WAIT` reader - no public"]
pub type SAR_TSENS_XPD_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_TSENS_XPD_WAIT` writer - no public"]
pub type SAR_TSENS_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SAR_TSENS_XPD_FORCE` reader - no public"]
pub type SAR_TSENS_XPD_FORCE_R = crate::FieldReader;
#[doc = "Field `SAR_TSENS_XPD_FORCE` writer - no public"]
pub type SAR_TSENS_XPD_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR_TSENS_CLK_INV` reader - no public"]
pub type SAR_TSENS_CLK_INV_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_CLK_INV` writer - no public"]
pub type SAR_TSENS_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - no public"]
    #[inline(always)]
    pub fn sar_tsens_xpd_wait(&self) -> SAR_TSENS_XPD_WAIT_R {
        SAR_TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - no public"]
    #[inline(always)]
    pub fn sar_tsens_xpd_force(&self) -> SAR_TSENS_XPD_FORCE_R {
        SAR_TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - no public"]
    #[inline(always)]
    pub fn sar_tsens_clk_inv(&self) -> SAR_TSENS_CLK_INV_R {
        SAR_TSENS_CLK_INV_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TSENS_CTRL2")
            .field(
                "sar_tsens_xpd_wait",
                &format_args!("{}", self.sar_tsens_xpd_wait().bits()),
            )
            .field(
                "sar_tsens_xpd_force",
                &format_args!("{}", self.sar_tsens_xpd_force().bits()),
            )
            .field(
                "sar_tsens_clk_inv",
                &format_args!("{}", self.sar_tsens_clk_inv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TSENS_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_xpd_wait(&mut self) -> SAR_TSENS_XPD_WAIT_W<SAR_TSENS_CTRL2_SPEC> {
        SAR_TSENS_XPD_WAIT_W::new(self, 0)
    }
    #[doc = "Bits 12:13 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_xpd_force(&mut self) -> SAR_TSENS_XPD_FORCE_W<SAR_TSENS_CTRL2_SPEC> {
        SAR_TSENS_XPD_FORCE_W::new(self, 12)
    }
    #[doc = "Bit 14 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_clk_inv(&mut self) -> SAR_TSENS_CLK_INV_W<SAR_TSENS_CTRL2_SPEC> {
        SAR_TSENS_CLK_INV_W::new(self, 14)
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
#[doc = "configure tsens controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_tsens_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_tsens_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_tsens_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_tsens_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for SAR_TSENS_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x4002;
}
