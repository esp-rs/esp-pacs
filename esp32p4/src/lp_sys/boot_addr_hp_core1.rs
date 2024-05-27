#[doc = "Register `BOOT_ADDR_HP_CORE1` reader"]
pub type R = crate::R<BOOT_ADDR_HP_CORE1_SPEC>;
#[doc = "Register `BOOT_ADDR_HP_CORE1` writer"]
pub type W = crate::W<BOOT_ADDR_HP_CORE1_SPEC>;
#[doc = "Field `BOOT_ADDR_HP_CORE1` reader - need_des"]
pub type BOOT_ADDR_HP_CORE1_R = crate::FieldReader<u32>;
#[doc = "Field `BOOT_ADDR_HP_CORE1` writer - need_des"]
pub type BOOT_ADDR_HP_CORE1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn boot_addr_hp_core1(&self) -> BOOT_ADDR_HP_CORE1_R {
        BOOT_ADDR_HP_CORE1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_ADDR_HP_CORE1")
            .field("boot_addr_hp_core1", &self.boot_addr_hp_core1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn boot_addr_hp_core1(&mut self) -> BOOT_ADDR_HP_CORE1_W<BOOT_ADDR_HP_CORE1_SPEC> {
        BOOT_ADDR_HP_CORE1_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_addr_hp_core1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_addr_hp_core1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_ADDR_HP_CORE1_SPEC;
impl crate::RegisterSpec for BOOT_ADDR_HP_CORE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_addr_hp_core1::R`](R) reader structure"]
impl crate::Readable for BOOT_ADDR_HP_CORE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot_addr_hp_core1::W`](W) writer structure"]
impl crate::Writable for BOOT_ADDR_HP_CORE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_ADDR_HP_CORE1 to value 0"]
impl crate::Resettable for BOOT_ADDR_HP_CORE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
