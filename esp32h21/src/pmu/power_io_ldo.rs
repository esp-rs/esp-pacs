#[doc = "Register `POWER_IO_LDO` reader"]
pub type R = crate::R<POWER_IO_LDO_SPEC>;
#[doc = "Register `POWER_IO_LDO` writer"]
pub type W = crate::W<POWER_IO_LDO_SPEC>;
#[doc = "Field `IO_LDO_RDY` reader - need_des"]
pub type IO_LDO_RDY_R = crate::BitReader;
#[doc = "Field `IO_SW_EN_XPD` reader - need_des"]
pub type IO_SW_EN_XPD_R = crate::BitReader;
#[doc = "Field `IO_SW_EN_XPD` writer - need_des"]
pub type IO_SW_EN_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_SW_EN_THRU` reader - need_des"]
pub type IO_SW_EN_THRU_R = crate::BitReader;
#[doc = "Field `IO_SW_EN_THRU` writer - need_des"]
pub type IO_SW_EN_THRU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_SW_EN_STANDBY` reader - need_des"]
pub type IO_SW_EN_STANDBY_R = crate::BitReader;
#[doc = "Field `IO_SW_EN_STANDBY` writer - need_des"]
pub type IO_SW_EN_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_SW_EN_POWER_ADJUST` reader - need_des"]
pub type IO_SW_EN_POWER_ADJUST_R = crate::BitReader;
#[doc = "Field `IO_SW_EN_POWER_ADJUST` writer - need_des"]
pub type IO_SW_EN_POWER_ADJUST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_SW_EN_ENDET` reader - need_des"]
pub type IO_SW_EN_ENDET_R = crate::BitReader;
#[doc = "Field `IO_SW_EN_ENDET` writer - need_des"]
pub type IO_SW_EN_ENDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_BYPASS_LDO_RDY` reader - need_des"]
pub type IO_BYPASS_LDO_RDY_R = crate::BitReader;
#[doc = "Field `IO_BYPASS_LDO_RDY` writer - need_des"]
pub type IO_BYPASS_LDO_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_XPD` reader - need_des"]
pub type IO_XPD_R = crate::BitReader;
#[doc = "Field `IO_XPD` writer - need_des"]
pub type IO_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_THRU` reader - need_des"]
pub type IO_THRU_R = crate::BitReader;
#[doc = "Field `IO_THRU` writer - need_des"]
pub type IO_THRU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_STANDBY` reader - need_des"]
pub type IO_STANDBY_R = crate::BitReader;
#[doc = "Field `IO_STANDBY` writer - need_des"]
pub type IO_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_POWER_ADJUST` reader - need_des"]
pub type IO_POWER_ADJUST_R = crate::FieldReader;
#[doc = "Field `IO_POWER_ADJUST` writer - need_des"]
pub type IO_POWER_ADJUST_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO_ENDET` reader - need_des"]
pub type IO_ENDET_R = crate::BitReader;
#[doc = "Field `IO_ENDET` writer - need_des"]
pub type IO_ENDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn io_ldo_rdy(&self) -> IO_LDO_RDY_R {
        IO_LDO_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_xpd(&self) -> IO_SW_EN_XPD_R {
        IO_SW_EN_XPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_thru(&self) -> IO_SW_EN_THRU_R {
        IO_SW_EN_THRU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_standby(&self) -> IO_SW_EN_STANDBY_R {
        IO_SW_EN_STANDBY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_power_adjust(&self) -> IO_SW_EN_POWER_ADJUST_R {
        IO_SW_EN_POWER_ADJUST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_endet(&self) -> IO_SW_EN_ENDET_R {
        IO_SW_EN_ENDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn io_bypass_ldo_rdy(&self) -> IO_BYPASS_LDO_RDY_R {
        IO_BYPASS_LDO_RDY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn io_xpd(&self) -> IO_XPD_R {
        IO_XPD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn io_thru(&self) -> IO_THRU_R {
        IO_THRU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn io_standby(&self) -> IO_STANDBY_R {
        IO_STANDBY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - need_des"]
    #[inline(always)]
    pub fn io_power_adjust(&self) -> IO_POWER_ADJUST_R {
        IO_POWER_ADJUST_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn io_endet(&self) -> IO_ENDET_R {
        IO_ENDET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_IO_LDO")
            .field("io_ldo_rdy", &self.io_ldo_rdy())
            .field("io_sw_en_xpd", &self.io_sw_en_xpd())
            .field("io_sw_en_thru", &self.io_sw_en_thru())
            .field("io_sw_en_standby", &self.io_sw_en_standby())
            .field("io_sw_en_power_adjust", &self.io_sw_en_power_adjust())
            .field("io_sw_en_endet", &self.io_sw_en_endet())
            .field("io_bypass_ldo_rdy", &self.io_bypass_ldo_rdy())
            .field("io_xpd", &self.io_xpd())
            .field("io_thru", &self.io_thru())
            .field("io_standby", &self.io_standby())
            .field("io_power_adjust", &self.io_power_adjust())
            .field("io_endet", &self.io_endet())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_xpd(&mut self) -> IO_SW_EN_XPD_W<'_, POWER_IO_LDO_SPEC> {
        IO_SW_EN_XPD_W::new(self, 1)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_thru(&mut self) -> IO_SW_EN_THRU_W<'_, POWER_IO_LDO_SPEC> {
        IO_SW_EN_THRU_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_standby(&mut self) -> IO_SW_EN_STANDBY_W<'_, POWER_IO_LDO_SPEC> {
        IO_SW_EN_STANDBY_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_power_adjust(&mut self) -> IO_SW_EN_POWER_ADJUST_W<'_, POWER_IO_LDO_SPEC> {
        IO_SW_EN_POWER_ADJUST_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn io_sw_en_endet(&mut self) -> IO_SW_EN_ENDET_W<'_, POWER_IO_LDO_SPEC> {
        IO_SW_EN_ENDET_W::new(self, 6)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn io_bypass_ldo_rdy(&mut self) -> IO_BYPASS_LDO_RDY_W<'_, POWER_IO_LDO_SPEC> {
        IO_BYPASS_LDO_RDY_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn io_xpd(&mut self) -> IO_XPD_W<'_, POWER_IO_LDO_SPEC> {
        IO_XPD_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn io_thru(&mut self) -> IO_THRU_W<'_, POWER_IO_LDO_SPEC> {
        IO_THRU_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn io_standby(&mut self) -> IO_STANDBY_W<'_, POWER_IO_LDO_SPEC> {
        IO_STANDBY_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - need_des"]
    #[inline(always)]
    pub fn io_power_adjust(&mut self) -> IO_POWER_ADJUST_W<'_, POWER_IO_LDO_SPEC> {
        IO_POWER_ADJUST_W::new(self, 26)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn io_endet(&mut self) -> IO_ENDET_W<'_, POWER_IO_LDO_SPEC> {
        IO_ENDET_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_io_ldo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_io_ldo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_IO_LDO_SPEC;
impl crate::RegisterSpec for POWER_IO_LDO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_io_ldo::R`](R) reader structure"]
impl crate::Readable for POWER_IO_LDO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_io_ldo::W`](W) writer structure"]
impl crate::Writable for POWER_IO_LDO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_IO_LDO to value 0x0100_0001"]
impl crate::Resettable for POWER_IO_LDO_SPEC {
    const RESET_VALUE: u32 = 0x0100_0001;
}
