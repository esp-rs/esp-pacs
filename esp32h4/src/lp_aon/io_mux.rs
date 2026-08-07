#[doc = "Register `IO_MUX` reader"]
pub type R = crate::R<IO_MUX_SPEC>;
#[doc = "Register `IO_MUX` writer"]
pub type W = crate::W<IO_MUX_SPEC>;
#[doc = "Field `PULL_LDO_EN` reader - need_des"]
pub type PULL_LDO_EN_R = crate::BitReader;
#[doc = "Field `PULL_LDO_EN` writer - need_des"]
pub type PULL_LDO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULL_LDO` reader - need_des"]
pub type PULL_LDO_R = crate::FieldReader;
#[doc = "Field `PULL_LDO` writer - need_des"]
pub type PULL_LDO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESET_DISABLE` reader - bypass hp iomux reset from hp system reset event 1: bypass 0: no operation"]
pub type RESET_DISABLE_R = crate::BitReader;
#[doc = "Field `RESET_DISABLE` writer - bypass hp iomux reset from hp system reset event 1: bypass 0: no operation"]
pub type RESET_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn pull_ldo_en(&self) -> PULL_LDO_EN_R {
        PULL_LDO_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    pub fn pull_ldo(&self) -> PULL_LDO_R {
        PULL_LDO_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - bypass hp iomux reset from hp system reset event 1: bypass 0: no operation"]
    #[inline(always)]
    pub fn reset_disable(&self) -> RESET_DISABLE_R {
        RESET_DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_MUX")
            .field("pull_ldo_en", &self.pull_ldo_en())
            .field("pull_ldo", &self.pull_ldo())
            .field("reset_disable", &self.reset_disable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn pull_ldo_en(&mut self) -> PULL_LDO_EN_W<'_, IO_MUX_SPEC> {
        PULL_LDO_EN_W::new(self, 27)
    }
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    pub fn pull_ldo(&mut self) -> PULL_LDO_W<'_, IO_MUX_SPEC> {
        PULL_LDO_W::new(self, 28)
    }
    #[doc = "Bit 31 - bypass hp iomux reset from hp system reset event 1: bypass 0: no operation"]
    #[inline(always)]
    pub fn reset_disable(&mut self) -> RESET_DISABLE_W<'_, IO_MUX_SPEC> {
        RESET_DISABLE_W::new(self, 31)
    }
}
#[doc = "configure hp iomux reset bypass\n\nYou can [`read`](crate::Reg::read) this register and get [`io_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_MUX_SPEC;
impl crate::RegisterSpec for IO_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_mux::R`](R) reader structure"]
impl crate::Readable for IO_MUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`io_mux::W`](W) writer structure"]
impl crate::Writable for IO_MUX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_MUX to value 0"]
impl crate::Resettable for IO_MUX_SPEC {}
