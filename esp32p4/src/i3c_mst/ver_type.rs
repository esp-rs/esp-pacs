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
            .field(
                "reg_i3c_mst_ver_type",
                &format_args!("{}", self.reg_i3c_mst_ver_type().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VER_TYPE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the controller current release type that is read by an application."]
    #[inline(always)]
    #[must_use]
    pub fn reg_i3c_mst_ver_type(&mut self) -> REG_I3C_MST_VER_TYPE_W<VER_TYPE_SPEC> {
        REG_I3C_MST_VER_TYPE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VER_TYPE_SPEC;
impl crate::RegisterSpec for VER_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver_type::R`](R) reader structure"]
impl crate::Readable for VER_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ver_type::W`](W) writer structure"]
impl crate::Writable for VER_TYPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VER_TYPE to value 0"]
impl crate::Resettable for VER_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
