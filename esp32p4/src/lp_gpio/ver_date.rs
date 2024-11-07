#[doc = "Register `VER_DATE` reader"]
pub type R = crate::R<VER_DATE_SPEC>;
#[doc = "Register `VER_DATE` writer"]
pub type W = crate::W<VER_DATE_SPEC>;
#[doc = "Field `REG_VER_DATE` reader - Reserved"]
pub type REG_VER_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `REG_VER_DATE` writer - Reserved"]
pub type REG_VER_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn reg_ver_date(&self) -> REG_VER_DATE_R {
        REG_VER_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER_DATE")
            .field("reg_ver_date", &self.reg_ver_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn reg_ver_date(&mut self) -> REG_VER_DATE_W<VER_DATE_SPEC> {
        REG_VER_DATE_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VER_DATE_SPEC;
impl crate::RegisterSpec for VER_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver_date::R`](R) reader structure"]
impl crate::Readable for VER_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ver_date::W`](W) writer structure"]
impl crate::Writable for VER_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VER_DATE to value 0x0023_0323"]
impl crate::Resettable for VER_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0023_0323;
}
