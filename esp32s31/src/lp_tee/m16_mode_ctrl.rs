#[doc = "Register `M16_MODE_CTRL` reader"]
pub type R = crate::R<M16_MODE_CTRL_SPEC>;
#[doc = "Register `M16_MODE_CTRL` writer"]
pub type W = crate::W<M16_MODE_CTRL_SPEC>;
#[doc = "Field `M16_MODE` reader - Configures M0 security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
pub type M16_MODE_R = crate::FieldReader;
#[doc = "Field `M16_MODE` writer - Configures M0 security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
pub type M16_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M16_LOCK` reader - Set 1 to lock m16 tee configuration"]
pub type M16_LOCK_R = crate::BitReader;
#[doc = "Field `M16_LOCK` writer - Set 1 to lock m16 tee configuration"]
pub type M16_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures M0 security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
    #[inline(always)]
    pub fn m16_mode(&self) -> M16_MODE_R {
        M16_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set 1 to lock m16 tee configuration"]
    #[inline(always)]
    pub fn m16_lock(&self) -> M16_LOCK_R {
        M16_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M16_MODE_CTRL")
            .field("m16_mode", &self.m16_mode())
            .field("m16_lock", &self.m16_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures M0 security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
    #[inline(always)]
    pub fn m16_mode(&mut self) -> M16_MODE_W<'_, M16_MODE_CTRL_SPEC> {
        M16_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set 1 to lock m16 tee configuration"]
    #[inline(always)]
    pub fn m16_lock(&mut self) -> M16_LOCK_W<'_, M16_MODE_CTRL_SPEC> {
        M16_LOCK_W::new(self, 2)
    }
}
#[doc = "TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m16_mode_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m16_mode_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M16_MODE_CTRL_SPEC;
impl crate::RegisterSpec for M16_MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m16_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for M16_MODE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m16_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for M16_MODE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M16_MODE_CTRL to value 0x03"]
impl crate::Resettable for M16_MODE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
