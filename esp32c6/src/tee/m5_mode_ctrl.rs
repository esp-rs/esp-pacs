#[doc = "Register `M5_MODE_CTRL` reader"]
pub type R = crate::R<M5_MODE_CTRL_SPEC>;
#[doc = "Register `M5_MODE_CTRL` writer"]
pub type W = crate::W<M5_MODE_CTRL_SPEC>;
#[doc = "Field `M5_MODE` reader - M5 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M5_MODE_R = crate::FieldReader;
#[doc = "Field `M5_MODE` writer - M5 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M5_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - M5 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    pub fn m5_mode(&self) -> M5_MODE_R {
        M5_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5_MODE_CTRL")
            .field("m5_mode", &format_args!("{}", self.m5_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M5_MODE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - M5 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    #[must_use]
    pub fn m5_mode(&mut self) -> M5_MODE_W<M5_MODE_CTRL_SPEC, 0> {
        M5_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tee mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5_mode_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5_mode_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5_MODE_CTRL_SPEC;
impl crate::RegisterSpec for M5_MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for M5_MODE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m5_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for M5_MODE_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M5_MODE_CTRL to value 0x03"]
impl crate::Resettable for M5_MODE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
