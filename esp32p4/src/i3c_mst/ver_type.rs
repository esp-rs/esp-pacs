#[doc = "Register `VER_TYPE` reader"]
pub type R = crate::R<VER_TYPE_SPEC>;
#[doc = "Register `VER_TYPE` writer"]
pub type W = crate::W<VER_TYPE_SPEC>;
#[doc = "Field `REG_I3C_MST_VER_TYPE` reader - This field indicates the controller current release type that is read by an application."]
pub type REG_I3C_MST_VER_TYPE_R = crate::FieldReader<u32>;
#[doc = "Field `REG_I3C_MST_VER_TYPE` writer - This field indicates the controller current release type that is read by an application."]
pub type REG_I3C_MST_VER_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the controller current release type that is read by an application."]
    #[inline(always)]
    pub fn reg_i3c_mst_ver_type(&self) -> REG_I3C_MST_VER_TYPE_R {
        REG_I3C_MST_VER_TYPE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER_TYPE")
            .field("reg_i3c_mst_ver_type", &self.reg_i3c_mst_ver_type())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the controller current release type that is read by an application."]
    #[inline(always)]
    pub fn reg_i3c_mst_ver_type(&mut self) -> REG_I3C_MST_VER_TYPE_W<'_, VER_TYPE_SPEC> {
        REG_I3C_MST_VER_TYPE_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VER_TYPE_SPEC;
impl crate::RegisterSpec for VER_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver_type::R`](R) reader structure"]
impl crate::Readable for VER_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ver_type::W`](W) writer structure"]
impl crate::Writable for VER_TYPE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VER_TYPE to value 0"]
impl crate::Resettable for VER_TYPE_SPEC {}
