#[doc = "Register `DCACHE_CTRL1` reader"]
pub type R = crate::R<DCACHE_CTRL1_SPEC>;
#[doc = "Register `DCACHE_CTRL1` writer"]
pub type W = crate::W<DCACHE_CTRL1_SPEC>;
#[doc = "Field `DCACHE_SHUT_CORE0_BUS` reader - The bit is used to disable core0 dbus, 0: enable, 1: disable"]
pub type DCACHE_SHUT_CORE0_BUS_R = crate::BitReader;
#[doc = "Field `DCACHE_SHUT_CORE0_BUS` writer - The bit is used to disable core0 dbus, 0: enable, 1: disable"]
pub type DCACHE_SHUT_CORE0_BUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_SHUT_CORE1_BUS` reader - The bit is used to disable core1 dbus, 0: enable, 1: disable"]
pub type DCACHE_SHUT_CORE1_BUS_R = crate::BitReader;
#[doc = "Field `DCACHE_SHUT_CORE1_BUS` writer - The bit is used to disable core1 dbus, 0: enable, 1: disable"]
pub type DCACHE_SHUT_CORE1_BUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn dcache_shut_core0_bus(&self) -> DCACHE_SHUT_CORE0_BUS_R {
        DCACHE_SHUT_CORE0_BUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn dcache_shut_core1_bus(&self) -> DCACHE_SHUT_CORE1_BUS_R {
        DCACHE_SHUT_CORE1_BUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_CTRL1")
            .field("dcache_shut_core0_bus", &self.dcache_shut_core0_bus())
            .field("dcache_shut_core1_bus", &self.dcache_shut_core1_bus())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn dcache_shut_core0_bus(&mut self) -> DCACHE_SHUT_CORE0_BUS_W<DCACHE_CTRL1_SPEC> {
        DCACHE_SHUT_CORE0_BUS_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn dcache_shut_core1_bus(&mut self) -> DCACHE_SHUT_CORE1_BUS_W<DCACHE_CTRL1_SPEC> {
        DCACHE_SHUT_CORE1_BUS_W::new(self, 1)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_CTRL1_SPEC;
impl crate::RegisterSpec for DCACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_ctrl1::R`](R) reader structure"]
impl crate::Readable for DCACHE_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_ctrl1::W`](W) writer structure"]
impl crate::Writable for DCACHE_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_CTRL1 to value 0x03"]
impl crate::Resettable for DCACHE_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
