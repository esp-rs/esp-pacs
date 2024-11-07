#[doc = "Register `APP_BOOT_REMAP_CTRL` reader"]
pub type R = crate::R<APP_BOOT_REMAP_CTRL_SPEC>;
#[doc = "Register `APP_BOOT_REMAP_CTRL` writer"]
pub type W = crate::W<APP_BOOT_REMAP_CTRL_SPEC>;
#[doc = "Field `APP_BOOT_REMAP` reader - "]
pub type APP_BOOT_REMAP_R = crate::BitReader;
#[doc = "Field `APP_BOOT_REMAP` writer - "]
pub type APP_BOOT_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_boot_remap(&self) -> APP_BOOT_REMAP_R {
        APP_BOOT_REMAP_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_BOOT_REMAP_CTRL")
            .field("app_boot_remap", &self.app_boot_remap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_boot_remap(&mut self) -> APP_BOOT_REMAP_W<APP_BOOT_REMAP_CTRL_SPEC> {
        APP_BOOT_REMAP_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_boot_remap_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_boot_remap_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_BOOT_REMAP_CTRL_SPEC;
impl crate::RegisterSpec for APP_BOOT_REMAP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_boot_remap_ctrl::R`](R) reader structure"]
impl crate::Readable for APP_BOOT_REMAP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_boot_remap_ctrl::W`](W) writer structure"]
impl crate::Writable for APP_BOOT_REMAP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_BOOT_REMAP_CTRL to value 0"]
impl crate::Resettable for APP_BOOT_REMAP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
