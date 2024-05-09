#[doc = "Register `IFS` reader"]
pub type R = crate::R<IFS_SPEC>;
#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `SIFS` reader - "]
pub type SIFS_R = crate::FieldReader;
#[doc = "Field `SIFS` writer - "]
pub type SIFS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LIFS` reader - "]
pub type LIFS_R = crate::FieldReader<u16>;
#[doc = "Field `LIFS` writer - "]
pub type LIFS_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sifs(&self) -> SIFS_R {
        SIFS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn lifs(&self) -> LIFS_R {
        LIFS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFS")
            .field("sifs", &format_args!("{}", self.sifs().bits()))
            .field("lifs", &format_args!("{}", self.lifs().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IFS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sifs(&mut self) -> SIFS_W<IFS_SPEC> {
        SIFS_W::new(self, 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn lifs(&mut self) -> LIFS_W<IFS_SPEC> {
        LIFS_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifs::R`](R) reader structure"]
impl crate::Readable for IFS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
