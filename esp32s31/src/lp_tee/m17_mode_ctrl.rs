#[doc = "Register `M17_MODE_CTRL` reader"]
pub type R = crate::R<M17_MODE_CTRL_SPEC>;
#[doc = "Register `M17_MODE_CTRL` writer"]
pub type W = crate::W<M17_MODE_CTRL_SPEC>;
#[doc = "Field `M17_MODE` reader - Configures M0 security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
pub type M17_MODE_R = crate::FieldReader;
#[doc = "Field `M17_MODE` writer - Configures M0 security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
pub type M17_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M17_LOCK` reader - Set 1 to lock m17 tee configuration"]
pub type M17_LOCK_R = crate::BitReader;
#[doc = "Field `M17_LOCK` writer - Set 1 to lock m17 tee configuration"]
pub type M17_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures M0 security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
    #[inline(always)]
    pub fn m17_mode(&self) -> M17_MODE_R {
        M17_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set 1 to lock m17 tee configuration"]
    #[inline(always)]
    pub fn m17_lock(&self) -> M17_LOCK_R {
        M17_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M17_MODE_CTRL")
            .field("m17_mode", &self.m17_mode())
            .field("m17_lock", &self.m17_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures M0 security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
    #[inline(always)]
    pub fn m17_mode(&mut self) -> M17_MODE_W<'_, M17_MODE_CTRL_SPEC> {
        M17_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set 1 to lock m17 tee configuration"]
    #[inline(always)]
    pub fn m17_lock(&mut self) -> M17_LOCK_W<'_, M17_MODE_CTRL_SPEC> {
        M17_LOCK_W::new(self, 2)
    }
}
#[doc = "TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m17_mode_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m17_mode_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M17_MODE_CTRL_SPEC;
impl crate::RegisterSpec for M17_MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m17_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for M17_MODE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m17_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for M17_MODE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M17_MODE_CTRL to value 0x03"]
impl crate::Resettable for M17_MODE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
