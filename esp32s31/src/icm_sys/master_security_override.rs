#[doc = "Register `MASTER_SECURITY_OVERRIDE` reader"]
pub type R = crate::R<MASTER_SECURITY_OVERRIDE_SPEC>;
#[doc = "Register `MASTER_SECURITY_OVERRIDE` writer"]
pub type W = crate::W<MASTER_SECURITY_OVERRIDE_SPEC>;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_0` reader - "]
pub type REG_MASTER_SECURITY_OVERRIDE_0_R = crate::BitReader;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_0` writer - "]
pub type REG_MASTER_SECURITY_OVERRIDE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_1` reader - "]
pub type REG_MASTER_SECURITY_OVERRIDE_1_R = crate::BitReader;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_1` writer - "]
pub type REG_MASTER_SECURITY_OVERRIDE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_2` reader - "]
pub type REG_MASTER_SECURITY_OVERRIDE_2_R = crate::BitReader;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_2` writer - "]
pub type REG_MASTER_SECURITY_OVERRIDE_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_3` reader - "]
pub type REG_MASTER_SECURITY_OVERRIDE_3_R = crate::BitReader;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_3` writer - "]
pub type REG_MASTER_SECURITY_OVERRIDE_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_4` reader - "]
pub type REG_MASTER_SECURITY_OVERRIDE_4_R = crate::BitReader;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_4` writer - "]
pub type REG_MASTER_SECURITY_OVERRIDE_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_5` reader - "]
pub type REG_MASTER_SECURITY_OVERRIDE_5_R = crate::BitReader;
#[doc = "Field `REG_MASTER_SECURITY_OVERRIDE_5` writer - "]
pub type REG_MASTER_SECURITY_OVERRIDE_5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_master_security_override_0(&self) -> REG_MASTER_SECURITY_OVERRIDE_0_R {
        REG_MASTER_SECURITY_OVERRIDE_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_master_security_override_1(&self) -> REG_MASTER_SECURITY_OVERRIDE_1_R {
        REG_MASTER_SECURITY_OVERRIDE_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_master_security_override_2(&self) -> REG_MASTER_SECURITY_OVERRIDE_2_R {
        REG_MASTER_SECURITY_OVERRIDE_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_master_security_override_3(&self) -> REG_MASTER_SECURITY_OVERRIDE_3_R {
        REG_MASTER_SECURITY_OVERRIDE_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_master_security_override_4(&self) -> REG_MASTER_SECURITY_OVERRIDE_4_R {
        REG_MASTER_SECURITY_OVERRIDE_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_master_security_override_5(&self) -> REG_MASTER_SECURITY_OVERRIDE_5_R {
        REG_MASTER_SECURITY_OVERRIDE_5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASTER_SECURITY_OVERRIDE")
            .field(
                "reg_master_security_override_0",
                &self.reg_master_security_override_0(),
            )
            .field(
                "reg_master_security_override_1",
                &self.reg_master_security_override_1(),
            )
            .field(
                "reg_master_security_override_2",
                &self.reg_master_security_override_2(),
            )
            .field(
                "reg_master_security_override_3",
                &self.reg_master_security_override_3(),
            )
            .field(
                "reg_master_security_override_4",
                &self.reg_master_security_override_4(),
            )
            .field(
                "reg_master_security_override_5",
                &self.reg_master_security_override_5(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_master_security_override_0(
        &mut self,
    ) -> REG_MASTER_SECURITY_OVERRIDE_0_W<'_, MASTER_SECURITY_OVERRIDE_SPEC> {
        REG_MASTER_SECURITY_OVERRIDE_0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_master_security_override_1(
        &mut self,
    ) -> REG_MASTER_SECURITY_OVERRIDE_1_W<'_, MASTER_SECURITY_OVERRIDE_SPEC> {
        REG_MASTER_SECURITY_OVERRIDE_1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_master_security_override_2(
        &mut self,
    ) -> REG_MASTER_SECURITY_OVERRIDE_2_W<'_, MASTER_SECURITY_OVERRIDE_SPEC> {
        REG_MASTER_SECURITY_OVERRIDE_2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_master_security_override_3(
        &mut self,
    ) -> REG_MASTER_SECURITY_OVERRIDE_3_W<'_, MASTER_SECURITY_OVERRIDE_SPEC> {
        REG_MASTER_SECURITY_OVERRIDE_3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_master_security_override_4(
        &mut self,
    ) -> REG_MASTER_SECURITY_OVERRIDE_4_W<'_, MASTER_SECURITY_OVERRIDE_SPEC> {
        REG_MASTER_SECURITY_OVERRIDE_4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_master_security_override_5(
        &mut self,
    ) -> REG_MASTER_SECURITY_OVERRIDE_5_W<'_, MASTER_SECURITY_OVERRIDE_SPEC> {
        REG_MASTER_SECURITY_OVERRIDE_5_W::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`master_security_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_security_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASTER_SECURITY_OVERRIDE_SPEC;
impl crate::RegisterSpec for MASTER_SECURITY_OVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master_security_override::R`](R) reader structure"]
impl crate::Readable for MASTER_SECURITY_OVERRIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`master_security_override::W`](W) writer structure"]
impl crate::Writable for MASTER_SECURITY_OVERRIDE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASTER_SECURITY_OVERRIDE to value 0"]
impl crate::Resettable for MASTER_SECURITY_OVERRIDE_SPEC {}
