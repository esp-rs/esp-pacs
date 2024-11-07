#[doc = "Register `APPCPU_CTRL_D` reader"]
pub type R = crate::R<APPCPU_CTRL_D_SPEC>;
#[doc = "Register `APPCPU_CTRL_D` writer"]
pub type W = crate::W<APPCPU_CTRL_D_SPEC>;
#[doc = "Field `APPCPU_BOOT_ADDR` reader - "]
pub type APPCPU_BOOT_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `APPCPU_BOOT_ADDR` writer - "]
pub type APPCPU_BOOT_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn appcpu_boot_addr(&self) -> APPCPU_BOOT_ADDR_R {
        APPCPU_BOOT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPCPU_CTRL_D")
            .field("appcpu_boot_addr", &self.appcpu_boot_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn appcpu_boot_addr(&mut self) -> APPCPU_BOOT_ADDR_W<APPCPU_CTRL_D_SPEC> {
        APPCPU_BOOT_ADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`appcpu_ctrl_d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appcpu_ctrl_d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPCPU_CTRL_D_SPEC;
impl crate::RegisterSpec for APPCPU_CTRL_D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appcpu_ctrl_d::R`](R) reader structure"]
impl crate::Readable for APPCPU_CTRL_D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`appcpu_ctrl_d::W`](W) writer structure"]
impl crate::Writable for APPCPU_CTRL_D_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPCPU_CTRL_D to value 0"]
impl crate::Resettable for APPCPU_CTRL_D_SPEC {
    const RESET_VALUE: u32 = 0;
}
