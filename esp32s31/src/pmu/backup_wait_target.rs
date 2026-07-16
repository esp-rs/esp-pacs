#[doc = "Register `BACKUP_WAIT_TARGET` reader"]
pub type R = crate::R<BACKUP_WAIT_TARGET_SPEC>;
#[doc = "Register `BACKUP_WAIT_TARGET` writer"]
pub type W = crate::W<BACKUP_WAIT_TARGET_SPEC>;
#[doc = "Field `BACKUP_WAIT_TARGET` reader - need_des"]
pub type BACKUP_WAIT_TARGET_R = crate::FieldReader;
#[doc = "Field `BACKUP_WAIT_TARGET` writer - need_des"]
pub type BACKUP_WAIT_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn backup_wait_target(&self) -> BACKUP_WAIT_TARGET_R {
        BACKUP_WAIT_TARGET_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_WAIT_TARGET")
            .field("backup_wait_target", &self.backup_wait_target())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn backup_wait_target(&mut self) -> BACKUP_WAIT_TARGET_W<'_, BACKUP_WAIT_TARGET_SPEC> {
        BACKUP_WAIT_TARGET_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_wait_target::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_wait_target::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_WAIT_TARGET_SPEC;
impl crate::RegisterSpec for BACKUP_WAIT_TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_wait_target::R`](R) reader structure"]
impl crate::Readable for BACKUP_WAIT_TARGET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_wait_target::W`](W) writer structure"]
impl crate::Writable for BACKUP_WAIT_TARGET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BACKUP_WAIT_TARGET to value 0x0f"]
impl crate::Resettable for BACKUP_WAIT_TARGET_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
