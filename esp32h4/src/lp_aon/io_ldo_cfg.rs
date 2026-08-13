#[doc = "Register `IO_LDO_CFG` reader"]
pub type R = crate::R<IO_LDO_CFG_SPEC>;
#[doc = "Register `IO_LDO_CFG` writer"]
pub type W = crate::W<IO_LDO_CFG_SPEC>;
#[doc = "Field `IO_LDO_3P3_SW` reader - need_des"]
pub type IO_LDO_3P3_SW_R = crate::BitReader;
#[doc = "Field `IO_LDO_3P3_SW` writer - need_des"]
pub type IO_LDO_3P3_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_LDO_3P3_SW_EN` reader - need_des"]
pub type IO_LDO_3P3_SW_EN_R = crate::BitReader;
#[doc = "Field `IO_LDO_3P3_SW_EN` writer - need_des"]
pub type IO_LDO_3P3_SW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_LDO_ADJUST_SW` reader - need_des"]
pub type IO_LDO_ADJUST_SW_R = crate::FieldReader;
#[doc = "Field `IO_LDO_ADJUST_SW` writer - need_des"]
pub type IO_LDO_ADJUST_SW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IO_LDO_ADJUST_SW_EN` reader - need_des"]
pub type IO_LDO_ADJUST_SW_EN_R = crate::BitReader;
#[doc = "Field `IO_LDO_ADJUST_SW_EN` writer - need_des"]
pub type IO_LDO_ADJUST_SW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn io_ldo_3p3_sw(&self) -> IO_LDO_3P3_SW_R {
        IO_LDO_3P3_SW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn io_ldo_3p3_sw_en(&self) -> IO_LDO_3P3_SW_EN_R {
        IO_LDO_3P3_SW_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn io_ldo_adjust_sw(&self) -> IO_LDO_ADJUST_SW_R {
        IO_LDO_ADJUST_SW_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn io_ldo_adjust_sw_en(&self) -> IO_LDO_ADJUST_SW_EN_R {
        IO_LDO_ADJUST_SW_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_LDO_CFG")
            .field("io_ldo_3p3_sw", &self.io_ldo_3p3_sw())
            .field("io_ldo_3p3_sw_en", &self.io_ldo_3p3_sw_en())
            .field("io_ldo_adjust_sw", &self.io_ldo_adjust_sw())
            .field("io_ldo_adjust_sw_en", &self.io_ldo_adjust_sw_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn io_ldo_3p3_sw(&mut self) -> IO_LDO_3P3_SW_W<'_, IO_LDO_CFG_SPEC> {
        IO_LDO_3P3_SW_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn io_ldo_3p3_sw_en(&mut self) -> IO_LDO_3P3_SW_EN_W<'_, IO_LDO_CFG_SPEC> {
        IO_LDO_3P3_SW_EN_W::new(self, 22)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn io_ldo_adjust_sw(&mut self) -> IO_LDO_ADJUST_SW_W<'_, IO_LDO_CFG_SPEC> {
        IO_LDO_ADJUST_SW_W::new(self, 23)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn io_ldo_adjust_sw_en(&mut self) -> IO_LDO_ADJUST_SW_EN_W<'_, IO_LDO_CFG_SPEC> {
        IO_LDO_ADJUST_SW_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ldo_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ldo_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_LDO_CFG_SPEC;
impl crate::RegisterSpec for IO_LDO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ldo_cfg::R`](R) reader structure"]
impl crate::Readable for IO_LDO_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`io_ldo_cfg::W`](W) writer structure"]
impl crate::Writable for IO_LDO_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_LDO_CFG to value 0"]
impl crate::Resettable for IO_LDO_CFG_SPEC {}
