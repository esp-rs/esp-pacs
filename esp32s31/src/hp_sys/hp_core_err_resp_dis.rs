#[doc = "Register `HP_CORE_ERR_RESP_DIS` reader"]
pub type R = crate::R<HP_CORE_ERR_RESP_DIS_SPEC>;
#[doc = "Register `HP_CORE_ERR_RESP_DIS` writer"]
pub type W = crate::W<HP_CORE_ERR_RESP_DIS_SPEC>;
#[doc = "Field `HP_CORE_ERR_RESP_DIS` reader - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
pub type HP_CORE_ERR_RESP_DIS_R = crate::FieldReader;
#[doc = "Field `HP_CORE_ERR_RESP_DIS` writer - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
pub type HP_CORE_ERR_RESP_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
    #[inline(always)]
    pub fn hp_core_err_resp_dis(&self) -> HP_CORE_ERR_RESP_DIS_R {
        HP_CORE_ERR_RESP_DIS_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CORE_ERR_RESP_DIS")
            .field("hp_core_err_resp_dis", &self.hp_core_err_resp_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
    #[inline(always)]
    pub fn hp_core_err_resp_dis(
        &mut self,
    ) -> HP_CORE_ERR_RESP_DIS_W<'_, HP_CORE_ERR_RESP_DIS_SPEC> {
        HP_CORE_ERR_RESP_DIS_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_err_resp_dis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_err_resp_dis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORE_ERR_RESP_DIS_SPEC;
impl crate::RegisterSpec for HP_CORE_ERR_RESP_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_core_err_resp_dis::R`](R) reader structure"]
impl crate::Readable for HP_CORE_ERR_RESP_DIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_core_err_resp_dis::W`](W) writer structure"]
impl crate::Writable for HP_CORE_ERR_RESP_DIS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CORE_ERR_RESP_DIS to value 0"]
impl crate::Resettable for HP_CORE_ERR_RESP_DIS_SPEC {}
