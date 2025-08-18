#[doc = "Register `B_RC_CONF0` reader"]
pub type R = crate::R<B_RC_CONF0_SPEC>;
#[doc = "Register `B_RC_CONF0` writer"]
pub type W = crate::W<B_RC_CONF0_SPEC>;
#[doc = "Field `B_QP` reader - Configures video B frame level initial luma QP value."]
pub type B_QP_R = crate::FieldReader;
#[doc = "Field `B_QP` writer - Configures video B frame level initial luma QP value."]
pub type B_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `B_RATE_CTRL_U` reader - Configures video B parameter U value. U = int((float) u << 8)."]
pub type B_RATE_CTRL_U_R = crate::FieldReader<u16>;
#[doc = "Field `B_RATE_CTRL_U` writer - Configures video B parameter U value. U = int((float) u << 8)."]
pub type B_RATE_CTRL_U_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `B_MB_RATE_CTRL_EN` reader - Configures video A whether or not to open macro block rate ctrl.\\\\1:Open the macro block rate ctrl\\\\1:Close the macro block rate ctrl."]
pub type B_MB_RATE_CTRL_EN_R = crate::BitReader;
#[doc = "Field `B_MB_RATE_CTRL_EN` writer - Configures video A whether or not to open macro block rate ctrl.\\\\1:Open the macro block rate ctrl\\\\1:Close the macro block rate ctrl."]
pub type B_MB_RATE_CTRL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures video B frame level initial luma QP value."]
    #[inline(always)]
    pub fn b_qp(&self) -> B_QP_R {
        B_QP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:21 - Configures video B parameter U value. U = int((float) u << 8)."]
    #[inline(always)]
    pub fn b_rate_ctrl_u(&self) -> B_RATE_CTRL_U_R {
        B_RATE_CTRL_U_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22 - Configures video A whether or not to open macro block rate ctrl.\\\\1:Open the macro block rate ctrl\\\\1:Close the macro block rate ctrl."]
    #[inline(always)]
    pub fn b_mb_rate_ctrl_en(&self) -> B_MB_RATE_CTRL_EN_R {
        B_MB_RATE_CTRL_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_RC_CONF0")
            .field("b_qp", &self.b_qp())
            .field("b_rate_ctrl_u", &self.b_rate_ctrl_u())
            .field("b_mb_rate_ctrl_en", &self.b_mb_rate_ctrl_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures video B frame level initial luma QP value."]
    #[inline(always)]
    pub fn b_qp(&mut self) -> B_QP_W<'_, B_RC_CONF0_SPEC> {
        B_QP_W::new(self, 0)
    }
    #[doc = "Bits 6:21 - Configures video B parameter U value. U = int((float) u << 8)."]
    #[inline(always)]
    pub fn b_rate_ctrl_u(&mut self) -> B_RATE_CTRL_U_W<'_, B_RC_CONF0_SPEC> {
        B_RATE_CTRL_U_W::new(self, 6)
    }
    #[doc = "Bit 22 - Configures video A whether or not to open macro block rate ctrl.\\\\1:Open the macro block rate ctrl\\\\1:Close the macro block rate ctrl."]
    #[inline(always)]
    pub fn b_mb_rate_ctrl_en(&mut self) -> B_MB_RATE_CTRL_EN_W<'_, B_RC_CONF0_SPEC> {
        B_MB_RATE_CTRL_EN_W::new(self, 22)
    }
}
#[doc = "Video B rate control configuration register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_rc_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_rc_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_RC_CONF0_SPEC;
impl crate::RegisterSpec for B_RC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_rc_conf0::R`](R) reader structure"]
impl crate::Readable for B_RC_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_rc_conf0::W`](W) writer structure"]
impl crate::Writable for B_RC_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_RC_CONF0 to value 0"]
impl crate::Resettable for B_RC_CONF0_SPEC {}
