#[doc = "Register `CH%sADDR` reader"]
pub struct R(crate::R<CHADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_MEM_WADDR` reader - This register records the memory address offset when writes RAM over APB bus."]
pub type APB_MEM_WADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `APB_MEM_RADDR` reader - This register records the memory address offset when reads RAM over APB bus."]
pub type APB_MEM_RADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - This register records the memory address offset when writes RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_waddr(&self) -> APB_MEM_WADDR_R {
        APB_MEM_WADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 10:18 - This register records the memory address offset when reads RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_raddr(&self) -> APB_MEM_RADDR_R {
        APB_MEM_RADDR_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHADDR")
            .field(
                "apb_mem_waddr",
                &format_args!("{}", self.apb_mem_waddr().bits()),
            )
            .field(
                "apb_mem_raddr",
                &format_args!("{}", self.apb_mem_raddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Channel %s address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chaddr](index.html) module"]
pub struct CHADDR_SPEC;
impl crate::RegisterSpec for CHADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chaddr::R](R) reader structure"]
impl crate::Readable for CHADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%sADDR to value 0"]
impl crate::Resettable for CHADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
