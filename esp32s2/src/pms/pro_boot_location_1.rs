#[doc = "Register `PRO_BOOT_LOCATION_1` reader"]
pub type R = crate::R<PRO_BOOT_LOCATION_1_SPEC>;
#[doc = "Register `PRO_BOOT_LOCATION_1` writer"]
pub type W = crate::W<PRO_BOOT_LOCATION_1_SPEC>;
#[doc = "Field `PRO_BOOT_REMAP` reader - If set to 1, enable boot remap function."]
pub type PRO_BOOT_REMAP_R = crate::BitReader;
#[doc = "Field `PRO_BOOT_REMAP` writer - If set to 1, enable boot remap function."]
pub type PRO_BOOT_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set to 1, enable boot remap function."]
    #[inline(always)]
    pub fn pro_boot_remap(&self) -> PRO_BOOT_REMAP_R {
        PRO_BOOT_REMAP_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_BOOT_LOCATION_1")
            .field("pro_boot_remap", &self.pro_boot_remap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, enable boot remap function."]
    #[inline(always)]
    pub fn pro_boot_remap(&mut self) -> PRO_BOOT_REMAP_W<PRO_BOOT_LOCATION_1_SPEC> {
        PRO_BOOT_REMAP_W::new(self, 0)
    }
}
#[doc = "Boot permission control register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_boot_location_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_boot_location_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_BOOT_LOCATION_1_SPEC;
impl crate::RegisterSpec for PRO_BOOT_LOCATION_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_boot_location_1::R`](R) reader structure"]
impl crate::Readable for PRO_BOOT_LOCATION_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_boot_location_1::W`](W) writer structure"]
impl crate::Writable for PRO_BOOT_LOCATION_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_BOOT_LOCATION_1 to value 0"]
impl crate::Resettable for PRO_BOOT_LOCATION_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
