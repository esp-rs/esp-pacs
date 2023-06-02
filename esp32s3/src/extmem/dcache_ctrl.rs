#[doc = "Register `DCACHE_CTRL` reader"]
pub struct R(crate::R<DCACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_CTRL` writer"]
pub struct W(crate::W<DCACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_CTRL_SPEC>;
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
impl From<crate::W<DCACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_ENABLE` reader - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub type DCACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `DCACHE_ENABLE` writer - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub type DCACHE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, DCACHE_CTRL_SPEC, O>;
#[doc = "Field `DCACHE_SIZE_MODE` reader - The bit is used to configure cache memory size.0: 32KB, 1: 64KB"]
pub type DCACHE_SIZE_MODE_R = crate::BitReader;
#[doc = "Field `DCACHE_SIZE_MODE` writer - The bit is used to configure cache memory size.0: 32KB, 1: 64KB"]
pub type DCACHE_SIZE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, DCACHE_CTRL_SPEC, O>;
#[doc = "Field `DCACHE_BLOCKSIZE_MODE` reader - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes"]
pub type DCACHE_BLOCKSIZE_MODE_R = crate::FieldReader;
#[doc = "Field `DCACHE_BLOCKSIZE_MODE` writer - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes"]
pub type DCACHE_BLOCKSIZE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DCACHE_CTRL_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dcache_enable(&self) -> DCACHE_ENABLE_R {
        DCACHE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 32KB, 1: 64KB"]
    #[inline(always)]
    pub fn dcache_size_mode(&self) -> DCACHE_SIZE_MODE_R {
        DCACHE_SIZE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes"]
    #[inline(always)]
    pub fn dcache_blocksize_mode(&self) -> DCACHE_BLOCKSIZE_MODE_R {
        DCACHE_BLOCKSIZE_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_CTRL")
            .field(
                "dcache_enable",
                &format_args!("{}", self.dcache_enable().bit()),
            )
            .field(
                "dcache_size_mode",
                &format_args!("{}", self.dcache_size_mode().bit()),
            )
            .field(
                "dcache_blocksize_mode",
                &format_args!("{}", self.dcache_blocksize_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_enable(&mut self) -> DCACHE_ENABLE_W<0> {
        DCACHE_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 32KB, 1: 64KB"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_size_mode(&mut self) -> DCACHE_SIZE_MODE_W<2> {
        DCACHE_SIZE_MODE_W::new(self)
    }
    #[doc = "Bits 3:4 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_blocksize_mode(&mut self) -> DCACHE_BLOCKSIZE_MODE_W<3> {
        DCACHE_BLOCKSIZE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_ctrl](index.html) module"]
pub struct DCACHE_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_ctrl::R](R) reader structure"]
impl crate::Readable for DCACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_ctrl::W](W) writer structure"]
impl crate::Writable for DCACHE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_CTRL to value 0"]
impl crate::Resettable for DCACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
