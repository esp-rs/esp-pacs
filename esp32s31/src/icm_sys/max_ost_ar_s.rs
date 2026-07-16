#[doc = "Register `MAX_OST_AR_S%s` reader"]
pub type R = crate::R<MAX_OST_AR_S_SPEC>;
#[doc = "Register `MAX_OST_AR_S%s` writer"]
pub type W = crate::W<MAX_OST_AR_S_SPEC>;
#[doc = "Field `REG_MAX_OST_AR_S_0` reader - "]
pub type REG_MAX_OST_AR_S_0_R = crate::FieldReader;
#[doc = "Field `REG_MAX_OST_AR_S_0` writer - "]
pub type REG_MAX_OST_AR_S_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_max_ost_ar_s_0(&self) -> REG_MAX_OST_AR_S_0_R {
        REG_MAX_OST_AR_S_0_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAX_OST_AR_S")
            .field("reg_max_ost_ar_s_0", &self.reg_max_ost_ar_s_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_max_ost_ar_s_0(&mut self) -> REG_MAX_OST_AR_S_0_W<'_, MAX_OST_AR_S_SPEC> {
        REG_MAX_OST_AR_S_0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`max_ost_ar_s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_ost_ar_s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAX_OST_AR_S_SPEC;
impl crate::RegisterSpec for MAX_OST_AR_S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_ost_ar_s::R`](R) reader structure"]
impl crate::Readable for MAX_OST_AR_S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`max_ost_ar_s::W`](W) writer structure"]
impl crate::Writable for MAX_OST_AR_S_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAX_OST_AR_S%s to value 0x0f"]
impl crate::Resettable for MAX_OST_AR_S_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
