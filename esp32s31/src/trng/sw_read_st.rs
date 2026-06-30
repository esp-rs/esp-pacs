#[doc = "Register `SW_READ_ST` reader"]
pub type R = crate::R<SW_READ_ST_SPEC>;
#[doc = "Register `SW_READ_ST` writer"]
pub type W = crate::W<SW_READ_ST_SPEC>;
#[doc = "Field `SW_DATA_VALID` reader - sw random data st"]
pub type SW_DATA_VALID_R = crate::BitReader;
#[doc = "Field `SW_RANDOM_REQ` writer - sw read random req"]
pub type SW_RANDOM_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sw random data st"]
    #[inline(always)]
    pub fn sw_data_valid(&self) -> SW_DATA_VALID_R {
        SW_DATA_VALID_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SW_READ_ST")
            .field("sw_data_valid", &self.sw_data_valid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - sw read random req"]
    #[inline(always)]
    pub fn sw_random_req(&mut self) -> SW_RANDOM_REQ_W<'_, SW_READ_ST_SPEC> {
        SW_RANDOM_REQ_W::new(self, 1)
    }
}
#[doc = "sw read st reg\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_read_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_read_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_READ_ST_SPEC;
impl crate::RegisterSpec for SW_READ_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_read_st::R`](R) reader structure"]
impl crate::Readable for SW_READ_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_read_st::W`](W) writer structure"]
impl crate::Writable for SW_READ_ST_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_READ_ST to value 0"]
impl crate::Resettable for SW_READ_ST_SPEC {}
