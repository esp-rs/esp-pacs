#[doc = "Register `DBUS_PMS_TBL_ATTR` reader"]
pub type R = crate::R<DBUS_PMS_TBL_ATTR_SPEC>;
#[doc = "Register `DBUS_PMS_TBL_ATTR` writer"]
pub type W = crate::W<DBUS_PMS_TBL_ATTR_SPEC>;
#[doc = "Field `DBUS_PMS_SCT1_ATTR` reader - The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
pub type DBUS_PMS_SCT1_ATTR_R = crate::FieldReader;
#[doc = "Field `DBUS_PMS_SCT1_ATTR` writer - The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
pub type DBUS_PMS_SCT1_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBUS_PMS_SCT2_ATTR` reader - The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
pub type DBUS_PMS_SCT2_ATTR_R = crate::FieldReader;
#[doc = "Field `DBUS_PMS_SCT2_ATTR` writer - The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
pub type DBUS_PMS_SCT2_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
    #[inline(always)]
    pub fn dbus_pms_sct1_attr(&self) -> DBUS_PMS_SCT1_ATTR_R {
        DBUS_PMS_SCT1_ATTR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
    #[inline(always)]
    pub fn dbus_pms_sct2_attr(&self) -> DBUS_PMS_SCT2_ATTR_R {
        DBUS_PMS_SCT2_ATTR_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS_PMS_TBL_ATTR")
            .field(
                "dbus_pms_sct1_attr",
                &format_args!("{}", self.dbus_pms_sct1_attr().bits()),
            )
            .field(
                "dbus_pms_sct2_attr",
                &format_args!("{}", self.dbus_pms_sct2_attr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS_PMS_TBL_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
    #[inline(always)]
    #[must_use]
    pub fn dbus_pms_sct1_attr(&mut self) -> DBUS_PMS_SCT1_ATTR_W<DBUS_PMS_TBL_ATTR_SPEC> {
        DBUS_PMS_SCT1_ATTR_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
    #[inline(always)]
    #[must_use]
    pub fn dbus_pms_sct2_attr(&mut self) -> DBUS_PMS_SCT2_ATTR_W<DBUS_PMS_TBL_ATTR_SPEC> {
        DBUS_PMS_SCT2_ATTR_W::new(self, 2)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_pms_tbl_attr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbus_pms_tbl_attr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS_PMS_TBL_ATTR_SPEC;
impl crate::RegisterSpec for DBUS_PMS_TBL_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus_pms_tbl_attr::R`](R) reader structure"]
impl crate::Readable for DBUS_PMS_TBL_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbus_pms_tbl_attr::W`](W) writer structure"]
impl crate::Writable for DBUS_PMS_TBL_ATTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBUS_PMS_TBL_ATTR to value 0x0f"]
impl crate::Resettable for DBUS_PMS_TBL_ATTR_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
