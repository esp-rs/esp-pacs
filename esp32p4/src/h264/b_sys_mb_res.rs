#[doc = "Register `B_SYS_MB_RES` reader"]
pub type R = crate::R<B_SYS_MB_RES_SPEC>;
#[doc = "Register `B_SYS_MB_RES` writer"]
pub type W = crate::W<B_SYS_MB_RES_SPEC>;
#[doc = "Field `B_SYS_TOTAL_MB_Y` reader - Configures video B vertical MB resolution."]
pub type B_SYS_TOTAL_MB_Y_R = crate::FieldReader;
#[doc = "Field `B_SYS_TOTAL_MB_Y` writer - Configures video B vertical MB resolution."]
pub type B_SYS_TOTAL_MB_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_SYS_TOTAL_MB_X` reader - Configures video B horizontal MB resolution."]
pub type B_SYS_TOTAL_MB_X_R = crate::FieldReader;
#[doc = "Field `B_SYS_TOTAL_MB_X` writer - Configures video B horizontal MB resolution."]
pub type B_SYS_TOTAL_MB_X_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configures video B vertical MB resolution."]
    #[inline(always)]
    pub fn b_sys_total_mb_y(&self) -> B_SYS_TOTAL_MB_Y_R {
        B_SYS_TOTAL_MB_Y_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configures video B horizontal MB resolution."]
    #[inline(always)]
    pub fn b_sys_total_mb_x(&self) -> B_SYS_TOTAL_MB_X_R {
        B_SYS_TOTAL_MB_X_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_SYS_MB_RES")
            .field(
                "b_sys_total_mb_y",
                &format_args!("{}", self.b_sys_total_mb_y().bits()),
            )
            .field(
                "b_sys_total_mb_x",
                &format_args!("{}", self.b_sys_total_mb_x().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<B_SYS_MB_RES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures video B vertical MB resolution."]
    #[inline(always)]
    #[must_use]
    pub fn b_sys_total_mb_y(&mut self) -> B_SYS_TOTAL_MB_Y_W<B_SYS_MB_RES_SPEC> {
        B_SYS_TOTAL_MB_Y_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configures video B horizontal MB resolution."]
    #[inline(always)]
    #[must_use]
    pub fn b_sys_total_mb_x(&mut self) -> B_SYS_TOTAL_MB_X_W<B_SYS_MB_RES_SPEC> {
        B_SYS_TOTAL_MB_X_W::new(self, 7)
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
#[doc = "Video B horizontal and vertical MB resolution register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_sys_mb_res::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_sys_mb_res::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_SYS_MB_RES_SPEC;
impl crate::RegisterSpec for B_SYS_MB_RES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_sys_mb_res::R`](R) reader structure"]
impl crate::Readable for B_SYS_MB_RES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_sys_mb_res::W`](W) writer structure"]
impl crate::Writable for B_SYS_MB_RES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets B_SYS_MB_RES to value 0"]
impl crate::Resettable for B_SYS_MB_RES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
