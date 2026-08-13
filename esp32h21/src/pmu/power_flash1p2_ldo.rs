#[doc = "Register `POWER_FLASH1P2_LDO` reader"]
pub type R = crate::R<POWER_FLASH1P2_LDO_SPEC>;
#[doc = "Register `POWER_FLASH1P2_LDO` writer"]
pub type W = crate::W<POWER_FLASH1P2_LDO_SPEC>;
#[doc = "Field `FLASH1P2_LDO_RDY` reader - need_des"]
pub type FLASH1P2_LDO_RDY_R = crate::BitReader;
#[doc = "Field `FLASH1P2_SW_EN_XPD` reader - need_des"]
pub type FLASH1P2_SW_EN_XPD_R = crate::BitReader;
#[doc = "Field `FLASH1P2_SW_EN_XPD` writer - need_des"]
pub type FLASH1P2_SW_EN_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_SW_EN_THRU` reader - need_des"]
pub type FLASH1P2_SW_EN_THRU_R = crate::BitReader;
#[doc = "Field `FLASH1P2_SW_EN_THRU` writer - need_des"]
pub type FLASH1P2_SW_EN_THRU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_SW_EN_STANDBY` reader - need_des"]
pub type FLASH1P2_SW_EN_STANDBY_R = crate::BitReader;
#[doc = "Field `FLASH1P2_SW_EN_STANDBY` writer - need_des"]
pub type FLASH1P2_SW_EN_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_SW_EN_POWER_ADJUST` reader - need_des"]
pub type FLASH1P2_SW_EN_POWER_ADJUST_R = crate::BitReader;
#[doc = "Field `FLASH1P2_SW_EN_POWER_ADJUST` writer - need_des"]
pub type FLASH1P2_SW_EN_POWER_ADJUST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_SW_EN_ENDET` reader - need_des"]
pub type FLASH1P2_SW_EN_ENDET_R = crate::BitReader;
#[doc = "Field `FLASH1P2_SW_EN_ENDET` writer - need_des"]
pub type FLASH1P2_SW_EN_ENDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_BYPASS_LDO_RDY` reader - need_des"]
pub type FLASH1P2_BYPASS_LDO_RDY_R = crate::BitReader;
#[doc = "Field `FLASH1P2_BYPASS_LDO_RDY` writer - need_des"]
pub type FLASH1P2_BYPASS_LDO_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_XPD` reader - need_des"]
pub type FLASH1P2_XPD_R = crate::BitReader;
#[doc = "Field `FLASH1P2_XPD` writer - need_des"]
pub type FLASH1P2_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_THRU` reader - need_des"]
pub type FLASH1P2_THRU_R = crate::BitReader;
#[doc = "Field `FLASH1P2_THRU` writer - need_des"]
pub type FLASH1P2_THRU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_STANDBY` reader - need_des"]
pub type FLASH1P2_STANDBY_R = crate::BitReader;
#[doc = "Field `FLASH1P2_STANDBY` writer - need_des"]
pub type FLASH1P2_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH1P2_POWER_ADJUST` reader - need_des"]
pub type FLASH1P2_POWER_ADJUST_R = crate::FieldReader;
#[doc = "Field `FLASH1P2_POWER_ADJUST` writer - need_des"]
pub type FLASH1P2_POWER_ADJUST_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FLASH1P2_ENDET` reader - need_des"]
pub type FLASH1P2_ENDET_R = crate::BitReader;
#[doc = "Field `FLASH1P2_ENDET` writer - need_des"]
pub type FLASH1P2_ENDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn flash1p2_ldo_rdy(&self) -> FLASH1P2_LDO_RDY_R {
        FLASH1P2_LDO_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_xpd(&self) -> FLASH1P2_SW_EN_XPD_R {
        FLASH1P2_SW_EN_XPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_thru(&self) -> FLASH1P2_SW_EN_THRU_R {
        FLASH1P2_SW_EN_THRU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_standby(&self) -> FLASH1P2_SW_EN_STANDBY_R {
        FLASH1P2_SW_EN_STANDBY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_power_adjust(&self) -> FLASH1P2_SW_EN_POWER_ADJUST_R {
        FLASH1P2_SW_EN_POWER_ADJUST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_endet(&self) -> FLASH1P2_SW_EN_ENDET_R {
        FLASH1P2_SW_EN_ENDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn flash1p2_bypass_ldo_rdy(&self) -> FLASH1P2_BYPASS_LDO_RDY_R {
        FLASH1P2_BYPASS_LDO_RDY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn flash1p2_xpd(&self) -> FLASH1P2_XPD_R {
        FLASH1P2_XPD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn flash1p2_thru(&self) -> FLASH1P2_THRU_R {
        FLASH1P2_THRU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn flash1p2_standby(&self) -> FLASH1P2_STANDBY_R {
        FLASH1P2_STANDBY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - need_des"]
    #[inline(always)]
    pub fn flash1p2_power_adjust(&self) -> FLASH1P2_POWER_ADJUST_R {
        FLASH1P2_POWER_ADJUST_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn flash1p2_endet(&self) -> FLASH1P2_ENDET_R {
        FLASH1P2_ENDET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_FLASH1P2_LDO")
            .field("flash1p2_ldo_rdy", &self.flash1p2_ldo_rdy())
            .field("flash1p2_sw_en_xpd", &self.flash1p2_sw_en_xpd())
            .field("flash1p2_sw_en_thru", &self.flash1p2_sw_en_thru())
            .field("flash1p2_sw_en_standby", &self.flash1p2_sw_en_standby())
            .field(
                "flash1p2_sw_en_power_adjust",
                &self.flash1p2_sw_en_power_adjust(),
            )
            .field("flash1p2_sw_en_endet", &self.flash1p2_sw_en_endet())
            .field("flash1p2_bypass_ldo_rdy", &self.flash1p2_bypass_ldo_rdy())
            .field("flash1p2_xpd", &self.flash1p2_xpd())
            .field("flash1p2_thru", &self.flash1p2_thru())
            .field("flash1p2_standby", &self.flash1p2_standby())
            .field("flash1p2_power_adjust", &self.flash1p2_power_adjust())
            .field("flash1p2_endet", &self.flash1p2_endet())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_xpd(&mut self) -> FLASH1P2_SW_EN_XPD_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_SW_EN_XPD_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_thru(&mut self) -> FLASH1P2_SW_EN_THRU_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_SW_EN_THRU_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_standby(
        &mut self,
    ) -> FLASH1P2_SW_EN_STANDBY_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_SW_EN_STANDBY_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_power_adjust(
        &mut self,
    ) -> FLASH1P2_SW_EN_POWER_ADJUST_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_SW_EN_POWER_ADJUST_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn flash1p2_sw_en_endet(&mut self) -> FLASH1P2_SW_EN_ENDET_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_SW_EN_ENDET_W::new(self, 5)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn flash1p2_bypass_ldo_rdy(
        &mut self,
    ) -> FLASH1P2_BYPASS_LDO_RDY_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_BYPASS_LDO_RDY_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn flash1p2_xpd(&mut self) -> FLASH1P2_XPD_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_XPD_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn flash1p2_thru(&mut self) -> FLASH1P2_THRU_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_THRU_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn flash1p2_standby(&mut self) -> FLASH1P2_STANDBY_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_STANDBY_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - need_des"]
    #[inline(always)]
    pub fn flash1p2_power_adjust(
        &mut self,
    ) -> FLASH1P2_POWER_ADJUST_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_POWER_ADJUST_W::new(self, 26)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn flash1p2_endet(&mut self) -> FLASH1P2_ENDET_W<'_, POWER_FLASH1P2_LDO_SPEC> {
        FLASH1P2_ENDET_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_flash1p2_ldo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_flash1p2_ldo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_FLASH1P2_LDO_SPEC;
impl crate::RegisterSpec for POWER_FLASH1P2_LDO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_flash1p2_ldo::R`](R) reader structure"]
impl crate::Readable for POWER_FLASH1P2_LDO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_flash1p2_ldo::W`](W) writer structure"]
impl crate::Writable for POWER_FLASH1P2_LDO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_FLASH1P2_LDO to value 0x0100_0001"]
impl crate::Resettable for POWER_FLASH1P2_LDO_SPEC {
    const RESET_VALUE: u32 = 0x0100_0001;
}
