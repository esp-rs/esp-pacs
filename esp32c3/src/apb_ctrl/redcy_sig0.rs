#[doc = "Register `REDCY_SIG0` reader"]
pub type R = crate::R<REDCY_SIG0_SPEC>;
#[doc = "Register `REDCY_SIG0` writer"]
pub type W = crate::W<REDCY_SIG0_SPEC>;
#[doc = "Field `REDCY_SIG0` reader - reg_redcy_sig0"]
pub type REDCY_SIG0_R = crate::FieldReader<u32>;
#[doc = "Field `REDCY_SIG0` writer - reg_redcy_sig0"]
pub type REDCY_SIG0_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `REDCY_ANDOR` reader - reg_redcy_andor"]
pub type REDCY_ANDOR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - reg_redcy_sig0"]
    #[inline(always)]
    pub fn redcy_sig0(&self) -> REDCY_SIG0_R {
        REDCY_SIG0_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - reg_redcy_andor"]
    #[inline(always)]
    pub fn redcy_andor(&self) -> REDCY_ANDOR_R {
        REDCY_ANDOR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDCY_SIG0")
            .field("redcy_sig0", &format_args!("{}", self.redcy_sig0().bits()))
            .field("redcy_andor", &format_args!("{}", self.redcy_andor().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REDCY_SIG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:30 - reg_redcy_sig0"]
    #[inline(always)]
    #[must_use]
    pub fn redcy_sig0(&mut self) -> REDCY_SIG0_W<REDCY_SIG0_SPEC> {
        REDCY_SIG0_W::new(self, 0)
    }
}
#[doc = "APB_CTRL_REDCY_SIG0_REG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redcy_sig0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redcy_sig0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REDCY_SIG0_SPEC;
impl crate::RegisterSpec for REDCY_SIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redcy_sig0::R`](R) reader structure"]
impl crate::Readable for REDCY_SIG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`redcy_sig0::W`](W) writer structure"]
impl crate::Writable for REDCY_SIG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REDCY_SIG0 to value 0"]
impl crate::Resettable for REDCY_SIG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
