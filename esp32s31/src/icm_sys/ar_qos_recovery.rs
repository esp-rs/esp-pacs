#[doc = "Register `AR_QOS_RECOVERY` reader"]
pub type R = crate::R<AR_QOS_RECOVERY_SPEC>;
#[doc = "Register `AR_QOS_RECOVERY` writer"]
pub type W = crate::W<AR_QOS_RECOVERY_SPEC>;
#[doc = "Field `REG_AR_QOS_RECOVERY_0` reader - "]
pub type REG_AR_QOS_RECOVERY_0_R = crate::BitReader;
#[doc = "Field `REG_AR_QOS_RECOVERY_0` writer - "]
pub type REG_AR_QOS_RECOVERY_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_AR_QOS_RECOVERY_1` reader - "]
pub type REG_AR_QOS_RECOVERY_1_R = crate::BitReader;
#[doc = "Field `REG_AR_QOS_RECOVERY_1` writer - "]
pub type REG_AR_QOS_RECOVERY_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_AR_QOS_RECOVERY_2` reader - "]
pub type REG_AR_QOS_RECOVERY_2_R = crate::BitReader;
#[doc = "Field `REG_AR_QOS_RECOVERY_2` writer - "]
pub type REG_AR_QOS_RECOVERY_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ar_qos_recovery_0(&self) -> REG_AR_QOS_RECOVERY_0_R {
        REG_AR_QOS_RECOVERY_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ar_qos_recovery_1(&self) -> REG_AR_QOS_RECOVERY_1_R {
        REG_AR_QOS_RECOVERY_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ar_qos_recovery_2(&self) -> REG_AR_QOS_RECOVERY_2_R {
        REG_AR_QOS_RECOVERY_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AR_QOS_RECOVERY")
            .field("reg_ar_qos_recovery_0", &self.reg_ar_qos_recovery_0())
            .field("reg_ar_qos_recovery_1", &self.reg_ar_qos_recovery_1())
            .field("reg_ar_qos_recovery_2", &self.reg_ar_qos_recovery_2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ar_qos_recovery_0(&mut self) -> REG_AR_QOS_RECOVERY_0_W<'_, AR_QOS_RECOVERY_SPEC> {
        REG_AR_QOS_RECOVERY_0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ar_qos_recovery_1(&mut self) -> REG_AR_QOS_RECOVERY_1_W<'_, AR_QOS_RECOVERY_SPEC> {
        REG_AR_QOS_RECOVERY_1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ar_qos_recovery_2(&mut self) -> REG_AR_QOS_RECOVERY_2_W<'_, AR_QOS_RECOVERY_SPEC> {
        REG_AR_QOS_RECOVERY_2_W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_recovery::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_recovery::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AR_QOS_RECOVERY_SPEC;
impl crate::RegisterSpec for AR_QOS_RECOVERY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar_qos_recovery::R`](R) reader structure"]
impl crate::Readable for AR_QOS_RECOVERY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ar_qos_recovery::W`](W) writer structure"]
impl crate::Writable for AR_QOS_RECOVERY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AR_QOS_RECOVERY to value 0"]
impl crate::Resettable for AR_QOS_RECOVERY_SPEC {}
