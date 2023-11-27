#[doc = "Register `PMUP_DRV_CFG` reader"]
pub type R = crate::R<PMUP_DRV_CFG_SPEC>;
#[doc = "Register `PMUP_DRV_CFG` writer"]
pub type W = crate::W<PMUP_DRV_CFG_SPEC>;
#[doc = "Field `PUMP_EN` reader - configure pvt charge xpd"]
pub type PUMP_EN_R = crate::BitReader;
#[doc = "Field `PUMP_EN` writer - configure pvt charge xpd"]
pub type PUMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - force register clken"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - force register clken"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUMP_DRV4` reader - configure cmd4 drv"]
pub type PUMP_DRV4_R = crate::FieldReader;
#[doc = "Field `PUMP_DRV4` writer - configure cmd4 drv"]
pub type PUMP_DRV4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DRV3` reader - configure cmd3 drv"]
pub type PUMP_DRV3_R = crate::FieldReader;
#[doc = "Field `PUMP_DRV3` writer - configure cmd3 drv"]
pub type PUMP_DRV3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DRV2` reader - configure cmd2 drv"]
pub type PUMP_DRV2_R = crate::FieldReader;
#[doc = "Field `PUMP_DRV2` writer - configure cmd2 drv"]
pub type PUMP_DRV2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DRV1` reader - configure cmd1 drv"]
pub type PUMP_DRV1_R = crate::FieldReader;
#[doc = "Field `PUMP_DRV1` writer - configure cmd1 drv"]
pub type PUMP_DRV1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DRV0` reader - configure cmd0 drv"]
pub type PUMP_DRV0_R = crate::FieldReader;
#[doc = "Field `PUMP_DRV0` writer - configure cmd0 drv"]
pub type PUMP_DRV0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 9 - configure pvt charge xpd"]
    #[inline(always)]
    pub fn pump_en(&self) -> PUMP_EN_R {
        PUMP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - force register clken"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - configure cmd4 drv"]
    #[inline(always)]
    pub fn pump_drv4(&self) -> PUMP_DRV4_R {
        PUMP_DRV4_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - configure cmd3 drv"]
    #[inline(always)]
    pub fn pump_drv3(&self) -> PUMP_DRV3_R {
        PUMP_DRV3_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - configure cmd2 drv"]
    #[inline(always)]
    pub fn pump_drv2(&self) -> PUMP_DRV2_R {
        PUMP_DRV2_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - configure cmd1 drv"]
    #[inline(always)]
    pub fn pump_drv1(&self) -> PUMP_DRV1_R {
        PUMP_DRV1_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30 - configure cmd0 drv"]
    #[inline(always)]
    pub fn pump_drv0(&self) -> PUMP_DRV0_R {
        PUMP_DRV0_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMUP_DRV_CFG")
            .field("pump_en", &format_args!("{}", self.pump_en().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field("pump_drv4", &format_args!("{}", self.pump_drv4().bits()))
            .field("pump_drv3", &format_args!("{}", self.pump_drv3().bits()))
            .field("pump_drv2", &format_args!("{}", self.pump_drv2().bits()))
            .field("pump_drv1", &format_args!("{}", self.pump_drv1().bits()))
            .field("pump_drv0", &format_args!("{}", self.pump_drv0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PMUP_DRV_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 9 - configure pvt charge xpd"]
    #[inline(always)]
    #[must_use]
    pub fn pump_en(&mut self) -> PUMP_EN_W<PMUP_DRV_CFG_SPEC> {
        PUMP_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - force register clken"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<PMUP_DRV_CFG_SPEC> {
        CLK_EN_W::new(self, 10)
    }
    #[doc = "Bits 11:14 - configure cmd4 drv"]
    #[inline(always)]
    #[must_use]
    pub fn pump_drv4(&mut self) -> PUMP_DRV4_W<PMUP_DRV_CFG_SPEC> {
        PUMP_DRV4_W::new(self, 11)
    }
    #[doc = "Bits 15:18 - configure cmd3 drv"]
    #[inline(always)]
    #[must_use]
    pub fn pump_drv3(&mut self) -> PUMP_DRV3_W<PMUP_DRV_CFG_SPEC> {
        PUMP_DRV3_W::new(self, 15)
    }
    #[doc = "Bits 19:22 - configure cmd2 drv"]
    #[inline(always)]
    #[must_use]
    pub fn pump_drv2(&mut self) -> PUMP_DRV2_W<PMUP_DRV_CFG_SPEC> {
        PUMP_DRV2_W::new(self, 19)
    }
    #[doc = "Bits 23:26 - configure cmd1 drv"]
    #[inline(always)]
    #[must_use]
    pub fn pump_drv1(&mut self) -> PUMP_DRV1_W<PMUP_DRV_CFG_SPEC> {
        PUMP_DRV1_W::new(self, 23)
    }
    #[doc = "Bits 27:30 - configure cmd0 drv"]
    #[inline(always)]
    #[must_use]
    pub fn pump_drv0(&mut self) -> PUMP_DRV0_W<PMUP_DRV_CFG_SPEC> {
        PUMP_DRV0_W::new(self, 27)
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
#[doc = "configure pump drv\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_drv_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_drv_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUP_DRV_CFG_SPEC;
impl crate::RegisterSpec for PMUP_DRV_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_drv_cfg::R`](R) reader structure"]
impl crate::Readable for PMUP_DRV_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmup_drv_cfg::W`](W) writer structure"]
impl crate::Writable for PMUP_DRV_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMUP_DRV_CFG to value 0"]
impl crate::Resettable for PMUP_DRV_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
