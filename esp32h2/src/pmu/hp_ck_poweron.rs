#[doc = "Register `HP_CK_POWERON` reader"]
pub type R = crate::R<HP_CK_POWERON_SPEC>;
#[doc = "Register `HP_CK_POWERON` writer"]
pub type W = crate::W<HP_CK_POWERON_SPEC>;
#[doc = "Field `I2C_POR_WAIT_TARGET` reader - need_des"]
pub type I2C_POR_WAIT_TARGET_R = crate::FieldReader;
#[doc = "Field `I2C_POR_WAIT_TARGET` writer - need_des"]
pub type I2C_POR_WAIT_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn i2c_por_wait_target(&self) -> I2C_POR_WAIT_TARGET_R {
        I2C_POR_WAIT_TARGET_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CK_POWERON")
            .field(
                "i2c_por_wait_target",
                &format_args!("{}", self.i2c_por_wait_target().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CK_POWERON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_por_wait_target(&mut self) -> I2C_POR_WAIT_TARGET_W<HP_CK_POWERON_SPEC> {
        I2C_POR_WAIT_TARGET_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_ck_poweron::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_ck_poweron::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CK_POWERON_SPEC;
impl crate::RegisterSpec for HP_CK_POWERON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_ck_poweron::R`](R) reader structure"]
impl crate::Readable for HP_CK_POWERON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_ck_poweron::W`](W) writer structure"]
impl crate::Writable for HP_CK_POWERON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_CK_POWERON to value 0x32"]
impl crate::Resettable for HP_CK_POWERON_SPEC {
    const RESET_VALUE: u32 = 0x32;
}
