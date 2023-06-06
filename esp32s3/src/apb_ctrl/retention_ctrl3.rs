#[doc = "Register `RETENTION_CTRL3` reader"]
pub struct R(crate::R<RETENTION_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CTRL3` writer"]
pub struct W(crate::W<RETENTION_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RETENTION_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RET_DCACHE_SIZE` reader - ******* Description ***********"]
pub type RET_DCACHE_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `RET_DCACHE_SIZE` writer - ******* Description ***********"]
pub type RET_DCACHE_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, RETENTION_CTRL3_SPEC, 9, O, u16>;
#[doc = "Field `RET_DCACHE_VLD_SIZE` reader - ******* Description ***********"]
pub type RET_DCACHE_VLD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `RET_DCACHE_VLD_SIZE` writer - ******* Description ***********"]
pub type RET_DCACHE_VLD_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, RETENTION_CTRL3_SPEC, 9, O, u16>;
#[doc = "Field `RET_DCACHE_START_POINT` reader - ******* Description ***********"]
pub type RET_DCACHE_START_POINT_R = crate::FieldReader<u16>;
#[doc = "Field `RET_DCACHE_START_POINT` writer - ******* Description ***********"]
pub type RET_DCACHE_START_POINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, RETENTION_CTRL3_SPEC, 9, O, u16>;
#[doc = "Field `RET_DCACHE_ENABLE` reader - ******* Description ***********"]
pub type RET_DCACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `RET_DCACHE_ENABLE` writer - ******* Description ***********"]
pub type RET_DCACHE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, RETENTION_CTRL3_SPEC, O>;
impl R {
    #[doc = "Bits 4:12 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_dcache_size(&self) -> RET_DCACHE_SIZE_R {
        RET_DCACHE_SIZE_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 13:21 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_dcache_vld_size(&self) -> RET_DCACHE_VLD_SIZE_R {
        RET_DCACHE_VLD_SIZE_R::new(((self.bits >> 13) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:30 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_dcache_start_point(&self) -> RET_DCACHE_START_POINT_R {
        RET_DCACHE_START_POINT_R::new(((self.bits >> 22) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_dcache_enable(&self) -> RET_DCACHE_ENABLE_R {
        RET_DCACHE_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL3")
            .field(
                "ret_dcache_size",
                &format_args!("{}", self.ret_dcache_size().bits()),
            )
            .field(
                "ret_dcache_vld_size",
                &format_args!("{}", self.ret_dcache_vld_size().bits()),
            )
            .field(
                "ret_dcache_start_point",
                &format_args!("{}", self.ret_dcache_start_point().bits()),
            )
            .field(
                "ret_dcache_enable",
                &format_args!("{}", self.ret_dcache_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CTRL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:12 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn ret_dcache_size(&mut self) -> RET_DCACHE_SIZE_W<4> {
        RET_DCACHE_SIZE_W::new(self)
    }
    #[doc = "Bits 13:21 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn ret_dcache_vld_size(&mut self) -> RET_DCACHE_VLD_SIZE_W<13> {
        RET_DCACHE_VLD_SIZE_W::new(self)
    }
    #[doc = "Bits 22:30 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn ret_dcache_start_point(&mut self) -> RET_DCACHE_START_POINT_W<22> {
        RET_DCACHE_START_POINT_W::new(self)
    }
    #[doc = "Bit 31 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn ret_dcache_enable(&mut self) -> RET_DCACHE_ENABLE_W<31> {
        RET_DCACHE_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl3](index.html) module"]
pub struct RETENTION_CTRL3_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_ctrl3::R](R) reader structure"]
impl crate::Readable for RETENTION_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_ctrl3::W](W) writer structure"]
impl crate::Writable for RETENTION_CTRL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL3 to value 0x003f_fff0"]
impl crate::Resettable for RETENTION_CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_fff0;
}
