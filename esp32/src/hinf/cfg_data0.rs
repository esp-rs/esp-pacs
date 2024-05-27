#[doc = "Register `CFG_DATA0` reader"]
pub type R = crate::R<CFG_DATA0_SPEC>;
#[doc = "Register `CFG_DATA0` writer"]
pub type W = crate::W<CFG_DATA0_SPEC>;
#[doc = "Field `USER_ID_FN1` reader - "]
pub type USER_ID_FN1_R = crate::FieldReader<u16>;
#[doc = "Field `USER_ID_FN1` writer - "]
pub type USER_ID_FN1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DEVICE_ID_FN1` reader - "]
pub type DEVICE_ID_FN1_R = crate::FieldReader<u16>;
#[doc = "Field `DEVICE_ID_FN1` writer - "]
pub type DEVICE_ID_FN1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn1(&self) -> USER_ID_FN1_R {
        USER_ID_FN1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn1(&self) -> DEVICE_ID_FN1_R {
        DEVICE_ID_FN1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA0")
            .field("user_id_fn1", &self.user_id_fn1())
            .field("device_id_fn1", &self.device_id_fn1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn user_id_fn1(&mut self) -> USER_ID_FN1_W<CFG_DATA0_SPEC> {
        USER_ID_FN1_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn device_id_fn1(&mut self) -> DEVICE_ID_FN1_W<CFG_DATA0_SPEC> {
        DEVICE_ID_FN1_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_data0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_DATA0_SPEC;
impl crate::RegisterSpec for CFG_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data0::R`](R) reader structure"]
impl crate::Readable for CFG_DATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_data0::W`](W) writer structure"]
impl crate::Writable for CFG_DATA0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DATA0 to value 0x2222_6666"]
impl crate::Resettable for CFG_DATA0_SPEC {
    const RESET_VALUE: u32 = 0x2222_6666;
}
