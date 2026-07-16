#[doc = "Register `REG_SLAVE_IS_SECURE` reader"]
pub type R = crate::R<REG_SLAVE_IS_SECURE_SPEC>;
#[doc = "Register `REG_SLAVE_IS_SECURE` writer"]
pub type W = crate::W<REG_SLAVE_IS_SECURE_SPEC>;
#[doc = "Field `REG_SLAVE_IS_SECURE_0` reader - "]
pub type REG_SLAVE_IS_SECURE_0_R = crate::BitReader;
#[doc = "Field `REG_SLAVE_IS_SECURE_0` writer - "]
pub type REG_SLAVE_IS_SECURE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SLAVE_IS_SECURE_1` reader - "]
pub type REG_SLAVE_IS_SECURE_1_R = crate::BitReader;
#[doc = "Field `REG_SLAVE_IS_SECURE_1` writer - "]
pub type REG_SLAVE_IS_SECURE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SLAVE_IS_SECURE_2` reader - "]
pub type REG_SLAVE_IS_SECURE_2_R = crate::BitReader;
#[doc = "Field `REG_SLAVE_IS_SECURE_2` writer - "]
pub type REG_SLAVE_IS_SECURE_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_slave_is_secure_0(&self) -> REG_SLAVE_IS_SECURE_0_R {
        REG_SLAVE_IS_SECURE_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_slave_is_secure_1(&self) -> REG_SLAVE_IS_SECURE_1_R {
        REG_SLAVE_IS_SECURE_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_slave_is_secure_2(&self) -> REG_SLAVE_IS_SECURE_2_R {
        REG_SLAVE_IS_SECURE_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_SLAVE_IS_SECURE")
            .field("reg_slave_is_secure_0", &self.reg_slave_is_secure_0())
            .field("reg_slave_is_secure_1", &self.reg_slave_is_secure_1())
            .field("reg_slave_is_secure_2", &self.reg_slave_is_secure_2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_slave_is_secure_0(
        &mut self,
    ) -> REG_SLAVE_IS_SECURE_0_W<'_, REG_SLAVE_IS_SECURE_SPEC> {
        REG_SLAVE_IS_SECURE_0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_slave_is_secure_1(
        &mut self,
    ) -> REG_SLAVE_IS_SECURE_1_W<'_, REG_SLAVE_IS_SECURE_SPEC> {
        REG_SLAVE_IS_SECURE_1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_slave_is_secure_2(
        &mut self,
    ) -> REG_SLAVE_IS_SECURE_2_W<'_, REG_SLAVE_IS_SECURE_SPEC> {
        REG_SLAVE_IS_SECURE_2_W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_slave_is_secure::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_slave_is_secure::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_SLAVE_IS_SECURE_SPEC;
impl crate::RegisterSpec for REG_SLAVE_IS_SECURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_slave_is_secure::R`](R) reader structure"]
impl crate::Readable for REG_SLAVE_IS_SECURE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_slave_is_secure::W`](W) writer structure"]
impl crate::Writable for REG_SLAVE_IS_SECURE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SLAVE_IS_SECURE to value 0"]
impl crate::Resettable for REG_SLAVE_IS_SECURE_SPEC {}
