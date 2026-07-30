#[doc = "Register `POWER_VDD_IO` reader"]
pub type R = crate::R<POWER_VDD_IO_SPEC>;
#[doc = "Register `POWER_VDD_IO` writer"]
pub type W = crate::W<POWER_VDD_IO_SPEC>;
#[doc = "Field `IO_LDO_POWER_SEL` reader - need_des"]
pub type IO_LDO_POWER_SEL_R = crate::BitReader;
#[doc = "Field `IO_LDO_POWER_SEL` writer - need_des"]
pub type IO_LDO_POWER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_LDO_SW_EN_POWER_SEL` reader - need_des"]
pub type IO_LDO_SW_EN_POWER_SEL_R = crate::BitReader;
#[doc = "Field `IO_LDO_SW_EN_POWER_SEL` writer - need_des"]
pub type IO_LDO_SW_EN_POWER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn io_ldo_power_sel(&self) -> IO_LDO_POWER_SEL_R {
        IO_LDO_POWER_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn io_ldo_sw_en_power_sel(&self) -> IO_LDO_SW_EN_POWER_SEL_R {
        IO_LDO_SW_EN_POWER_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_VDD_IO")
            .field("io_ldo_power_sel", &self.io_ldo_power_sel())
            .field("io_ldo_sw_en_power_sel", &self.io_ldo_sw_en_power_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn io_ldo_power_sel(&mut self) -> IO_LDO_POWER_SEL_W<'_, POWER_VDD_IO_SPEC> {
        IO_LDO_POWER_SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn io_ldo_sw_en_power_sel(&mut self) -> IO_LDO_SW_EN_POWER_SEL_W<'_, POWER_VDD_IO_SPEC> {
        IO_LDO_SW_EN_POWER_SEL_W::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_vdd_io::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_vdd_io::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_VDD_IO_SPEC;
impl crate::RegisterSpec for POWER_VDD_IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_vdd_io::R`](R) reader structure"]
impl crate::Readable for POWER_VDD_IO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_vdd_io::W`](W) writer structure"]
impl crate::Writable for POWER_VDD_IO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_VDD_IO to value 0"]
impl crate::Resettable for POWER_VDD_IO_SPEC {}
