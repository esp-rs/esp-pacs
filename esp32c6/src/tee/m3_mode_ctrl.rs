#[doc = "Register `M3_MODE_CTRL` reader"]
pub type R = crate::R<M3_MODE_CTRL_SPEC>;
#[doc = "Register `M3_MODE_CTRL` writer"]
pub type W = crate::W<M3_MODE_CTRL_SPEC>;
#[doc = "Field `M3_MODE` reader - M3 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M3_MODE_R = crate::FieldReader;
#[doc = "Field `M3_MODE` writer - M3 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M3_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - M3 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    pub fn m3_mode(&self) -> M3_MODE_R {
        M3_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3_MODE_CTRL")
            .field("m3_mode", &format_args!("{}", self.m3_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M3_MODE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - M3 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    #[must_use]
    pub fn m3_mode(&mut self) -> M3_MODE_W<M3_MODE_CTRL_SPEC> {
        M3_MODE_W::new(self, 0)
    }
}
#[doc = "Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3_mode_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3_mode_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3_MODE_CTRL_SPEC;
impl crate::RegisterSpec for M3_MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for M3_MODE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m3_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for M3_MODE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M3_MODE_CTRL to value 0x03"]
impl crate::Resettable for M3_MODE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
