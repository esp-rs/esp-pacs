#[doc = "Register `TXBRK_CONF_SYNC` reader"]
pub type R = crate::R<TXBRK_CONF_SYNC_SPEC>;
#[doc = "Register `TXBRK_CONF_SYNC` writer"]
pub type W = crate::W<TXBRK_CONF_SYNC_SPEC>;
#[doc = "Field `TX_BRK_NUM` reader - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
pub type TX_BRK_NUM_R = crate::FieldReader;
#[doc = "Field `TX_BRK_NUM` writer - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
pub type TX_BRK_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
    #[inline(always)]
    pub fn tx_brk_num(&self) -> TX_BRK_NUM_R {
        TX_BRK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBRK_CONF_SYNC")
            .field("tx_brk_num", &format_args!("{}", self.tx_brk_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TXBRK_CONF_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn tx_brk_num(&mut self) -> TX_BRK_NUM_W<TXBRK_CONF_SYNC_SPEC, 0> {
        TX_BRK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tx Break character configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrk_conf_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbrk_conf_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBRK_CONF_SYNC_SPEC;
impl crate::RegisterSpec for TXBRK_CONF_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbrk_conf_sync::R`](R) reader structure"]
impl crate::Readable for TXBRK_CONF_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbrk_conf_sync::W`](W) writer structure"]
impl crate::Writable for TXBRK_CONF_SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXBRK_CONF_SYNC to value 0x0a"]
impl crate::Resettable for TXBRK_CONF_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
