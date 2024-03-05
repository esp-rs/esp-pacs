#[doc = "Register `IBUS_PMS_TBL_ATTR` reader"]
pub type R = crate::R<IBUS_PMS_TBL_ATTR_SPEC>;
#[doc = "Register `IBUS_PMS_TBL_ATTR` writer"]
pub type W = crate::W<IBUS_PMS_TBL_ATTR_SPEC>;
#[doc = "Field `IBUS_PMS_SCT1_ATTR` reader - The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
pub type IBUS_PMS_SCT1_ATTR_R = crate::FieldReader;
#[doc = "Field `IBUS_PMS_SCT1_ATTR` writer - The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
pub type IBUS_PMS_SCT1_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IBUS_PMS_SCT2_ATTR` reader - The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
pub type IBUS_PMS_SCT2_ATTR_R = crate::FieldReader;
#[doc = "Field `IBUS_PMS_SCT2_ATTR` writer - The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
pub type IBUS_PMS_SCT2_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    #[inline(always)]
    pub fn ibus_pms_sct1_attr(&self) -> IBUS_PMS_SCT1_ATTR_R {
        IBUS_PMS_SCT1_ATTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    #[inline(always)]
    pub fn ibus_pms_sct2_attr(&self) -> IBUS_PMS_SCT2_ATTR_R {
        IBUS_PMS_SCT2_ATTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS_PMS_TBL_ATTR")
            .field(
                "ibus_pms_sct1_attr",
                &format_args!("{}", self.ibus_pms_sct1_attr().bits()),
            )
            .field(
                "ibus_pms_sct2_attr",
                &format_args!("{}", self.ibus_pms_sct2_attr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBUS_PMS_TBL_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    #[inline(always)]
    #[must_use]
    pub fn ibus_pms_sct1_attr(&mut self) -> IBUS_PMS_SCT1_ATTR_W<IBUS_PMS_TBL_ATTR_SPEC> {
        IBUS_PMS_SCT1_ATTR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    #[inline(always)]
    #[must_use]
    pub fn ibus_pms_sct2_attr(&mut self) -> IBUS_PMS_SCT2_ATTR_W<IBUS_PMS_TBL_ATTR_SPEC> {
        IBUS_PMS_SCT2_ATTR_W::new(self, 4)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_pms_tbl_attr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibus_pms_tbl_attr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBUS_PMS_TBL_ATTR_SPEC;
impl crate::RegisterSpec for IBUS_PMS_TBL_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibus_pms_tbl_attr::R`](R) reader structure"]
impl crate::Readable for IBUS_PMS_TBL_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibus_pms_tbl_attr::W`](W) writer structure"]
impl crate::Writable for IBUS_PMS_TBL_ATTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBUS_PMS_TBL_ATTR to value 0xff"]
impl crate::Resettable for IBUS_PMS_TBL_ATTR_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
