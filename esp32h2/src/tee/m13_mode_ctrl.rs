#[doc = "Register `M13_MODE_CTRL` reader"]
pub type R = crate::R<M13_MODE_CTRL_SPEC>;
#[doc = "Register `M13_MODE_CTRL` writer"]
pub type W = crate::W<M13_MODE_CTRL_SPEC>;
#[doc = "Field `M13_MODE` reader - M13 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M13_MODE_R = crate::FieldReader;
#[doc = "Field `M13_MODE` writer - M13 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M13_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - M13 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    pub fn m13_mode(&self) -> M13_MODE_R {
        M13_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M13_MODE_CTRL")
            .field("m13_mode", &format_args!("{}", self.m13_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M13_MODE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - M13 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    #[must_use]
    pub fn m13_mode(&mut self) -> M13_MODE_W<M13_MODE_CTRL_SPEC> {
        M13_MODE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m13_mode_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m13_mode_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M13_MODE_CTRL_SPEC;
impl crate::RegisterSpec for M13_MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m13_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for M13_MODE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m13_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for M13_MODE_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M13_MODE_CTRL to value 0x03"]
impl crate::Resettable for M13_MODE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
