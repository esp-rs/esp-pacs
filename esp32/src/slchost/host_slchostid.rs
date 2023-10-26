#[doc = "Register `HOST_SLCHOSTID` reader"]
pub type R = crate::R<HOST_SLCHOSTID_SPEC>;
#[doc = "Register `HOST_SLCHOSTID` writer"]
pub type W = crate::W<HOST_SLCHOSTID_SPEC>;
#[doc = "Field `HOST_SLCHOST_ID` reader - "]
pub type HOST_SLCHOST_ID_R = crate::FieldReader<u32>;
#[doc = "Field `HOST_SLCHOST_ID` writer - "]
pub type HOST_SLCHOST_ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_id(&self) -> HOST_SLCHOST_ID_R {
        HOST_SLCHOST_ID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOSTID")
            .field(
                "host_slchost_id",
                &format_args!("{}", self.host_slchost_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOSTID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_id(&mut self) -> HOST_SLCHOST_ID_W<HOST_SLCHOSTID_SPEC, 0> {
        HOST_SLCHOST_ID_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchostid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchostid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOSTID_SPEC;
impl crate::RegisterSpec for HOST_SLCHOSTID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchostid::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOSTID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchostid::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOSTID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOSTID to value 0x0600"]
impl crate::Resettable for HOST_SLCHOSTID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
