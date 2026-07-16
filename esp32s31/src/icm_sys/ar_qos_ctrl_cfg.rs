#[doc = "Register `AR_QOS_CTRL_CFG%s` reader"]
pub type R = crate::R<AR_QOS_CTRL_CFG_SPEC>;
#[doc = "Register `AR_QOS_CTRL_CFG%s` writer"]
pub type W = crate::W<AR_QOS_CTRL_CFG_SPEC>;
#[doc = "Field `REG_AR_QOS_CTRL_CFG_0` reader - "]
pub type REG_AR_QOS_CTRL_CFG_0_R = crate::FieldReader<u32>;
#[doc = "Field `REG_AR_QOS_CTRL_CFG_0` writer - "]
pub type REG_AR_QOS_CTRL_CFG_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_ar_qos_ctrl_cfg_0(&self) -> REG_AR_QOS_CTRL_CFG_0_R {
        REG_AR_QOS_CTRL_CFG_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AR_QOS_CTRL_CFG")
            .field("reg_ar_qos_ctrl_cfg_0", &self.reg_ar_qos_ctrl_cfg_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_ar_qos_ctrl_cfg_0(&mut self) -> REG_AR_QOS_CTRL_CFG_0_W<'_, AR_QOS_CTRL_CFG_SPEC> {
        REG_AR_QOS_CTRL_CFG_0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_ctrl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_ctrl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AR_QOS_CTRL_CFG_SPEC;
impl crate::RegisterSpec for AR_QOS_CTRL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar_qos_ctrl_cfg::R`](R) reader structure"]
impl crate::Readable for AR_QOS_CTRL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ar_qos_ctrl_cfg::W`](W) writer structure"]
impl crate::Writable for AR_QOS_CTRL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AR_QOS_CTRL_CFG%s to value 0x2010"]
impl crate::Resettable for AR_QOS_CTRL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x2010;
}
