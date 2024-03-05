#[doc = "Register `BT_SELECT` reader"]
pub type R = crate::R<BT_SELECT_SPEC>;
#[doc = "Register `BT_SELECT` writer"]
pub type W = crate::W<BT_SELECT_SPEC>;
#[doc = "Field `BT_SEL` reader - GPIO bit select register"]
pub type BT_SEL_R = crate::FieldReader<u32>;
#[doc = "Field `BT_SEL` writer - GPIO bit select register"]
pub type BT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO bit select register"]
    #[inline(always)]
    pub fn bt_sel(&self) -> BT_SEL_R {
        BT_SEL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_SELECT")
            .field("bt_sel", &format_args!("{}", self.bt_sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BT_SELECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO bit select register"]
    #[inline(always)]
    #[must_use]
    pub fn bt_sel(&mut self) -> BT_SEL_W<BT_SELECT_SPEC> {
        BT_SEL_W::new(self, 0)
    }
}
#[doc = "GPIO bit select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_SELECT_SPEC;
impl crate::RegisterSpec for BT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_select::R`](R) reader structure"]
impl crate::Readable for BT_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt_select::W`](W) writer structure"]
impl crate::Writable for BT_SELECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BT_SELECT to value 0"]
impl crate::Resettable for BT_SELECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
