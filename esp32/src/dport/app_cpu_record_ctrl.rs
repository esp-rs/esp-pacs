#[doc = "Register `APP_CPU_RECORD_CTRL` reader"]
pub type R = crate::R<APP_CPU_RECORD_CTRL_SPEC>;
#[doc = "Register `APP_CPU_RECORD_CTRL` writer"]
pub type W = crate::W<APP_CPU_RECORD_CTRL_SPEC>;
#[doc = "Field `APP_CPU_RECORD_ENABLE` reader - "]
pub type APP_CPU_RECORD_ENABLE_R = crate::BitReader;
#[doc = "Field `APP_CPU_RECORD_ENABLE` writer - "]
pub type APP_CPU_RECORD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CPU_RECORD_DISABLE` reader - "]
pub type APP_CPU_RECORD_DISABLE_R = crate::BitReader;
#[doc = "Field `APP_CPU_RECORD_DISABLE` writer - "]
pub type APP_CPU_RECORD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CPU_PDEBUG_ENABLE` reader - "]
pub type APP_CPU_PDEBUG_ENABLE_R = crate::BitReader;
#[doc = "Field `APP_CPU_PDEBUG_ENABLE` writer - "]
pub type APP_CPU_PDEBUG_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_record_enable(&self) -> APP_CPU_RECORD_ENABLE_R {
        APP_CPU_RECORD_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cpu_record_disable(&self) -> APP_CPU_RECORD_DISABLE_R {
        APP_CPU_RECORD_DISABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cpu_pdebug_enable(&self) -> APP_CPU_PDEBUG_ENABLE_R {
        APP_CPU_PDEBUG_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CPU_RECORD_CTRL")
            .field("app_cpu_record_enable", &self.app_cpu_record_enable())
            .field("app_cpu_record_disable", &self.app_cpu_record_disable())
            .field("app_cpu_pdebug_enable", &self.app_cpu_pdebug_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_record_enable(&mut self) -> APP_CPU_RECORD_ENABLE_W<APP_CPU_RECORD_CTRL_SPEC> {
        APP_CPU_RECORD_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cpu_record_disable(&mut self) -> APP_CPU_RECORD_DISABLE_W<APP_CPU_RECORD_CTRL_SPEC> {
        APP_CPU_RECORD_DISABLE_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cpu_pdebug_enable(&mut self) -> APP_CPU_PDEBUG_ENABLE_W<APP_CPU_RECORD_CTRL_SPEC> {
        APP_CPU_PDEBUG_ENABLE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cpu_record_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cpu_record_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_CPU_RECORD_CTRL_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cpu_record_ctrl::R`](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_cpu_record_ctrl::W`](W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CPU_RECORD_CTRL to value 0x0100"]
impl crate::Resettable for APP_CPU_RECORD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
