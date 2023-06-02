#[doc = "Register `ICACHE_CTRL` reader"]
pub struct R(crate::R<ICACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_CTRL` writer"]
pub struct W(crate::W<ICACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_CTRL_SPEC>;
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
impl From<crate::W<ICACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_ENABLE` reader - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub type ICACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `ICACHE_ENABLE` writer - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub type ICACHE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, ICACHE_CTRL_SPEC, O>;
#[doc = "Field `ICACHE_WAY_MODE` reader - The bit is used to configure cache way mode.0: 4-way, 1: 8-way"]
pub type ICACHE_WAY_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE_WAY_MODE` writer - The bit is used to configure cache way mode.0: 4-way, 1: 8-way"]
pub type ICACHE_WAY_MODE_W<'a, const O: u8> = crate::BitWriter<'a, ICACHE_CTRL_SPEC, O>;
#[doc = "Field `ICACHE_SIZE_MODE` reader - The bit is used to configure cache memory size.0: 16KB, 1: 32KB"]
pub type ICACHE_SIZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE_SIZE_MODE` writer - The bit is used to configure cache memory size.0: 16KB, 1: 32KB"]
pub type ICACHE_SIZE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, ICACHE_CTRL_SPEC, O>;
#[doc = "Field `ICACHE_BLOCKSIZE_MODE` reader - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
pub type ICACHE_BLOCKSIZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE_BLOCKSIZE_MODE` writer - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
pub type ICACHE_BLOCKSIZE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, ICACHE_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn icache_enable(&self) -> ICACHE_ENABLE_R {
        ICACHE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure cache way mode.0: 4-way, 1: 8-way"]
    #[inline(always)]
    pub fn icache_way_mode(&self) -> ICACHE_WAY_MODE_R {
        ICACHE_WAY_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 16KB, 1: 32KB"]
    #[inline(always)]
    pub fn icache_size_mode(&self) -> ICACHE_SIZE_MODE_R {
        ICACHE_SIZE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
    #[inline(always)]
    pub fn icache_blocksize_mode(&self) -> ICACHE_BLOCKSIZE_MODE_R {
        ICACHE_BLOCKSIZE_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_CTRL")
            .field(
                "icache_enable",
                &format_args!("{}", self.icache_enable().bit()),
            )
            .field(
                "icache_way_mode",
                &format_args!("{}", self.icache_way_mode().bit()),
            )
            .field(
                "icache_size_mode",
                &format_args!("{}", self.icache_size_mode().bit()),
            )
            .field(
                "icache_blocksize_mode",
                &format_args!("{}", self.icache_blocksize_mode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICACHE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn icache_enable(&mut self) -> ICACHE_ENABLE_W<0> {
        ICACHE_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to configure cache way mode.0: 4-way, 1: 8-way"]
    #[inline(always)]
    #[must_use]
    pub fn icache_way_mode(&mut self) -> ICACHE_WAY_MODE_W<1> {
        ICACHE_WAY_MODE_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 16KB, 1: 32KB"]
    #[inline(always)]
    #[must_use]
    pub fn icache_size_mode(&mut self) -> ICACHE_SIZE_MODE_W<2> {
        ICACHE_SIZE_MODE_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn icache_blocksize_mode(&mut self) -> ICACHE_BLOCKSIZE_MODE_W<3> {
        ICACHE_BLOCKSIZE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_ctrl](index.html) module"]
pub struct ICACHE_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_ctrl::R](R) reader structure"]
impl crate::Readable for ICACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_ctrl::W](W) writer structure"]
impl crate::Writable for ICACHE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_CTRL to value 0"]
impl crate::Resettable for ICACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
