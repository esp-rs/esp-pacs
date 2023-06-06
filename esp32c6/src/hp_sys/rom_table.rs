#[doc = "Register `ROM_TABLE` reader"]
pub struct R(crate::R<ROM_TABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_TABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_TABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_TABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_TABLE` writer"]
pub struct W(crate::W<ROM_TABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_TABLE_SPEC>;
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
impl From<crate::W<ROM_TABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_TABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_TABLE` reader - XXXX"]
pub type ROM_TABLE_R = crate::FieldReader<u32>;
#[doc = "Field `ROM_TABLE` writer - XXXX"]
pub type ROM_TABLE_W<'a, const O: u8> = crate::FieldWriter<'a, ROM_TABLE_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - XXXX"]
    #[inline(always)]
    pub fn rom_table(&self) -> ROM_TABLE_R {
        ROM_TABLE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_TABLE")
            .field("rom_table", &format_args!("{}", self.rom_table().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ROM_TABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - XXXX"]
    #[inline(always)]
    #[must_use]
    pub fn rom_table(&mut self) -> ROM_TABLE_W<0> {
        ROM_TABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rom-Table register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_table](index.html) module"]
pub struct ROM_TABLE_SPEC;
impl crate::RegisterSpec for ROM_TABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_table::R](R) reader structure"]
impl crate::Readable for ROM_TABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_table::W](W) writer structure"]
impl crate::Writable for ROM_TABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROM_TABLE to value 0"]
impl crate::Resettable for ROM_TABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
