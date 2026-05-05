#[doc = "Register `HP_CK_CNTL` reader"]
pub type R = crate::R<HP_CK_CNTL_SPEC>;
#[doc = "Register `HP_CK_CNTL` writer"]
pub type W = crate::W<HP_CK_CNTL_SPEC>;
#[doc = "Field `MODIFY_ICG_CNTL_WAIT` reader - PMU_MODIFY_ICG_CNTL_WAIT"]
pub type MODIFY_ICG_CNTL_WAIT_R = crate::FieldReader;
#[doc = "Field `MODIFY_ICG_CNTL_WAIT` writer - PMU_MODIFY_ICG_CNTL_WAIT"]
pub type MODIFY_ICG_CNTL_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWITCH_ICG_CNTL_WAIT` reader - PMU_SWITCH_ICG_CNTL_WAIT"]
pub type SWITCH_ICG_CNTL_WAIT_R = crate::FieldReader;
#[doc = "Field `SWITCH_ICG_CNTL_WAIT` writer - PMU_SWITCH_ICG_CNTL_WAIT"]
pub type SWITCH_ICG_CNTL_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PMU_MODIFY_ICG_CNTL_WAIT"]
    #[inline(always)]
    pub fn modify_icg_cntl_wait(&self) -> MODIFY_ICG_CNTL_WAIT_R {
        MODIFY_ICG_CNTL_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PMU_SWITCH_ICG_CNTL_WAIT"]
    #[inline(always)]
    pub fn switch_icg_cntl_wait(&self) -> SWITCH_ICG_CNTL_WAIT_R {
        SWITCH_ICG_CNTL_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CK_CNTL")
            .field("modify_icg_cntl_wait", &self.modify_icg_cntl_wait())
            .field("switch_icg_cntl_wait", &self.switch_icg_cntl_wait())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - PMU_MODIFY_ICG_CNTL_WAIT"]
    #[inline(always)]
    pub fn modify_icg_cntl_wait(&mut self) -> MODIFY_ICG_CNTL_WAIT_W<'_, HP_CK_CNTL_SPEC> {
        MODIFY_ICG_CNTL_WAIT_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - PMU_SWITCH_ICG_CNTL_WAIT"]
    #[inline(always)]
    pub fn switch_icg_cntl_wait(&mut self) -> SWITCH_ICG_CNTL_WAIT_W<'_, HP_CK_CNTL_SPEC> {
        SWITCH_ICG_CNTL_WAIT_W::new(self, 8)
    }
}
#[doc = "PMU_HP_CK_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ck_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ck_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CK_CNTL_SPEC;
impl crate::RegisterSpec for HP_CK_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_ck_cntl::R`](R) reader structure"]
impl crate::Readable for HP_CK_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_ck_cntl::W`](W) writer structure"]
impl crate::Writable for HP_CK_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CK_CNTL to value 0"]
impl crate::Resettable for HP_CK_CNTL_SPEC {}
