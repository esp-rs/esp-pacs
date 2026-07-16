#[doc = "Register `SYS_VER_DATE` reader"]
pub type R = crate::R<SYS_VER_DATE_SPEC>;
#[doc = "Register `SYS_VER_DATE` writer"]
pub type W = crate::W<SYS_VER_DATE_SPEC>;
#[doc = "Field `SYS_VER_DATE` reader - "]
pub type SYS_VER_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `SYS_VER_DATE` writer - "]
pub type SYS_VER_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_ver_date(&self) -> SYS_VER_DATE_R {
        SYS_VER_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_VER_DATE")
            .field("sys_ver_date", &self.sys_ver_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_ver_date(&mut self) -> SYS_VER_DATE_W<'_, SYS_VER_DATE_SPEC> {
        SYS_VER_DATE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_ver_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_ver_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_VER_DATE_SPEC;
impl crate::RegisterSpec for SYS_VER_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ver_date::R`](R) reader structure"]
impl crate::Readable for SYS_VER_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_ver_date::W`](W) writer structure"]
impl crate::Writable for SYS_VER_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_VER_DATE to value 0x2025_0926"]
impl crate::Resettable for SYS_VER_DATE_SPEC {
    const RESET_VALUE: u32 = 0x2025_0926;
}
