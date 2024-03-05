#[doc = "Register `A_RC_CONF0` reader"]
pub type R = crate::R<A_RC_CONF0_SPEC>;
#[doc = "Register `A_RC_CONF0` writer"]
pub type W = crate::W<A_RC_CONF0_SPEC>;
#[doc = "Field `A_QP` reader - Configures video A frame level initial luma QP value."]
pub type A_QP_R = crate::FieldReader;
#[doc = "Field `A_QP` writer - Configures video A frame level initial luma QP value."]
pub type A_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_RATE_CTRL_U` reader - Configures video A parameter U value. U = int((float) u &lt;&lt; 8)."]
pub type A_RATE_CTRL_U_R = crate::FieldReader<u16>;
#[doc = "Field `A_RATE_CTRL_U` writer - Configures video A parameter U value. U = int((float) u &lt;&lt; 8)."]
pub type A_RATE_CTRL_U_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `A_MB_RATE_CTRL_EN` reader - Configures video A whether or not to open macro block rate ctrl.\\\\1:Open the macro block rate ctrl\\\\1:Close the macro block rate ctrl."]
pub type A_MB_RATE_CTRL_EN_R = crate::BitReader;
#[doc = "Field `A_MB_RATE_CTRL_EN` writer - Configures video A whether or not to open macro block rate ctrl.\\\\1:Open the macro block rate ctrl\\\\1:Close the macro block rate ctrl."]
pub type A_MB_RATE_CTRL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures video A frame level initial luma QP value."]
    #[inline(always)]
    pub fn a_qp(&self) -> A_QP_R {
        A_QP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:21 - Configures video A parameter U value. U = int((float) u &lt;&lt; 8)."]
    #[inline(always)]
    pub fn a_rate_ctrl_u(&self) -> A_RATE_CTRL_U_R {
        A_RATE_CTRL_U_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22 - Configures video A whether or not to open macro block rate ctrl.\\\\1:Open the macro block rate ctrl\\\\1:Close the macro block rate ctrl."]
    #[inline(always)]
    pub fn a_mb_rate_ctrl_en(&self) -> A_MB_RATE_CTRL_EN_R {
        A_MB_RATE_CTRL_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_RC_CONF0")
            .field("a_qp", &format_args!("{}", self.a_qp().bits()))
            .field(
                "a_rate_ctrl_u",
                &format_args!("{}", self.a_rate_ctrl_u().bits()),
            )
            .field(
                "a_mb_rate_ctrl_en",
                &format_args!("{}", self.a_mb_rate_ctrl_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<A_RC_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures video A frame level initial luma QP value."]
    #[inline(always)]
    #[must_use]
    pub fn a_qp(&mut self) -> A_QP_W<A_RC_CONF0_SPEC> {
        A_QP_W::new(self, 0)
    }
    #[doc = "Bits 6:21 - Configures video A parameter U value. U = int((float) u &lt;&lt; 8)."]
    #[inline(always)]
    #[must_use]
    pub fn a_rate_ctrl_u(&mut self) -> A_RATE_CTRL_U_W<A_RC_CONF0_SPEC> {
        A_RATE_CTRL_U_W::new(self, 6)
    }
    #[doc = "Bit 22 - Configures video A whether or not to open macro block rate ctrl.\\\\1:Open the macro block rate ctrl\\\\1:Close the macro block rate ctrl."]
    #[inline(always)]
    #[must_use]
    pub fn a_mb_rate_ctrl_en(&mut self) -> A_MB_RATE_CTRL_EN_W<A_RC_CONF0_SPEC> {
        A_MB_RATE_CTRL_EN_W::new(self, 22)
    }
}
#[doc = "Video A rate control configuration register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_rc_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_rc_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_RC_CONF0_SPEC;
impl crate::RegisterSpec for A_RC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_rc_conf0::R`](R) reader structure"]
impl crate::Readable for A_RC_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_rc_conf0::W`](W) writer structure"]
impl crate::Writable for A_RC_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A_RC_CONF0 to value 0"]
impl crate::Resettable for A_RC_CONF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
