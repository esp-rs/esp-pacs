#[doc = "Register `SLAVE_SECURITY_OVERRIDE` reader"]
pub type R = crate::R<SLAVE_SECURITY_OVERRIDE_SPEC>;
#[doc = "Register `SLAVE_SECURITY_OVERRIDE` writer"]
pub type W = crate::W<SLAVE_SECURITY_OVERRIDE_SPEC>;
#[doc = "Field `REG_SLAVE_SECURITY_OVERRIDE_0` reader - "]
pub type REG_SLAVE_SECURITY_OVERRIDE_0_R = crate::BitReader;
#[doc = "Field `REG_SLAVE_SECURITY_OVERRIDE_0` writer - "]
pub type REG_SLAVE_SECURITY_OVERRIDE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SLAVE_SECURITY_OVERRIDE_1` reader - "]
pub type REG_SLAVE_SECURITY_OVERRIDE_1_R = crate::BitReader;
#[doc = "Field `REG_SLAVE_SECURITY_OVERRIDE_1` writer - "]
pub type REG_SLAVE_SECURITY_OVERRIDE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SLAVE_SECURITY_OVERRIDE_2` reader - "]
pub type REG_SLAVE_SECURITY_OVERRIDE_2_R = crate::BitReader;
#[doc = "Field `REG_SLAVE_SECURITY_OVERRIDE_2` writer - "]
pub type REG_SLAVE_SECURITY_OVERRIDE_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_slave_security_override_0(&self) -> REG_SLAVE_SECURITY_OVERRIDE_0_R {
        REG_SLAVE_SECURITY_OVERRIDE_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_slave_security_override_1(&self) -> REG_SLAVE_SECURITY_OVERRIDE_1_R {
        REG_SLAVE_SECURITY_OVERRIDE_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_slave_security_override_2(&self) -> REG_SLAVE_SECURITY_OVERRIDE_2_R {
        REG_SLAVE_SECURITY_OVERRIDE_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE_SECURITY_OVERRIDE")
            .field(
                "reg_slave_security_override_0",
                &self.reg_slave_security_override_0(),
            )
            .field(
                "reg_slave_security_override_1",
                &self.reg_slave_security_override_1(),
            )
            .field(
                "reg_slave_security_override_2",
                &self.reg_slave_security_override_2(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_slave_security_override_0(
        &mut self,
    ) -> REG_SLAVE_SECURITY_OVERRIDE_0_W<'_, SLAVE_SECURITY_OVERRIDE_SPEC> {
        REG_SLAVE_SECURITY_OVERRIDE_0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_slave_security_override_1(
        &mut self,
    ) -> REG_SLAVE_SECURITY_OVERRIDE_1_W<'_, SLAVE_SECURITY_OVERRIDE_SPEC> {
        REG_SLAVE_SECURITY_OVERRIDE_1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_slave_security_override_2(
        &mut self,
    ) -> REG_SLAVE_SECURITY_OVERRIDE_2_W<'_, SLAVE_SECURITY_OVERRIDE_SPEC> {
        REG_SLAVE_SECURITY_OVERRIDE_2_W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_security_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_security_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE_SECURITY_OVERRIDE_SPEC;
impl crate::RegisterSpec for SLAVE_SECURITY_OVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_security_override::R`](R) reader structure"]
impl crate::Readable for SLAVE_SECURITY_OVERRIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave_security_override::W`](W) writer structure"]
impl crate::Writable for SLAVE_SECURITY_OVERRIDE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLAVE_SECURITY_OVERRIDE to value 0"]
impl crate::Resettable for SLAVE_SECURITY_OVERRIDE_SPEC {}
