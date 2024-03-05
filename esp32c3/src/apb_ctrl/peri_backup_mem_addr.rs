#[doc = "Register `PERI_BACKUP_MEM_ADDR` reader"]
pub type R = crate::R<PERI_BACKUP_MEM_ADDR_SPEC>;
#[doc = "Register `PERI_BACKUP_MEM_ADDR` writer"]
pub type W = crate::W<PERI_BACKUP_MEM_ADDR_SPEC>;
#[doc = "Field `BACKUP_MEM_START_ADDR` reader - reg_backup_mem_start_addr"]
pub type BACKUP_MEM_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `BACKUP_MEM_START_ADDR` writer - reg_backup_mem_start_addr"]
pub type BACKUP_MEM_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_backup_mem_start_addr"]
    #[inline(always)]
    pub fn backup_mem_start_addr(&self) -> BACKUP_MEM_START_ADDR_R {
        BACKUP_MEM_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_BACKUP_MEM_ADDR")
            .field(
                "backup_mem_start_addr",
                &format_args!("{}", self.backup_mem_start_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_BACKUP_MEM_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_backup_mem_start_addr"]
    #[inline(always)]
    #[must_use]
    pub fn backup_mem_start_addr(&mut self) -> BACKUP_MEM_START_ADDR_W<PERI_BACKUP_MEM_ADDR_SPEC> {
        BACKUP_MEM_START_ADDR_W::new(self, 0)
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_MEM_ADDR_REG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_mem_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_mem_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_BACKUP_MEM_ADDR_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_MEM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_backup_mem_addr::R`](R) reader structure"]
impl crate::Readable for PERI_BACKUP_MEM_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_backup_mem_addr::W`](W) writer structure"]
impl crate::Writable for PERI_BACKUP_MEM_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_BACKUP_MEM_ADDR to value 0"]
impl crate::Resettable for PERI_BACKUP_MEM_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
