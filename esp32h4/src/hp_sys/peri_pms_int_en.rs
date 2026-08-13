#[doc = "Register `PERI_PMS_INT_EN` reader"]
pub type R = crate::R<PERI_PMS_INT_EN_SPEC>;
#[doc = "Register `PERI_PMS_INT_EN` writer"]
pub type W = crate::W<PERI_PMS_INT_EN_SPEC>;
#[doc = "Field `HP_PERI_PMS_INT_EN` reader - Configures to enable hp peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type HP_PERI_PMS_INT_EN_R = crate::BitReader;
#[doc = "Field `HP_PERI_PMS_INT_EN` writer - Configures to enable hp peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type HP_PERI_PMS_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_PERI_PMS_INT_EN` reader - Configures to enable cpu peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type CPU_PERI_PMS_INT_EN_R = crate::BitReader;
#[doc = "Field `CPU_PERI_PMS_INT_EN` writer - Configures to enable cpu peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type CPU_PERI_PMS_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_PERI_PMS_INT_EN` reader - Configures to enable modem peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type MODEM_PERI_PMS_INT_EN_R = crate::BitReader;
#[doc = "Field `MODEM_PERI_PMS_INT_EN` writer - Configures to enable modem peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type MODEM_PERI_PMS_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures to enable hp peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn hp_peri_pms_int_en(&self) -> HP_PERI_PMS_INT_EN_R {
        HP_PERI_PMS_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures to enable cpu peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn cpu_peri_pms_int_en(&self) -> CPU_PERI_PMS_INT_EN_R {
        CPU_PERI_PMS_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures to enable modem peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn modem_peri_pms_int_en(&self) -> MODEM_PERI_PMS_INT_EN_R {
        MODEM_PERI_PMS_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_PMS_INT_EN")
            .field("hp_peri_pms_int_en", &self.hp_peri_pms_int_en())
            .field("cpu_peri_pms_int_en", &self.cpu_peri_pms_int_en())
            .field("modem_peri_pms_int_en", &self.modem_peri_pms_int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures to enable hp peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn hp_peri_pms_int_en(&mut self) -> HP_PERI_PMS_INT_EN_W<'_, PERI_PMS_INT_EN_SPEC> {
        HP_PERI_PMS_INT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures to enable cpu peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn cpu_peri_pms_int_en(&mut self) -> CPU_PERI_PMS_INT_EN_W<'_, PERI_PMS_INT_EN_SPEC> {
        CPU_PERI_PMS_INT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures to enable modem peri pms interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn modem_peri_pms_int_en(&mut self) -> MODEM_PERI_PMS_INT_EN_W<'_, PERI_PMS_INT_EN_SPEC> {
        MODEM_PERI_PMS_INT_EN_W::new(self, 2)
    }
}
#[doc = "APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_pms_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_pms_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_PMS_INT_EN_SPEC;
impl crate::RegisterSpec for PERI_PMS_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_pms_int_en::R`](R) reader structure"]
impl crate::Readable for PERI_PMS_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_pms_int_en::W`](W) writer structure"]
impl crate::Writable for PERI_PMS_INT_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_PMS_INT_EN to value 0"]
impl crate::Resettable for PERI_PMS_INT_EN_SPEC {}
