///Register `VER_ID` reader
pub type R = crate::R<VER_ID_SPEC>;
///Register `VER_ID` writer
pub type W = crate::W<VER_ID_SPEC>;
///Field `REG_I3C_MST_VER_ID` reader - This field indicates the controller current release number that is read by an application.
pub type REG_I3C_MST_VER_ID_R = crate::FieldReader<u32>;
///Field `REG_I3C_MST_VER_ID` writer - This field indicates the controller current release number that is read by an application.
pub type REG_I3C_MST_VER_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - This field indicates the controller current release number that is read by an application.
    #[inline(always)]
    pub fn reg_i3c_mst_ver_id(&self) -> REG_I3C_MST_VER_ID_R {
        REG_I3C_MST_VER_ID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER_ID")
            .field("reg_i3c_mst_ver_id", &self.reg_i3c_mst_ver_id())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - This field indicates the controller current release number that is read by an application.
    #[inline(always)]
    #[must_use]
    pub fn reg_i3c_mst_ver_id(&mut self) -> REG_I3C_MST_VER_ID_W<VER_ID_SPEC> {
        REG_I3C_MST_VER_ID_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`ver_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VER_ID_SPEC;
impl crate::RegisterSpec for VER_ID_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ver_id::R`](R) reader structure
impl crate::Readable for VER_ID_SPEC {}
///`write(|w| ..)` method takes [`ver_id::W`](W) writer structure
impl crate::Writable for VER_ID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VER_ID to value 0x2023_0504
impl crate::Resettable for VER_ID_SPEC {
    const RESET_VALUE: u32 = 0x2023_0504;
}
