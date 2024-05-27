///Register `SEARCH_POS` reader
pub type R = crate::R<SEARCH_POS_SPEC>;
///Register `SEARCH_POS` writer
pub type W = crate::W<SEARCH_POS_SPEC>;
///Field `SEARCH_POS` reader - Is used to configure the starting address when the acceleration option of search is used.
pub type SEARCH_POS_R = crate::FieldReader<u16>;
///Field `SEARCH_POS` writer - Is used to configure the starting address when the acceleration option of search is used.
pub type SEARCH_POS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Is used to configure the starting address when the acceleration option of search is used.
    #[inline(always)]
    pub fn search_pos(&self) -> SEARCH_POS_R {
        SEARCH_POS_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEARCH_POS")
            .field("search_pos", &self.search_pos())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Is used to configure the starting address when the acceleration option of search is used.
    #[inline(always)]
    #[must_use]
    pub fn search_pos(&mut self) -> SEARCH_POS_W<SEARCH_POS_SPEC> {
        SEARCH_POS_W::new(self, 0)
    }
}
/**The search position

You can [`read`](crate::generic::Reg::read) this register and get [`search_pos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_pos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SEARCH_POS_SPEC;
impl crate::RegisterSpec for SEARCH_POS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`search_pos::R`](R) reader structure
impl crate::Readable for SEARCH_POS_SPEC {}
///`write(|w| ..)` method takes [`search_pos::W`](W) writer structure
impl crate::Writable for SEARCH_POS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEARCH_POS to value 0
impl crate::Resettable for SEARCH_POS_SPEC {
    const RESET_VALUE: u32 = 0;
}
