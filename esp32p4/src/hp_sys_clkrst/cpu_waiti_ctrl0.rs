#[doc = "Register `CPU_WAITI_CTRL0` reader"]
pub type R = crate::R<CPU_WAITI_CTRL0_SPEC>;
#[doc = "Register `CPU_WAITI_CTRL0` writer"]
pub type W = crate::W<CPU_WAITI_CTRL0_SPEC>;
#[doc = "Field `CORE0_WAITI_ICG_EN` reader - Configures whether cpu core0 waiti signal can control clock gate. If both core0 and core1 waiti_icg_en is 1, then only when core0 and core1 all in waiti will close related clock"]
pub type CORE0_WAITI_ICG_EN_R = crate::BitReader;
#[doc = "Field `CORE0_WAITI_ICG_EN` writer - Configures whether cpu core0 waiti signal can control clock gate. If both core0 and core1 waiti_icg_en is 1, then only when core0 and core1 all in waiti will close related clock"]
pub type CORE0_WAITI_ICG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_WAITI_ICG_EN` reader - Configures whether cpu core1 waiti signal can control clock gate. If both core0 and core1 waiti_icg_en is 1, then only when core0 and core1 all in waiti will close related clock"]
pub type CORE1_WAITI_ICG_EN_R = crate::BitReader;
#[doc = "Field `CORE1_WAITI_ICG_EN` writer - Configures whether cpu core1 waiti signal can control clock gate. If both core0 and core1 waiti_icg_en is 1, then only when core0 and core1 all in waiti will close related clock"]
pub type CORE1_WAITI_ICG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether cpu core0 waiti signal can control clock gate. If both core0 and core1 waiti_icg_en is 1, then only when core0 and core1 all in waiti will close related clock"]
    #[inline(always)]
    pub fn core0_waiti_icg_en(&self) -> CORE0_WAITI_ICG_EN_R {
        CORE0_WAITI_ICG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether cpu core1 waiti signal can control clock gate. If both core0 and core1 waiti_icg_en is 1, then only when core0 and core1 all in waiti will close related clock"]
    #[inline(always)]
    pub fn core1_waiti_icg_en(&self) -> CORE1_WAITI_ICG_EN_R {
        CORE1_WAITI_ICG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WAITI_CTRL0")
            .field("core0_waiti_icg_en", &self.core0_waiti_icg_en())
            .field("core1_waiti_icg_en", &self.core1_waiti_icg_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether cpu core0 waiti signal can control clock gate. If both core0 and core1 waiti_icg_en is 1, then only when core0 and core1 all in waiti will close related clock"]
    #[inline(always)]
    pub fn core0_waiti_icg_en(&mut self) -> CORE0_WAITI_ICG_EN_W<'_, CPU_WAITI_CTRL0_SPEC> {
        CORE0_WAITI_ICG_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether cpu core1 waiti signal can control clock gate. If both core0 and core1 waiti_icg_en is 1, then only when core0 and core1 all in waiti will close related clock"]
    #[inline(always)]
    pub fn core1_waiti_icg_en(&mut self) -> CORE1_WAITI_ICG_EN_W<'_, CPU_WAITI_CTRL0_SPEC> {
        CORE1_WAITI_ICG_EN_W::new(self, 1)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_waiti_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_waiti_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_WAITI_CTRL0_SPEC;
impl crate::RegisterSpec for CPU_WAITI_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_waiti_ctrl0::R`](R) reader structure"]
impl crate::Readable for CPU_WAITI_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_waiti_ctrl0::W`](W) writer structure"]
impl crate::Writable for CPU_WAITI_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_WAITI_CTRL0 to value 0x03"]
impl crate::Resettable for CPU_WAITI_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
