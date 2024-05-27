///Register `CFG_DATA16` reader
pub type R = crate::R<CFG_DATA16_SPEC>;
///Register `CFG_DATA16` writer
pub type W = crate::W<CFG_DATA16_SPEC>;
///Field `DEVICE_ID_FN2` reader - configure device id of function2 in cis
pub type DEVICE_ID_FN2_R = crate::FieldReader<u16>;
///Field `DEVICE_ID_FN2` writer - configure device id of function2 in cis
pub type DEVICE_ID_FN2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `USER_ID_FN2` reader - configure user id of function2 in cis
pub type USER_ID_FN2_R = crate::FieldReader<u16>;
///Field `USER_ID_FN2` writer - configure user id of function2 in cis
pub type USER_ID_FN2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - configure device id of function2 in cis
    #[inline(always)]
    pub fn device_id_fn2(&self) -> DEVICE_ID_FN2_R {
        DEVICE_ID_FN2_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - configure user id of function2 in cis
    #[inline(always)]
    pub fn user_id_fn2(&self) -> USER_ID_FN2_R {
        USER_ID_FN2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA16")
            .field("device_id_fn2", &self.device_id_fn2())
            .field("user_id_fn2", &self.user_id_fn2())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - configure device id of function2 in cis
    #[inline(always)]
    #[must_use]
    pub fn device_id_fn2(&mut self) -> DEVICE_ID_FN2_W<CFG_DATA16_SPEC> {
        DEVICE_ID_FN2_W::new(self, 0)
    }
    ///Bits 16:31 - configure user id of function2 in cis
    #[inline(always)]
    #[must_use]
    pub fn user_id_fn2(&mut self) -> USER_ID_FN2_W<CFG_DATA16_SPEC> {
        USER_ID_FN2_W::new(self, 16)
    }
}
/**SDIO cis configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cfg_data16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFG_DATA16_SPEC;
impl crate::RegisterSpec for CFG_DATA16_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cfg_data16::R`](R) reader structure
impl crate::Readable for CFG_DATA16_SPEC {}
///`write(|w| ..)` method takes [`cfg_data16::W`](W) writer structure
impl crate::Writable for CFG_DATA16_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFG_DATA16 to value 0x0092_7777
impl crate::Resettable for CFG_DATA16_SPEC {
    const RESET_VALUE: u32 = 0x0092_7777;
}
