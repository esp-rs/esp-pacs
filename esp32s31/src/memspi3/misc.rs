#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `CS_DIS(0-1)` reader - "]
pub type CS_DIS_R = crate::BitReader;
#[doc = "Field `CS_DIS(0-1)` writer - "]
pub type CS_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_IDLE_EDGE` reader - "]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - "]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - "]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - "]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field.</div>"]
    #[inline(always)]
    pub fn cs_dis(&self, n: u8) -> CS_DIS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CS_DIS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn cs_dis_iter(&self) -> impl Iterator<Item = CS_DIS_R> + '_ {
        (0..2).map(move |n| CS_DIS_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - CS0_DIS"]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CS1_DIS"]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("cs0_dis", &self.cs0_dis())
            .field("cs1_dis", &self.cs1_dis())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("cs_keep_active", &self.cs_keep_active())
            .finish()
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field.</div>"]
    #[inline(always)]
    pub fn cs_dis(&mut self, n: u8) -> CS_DIS_W<'_, MISC_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CS_DIS_W::new(self, n)
    }
    #[doc = "Bit 0 - CS0_DIS"]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS_DIS_W<'_, MISC_SPEC> {
        CS_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - CS1_DIS"]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS_DIS_W<'_, MISC_SPEC> {
        CS_DIS_W::new(self, 1)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<'_, MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<'_, MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "SPI1 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC to value 0x02"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
