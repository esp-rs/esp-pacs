#[doc = "Register `PRO_DCACHE_CTRL1` reader"]
pub type R = crate::R<PRO_DCACHE_CTRL1_SPEC>;
#[doc = "Register `PRO_DCACHE_CTRL1` writer"]
pub type W = crate::W<PRO_DCACHE_CTRL1_SPEC>;
#[doc = "Field `PRO_DCACHE_MASK_BUS0` reader - The bit is used to disable dbus0, 0: enable, 1: disable"]
pub type PRO_DCACHE_MASK_BUS0_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_MASK_BUS0` writer - The bit is used to disable dbus0, 0: enable, 1: disable"]
pub type PRO_DCACHE_MASK_BUS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DCACHE_MASK_BUS1` reader - The bit is used to disable dbus1, 0: enable, 1: disable"]
pub type PRO_DCACHE_MASK_BUS1_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_MASK_BUS1` writer - The bit is used to disable dbus1, 0: enable, 1: disable"]
pub type PRO_DCACHE_MASK_BUS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DCACHE_MASK_BUS2` reader - The bit is used to disable dbus2, 0: enable, 1: disable"]
pub type PRO_DCACHE_MASK_BUS2_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_MASK_BUS2` writer - The bit is used to disable dbus2, 0: enable, 1: disable"]
pub type PRO_DCACHE_MASK_BUS2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable dbus0, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_dcache_mask_bus0(&self) -> PRO_DCACHE_MASK_BUS0_R {
        PRO_DCACHE_MASK_BUS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable dbus1, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_dcache_mask_bus1(&self) -> PRO_DCACHE_MASK_BUS1_R {
        PRO_DCACHE_MASK_BUS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to disable dbus2, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_dcache_mask_bus2(&self) -> PRO_DCACHE_MASK_BUS2_R {
        PRO_DCACHE_MASK_BUS2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_CTRL1")
            .field("pro_dcache_mask_bus0", &self.pro_dcache_mask_bus0().bit())
            .field("pro_dcache_mask_bus1", &self.pro_dcache_mask_bus1().bit())
            .field("pro_dcache_mask_bus2", &self.pro_dcache_mask_bus2().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable dbus0, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_mask_bus0(&mut self) -> PRO_DCACHE_MASK_BUS0_W<PRO_DCACHE_CTRL1_SPEC> {
        PRO_DCACHE_MASK_BUS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to disable dbus1, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_mask_bus1(&mut self) -> PRO_DCACHE_MASK_BUS1_W<PRO_DCACHE_CTRL1_SPEC> {
        PRO_DCACHE_MASK_BUS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to disable dbus2, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_mask_bus2(&mut self) -> PRO_DCACHE_MASK_BUS2_W<PRO_DCACHE_CTRL1_SPEC> {
        PRO_DCACHE_MASK_BUS2_W::new(self, 2)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_CTRL1_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_ctrl1::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_ctrl1::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_CTRL1 to value 0x07"]
impl crate::Resettable for PRO_DCACHE_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
